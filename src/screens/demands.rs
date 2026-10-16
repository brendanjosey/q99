use std::fmt::Debug;

use iced::widget::{
    button, column, container, pick_list, row, scrollable, space, svg, text, text_input, tooltip,
};
use iced::{Center, Color, Element, Fill, Right, Task};

use polars::prelude::*;
use polars_excel_writer::PolarsExcelWriter;
use rfd::FileDialog;

use crate::components::demand_panel::{DemandPanel, Message as DemandPanelMessage};
use crate::props::core::{FixtureProps, TempProps, WaterUseType};
use crate::props::fixtures::FixtureType;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SaveFile {
    format_version: u32,
    state: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
struct StateV1 {
    project_name: String,
    project_description: String,
    project_date: String,
    project_client: String,
    project_revision: String,
    project_job_number: String,
    demand_panels: Vec<DemandPanel>,
    temp_props: TempProps,
    fixture_props: [FixtureProps; 8],
}

//TODO - if newer version is added..
// #[derive(Deserialize, Serialize)]
// struct StateV2 {
//     project_name: String,
//     project_description: String,
//     project_date: String,
//     project_client: String,
//     project_revision: String,
//     project_job_number: String,
//     demand_panels: Vec<DemandPanel>,
//     temp_props: TempProps,
//     fixture_props: [FixtureProps; 8],
// }

#[derive(Deserialize, Serialize)]
pub struct State {
    project_name: String,
    project_description: String,
    project_date: String,
    project_client: String,
    project_revision: String,
    project_job_number: String,
    demand_panels: Vec<DemandPanel>,
    temp_props: TempProps,
    fixture_props: [FixtureProps; 8],
    #[serde(skip)]
    current_file_name: std::path::PathBuf,
}

impl Default for State {
    fn default() -> Self {
        Self {
            project_name: String::new(),
            project_description: String::new(),
            project_date: String::new(),
            project_client: String::new(),
            project_revision: String::new(),
            project_job_number: String::new(),
            demand_panels: Vec::new(),
            temp_props: TempProps::default(),
            fixture_props: FixtureProps::all(),
            current_file_name: std::path::PathBuf::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ProjectNameChanged(String),
    ProjectDescriptionChanged(String),
    ProjectDateChanged(String),
    ProjectClientChanged(String),
    ProjectRevChanged(String),
    ProjectJobChanged(String),
    ColdWaterTempChanged(String),
    HeatedWaterTempChanged(String),
    FlowRateChanged(FixtureType, String),
    TemperatureChanged(FixtureType, String),
    WaterUseTypeChanged(FixtureType, WaterUseType),
    DemandPanel(usize, DemandPanelMessage),
    AddDemandPanel,
    ExportToExcel,
    FileSave,
    FileSaveAs,
    FileOpen,
}

impl State {
    // save function
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        //TODO - add in stateV1 funcionality....
        // let state: StateV1 = StateV1 {
        //
        // };

        let file = SaveFile {
            format_version: 1,
            state: serde_json::to_value(self)?,
        };

        let json = serde_json::to_vec_pretty(&file)?;
        std::fs::write(&self.current_file_name, json)?;
        Ok(())
    }

    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let Some(path) = FileDialog::new()
            .add_filter("Open File", &["json"])
            .pick_file()
        else {
            return Ok(()); // User cancelled
        };

        let bytes = std::fs::read(&path)?;
        let file: SaveFile = serde_json::from_slice(&bytes)?;

        if file.format_version != 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported save version: {}", file.format_version),
            )
            .into());
        }

        let mut state: Self = serde_json::from_value(file.state)?;
        state.current_file_name = path;
        *self = state;

        Ok(())
    }

    fn save_as(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let Some(path) = FileDialog::new()
            .add_filter("Save File", &["json"])
            .save_file()
        else {
            return Ok(()); // User cancelled
        };

        self.current_file_name = path;
        self.save()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ProjectNameChanged(name) => {
                self.project_name = name;
            }

            Message::ProjectDescriptionChanged(description) => {
                self.project_description = description;
            }

            Message::ProjectDateChanged(value) => {
                self.project_date = value;
            }

            Message::ProjectClientChanged(value) => {
                self.project_client = value;
            }

            Message::ProjectRevChanged(value) => {
                self.project_revision = value;
            }

            Message::ProjectJobChanged(value) => {
                self.project_job_number = value;
            }

            Message::ColdWaterTempChanged(value) => {
                match value.trim().parse::<f64>() {
                    Ok(t) => self.temp_props.cold_water_temp = Some(t),
                    _ => self.temp_props.cold_water_temp = Some(0.0),
                }
                // self.temp_props.cold_water_temp = value.trim().parse::<f64>().ok();
                self.temp_props.cold_water_temp_str = value;
                for panel in &mut self.demand_panels {
                    panel.refresh_results(&self.fixture_props, &self.temp_props);
                }
            }

            Message::HeatedWaterTempChanged(value) => {
                let cw_temp = match self.temp_props.cold_water_temp {
                    Some(t) => t,
                    None => 0.0,
                };

                match value.trim().parse::<f64>() {
                    Ok(t) => self.temp_props.heated_water_temp = Some(t.clamp(cw_temp, 100.0)),
                    _ => self.temp_props.heated_water_temp = Some(cw_temp + 0.1),
                }
                // self.temp_props.heated_water_temp = value.trim().parse::<f64>().ok();
                self.temp_props.heated_water_temp_str = value;
                for panel in &mut self.demand_panels {
                    panel.refresh_results(&self.fixture_props, &self.temp_props);
                }
            }

            Message::FlowRateChanged(fixture_type, value) => {
                //
                if let Some(props) = self
                    .fixture_props
                    .iter_mut()
                    .find(|f| f.fixture_type == fixture_type)
                {
                    props.set_flow_rate(value);

                    for panel in &mut self.demand_panels {
                        panel.refresh_results(&self.fixture_props, &self.temp_props);
                    }
                }
            }

            Message::TemperatureChanged(fixture_type, value) => {
                //
                if let Some(props) = self
                    .fixture_props
                    .iter_mut()
                    .find(|f| f.fixture_type == fixture_type)
                {
                    props.set_temperature(
                        value,
                        self.temp_props.cold_water_temp,
                        self.temp_props.heated_water_temp,
                    );

                    for panel in &mut self.demand_panels {
                        panel.refresh_results(&self.fixture_props, &self.temp_props);
                    }
                }
            }

            Message::WaterUseTypeChanged(fixture_type, water_use_type) => {
                if let Some(props) = self
                    .fixture_props
                    .iter_mut()
                    .find(|f| f.fixture_type == fixture_type)
                {
                    props.water_use_type = Some(water_use_type.clone());

                    let cw_temp = self.temp_props.cold_water_temp;
                    let hw_temp = self.temp_props.heated_water_temp;

                    match water_use_type {
                        WaterUseType::ColdAndHeatedWater => {
                            props.set_temperature(cw_temp.unwrap().to_string(), cw_temp, hw_temp)
                        }
                        WaterUseType::HeatedWater => {
                            props.set_temperature(hw_temp.unwrap().to_string(), cw_temp, hw_temp)
                        }
                        WaterUseType::ColdWater => {
                            props.set_temperature(cw_temp.unwrap().to_string(), cw_temp, hw_temp)
                        }
                    };
                }

                for panel in &mut self.demand_panels {
                    panel.refresh_results(&self.fixture_props, &self.temp_props);
                }
            }

            Message::DemandPanel(index, DemandPanelMessage::Delete) => {
                // println!("delete called");
                if index < self.demand_panels.len() {
                    self.demand_panels.remove(index);
                }
            }

            Message::DemandPanel(index, message) => {
                if let Some(panel) = self.demand_panels.get_mut(index) {
                    let _ = panel.update(message);
                    panel.refresh_results(&self.fixture_props, &self.temp_props);
                }
            }

            Message::AddDemandPanel => {
                let dp = DemandPanel::new();
                self.demand_panels.push(dp);
            }

            Message::ExportToExcel => {
                if self.demand_panels.is_empty() != true {
                    let mut name_col = Vec::new();
                    let mut service_col = Vec::new();
                    let mut n_fixtures_col = Vec::new();
                    let mut n_apts_col = Vec::new();
                    let mut n_occs_col = Vec::new();
                    let mut flow_rate_col = Vec::new();

                    for panel in &self.demand_panels {
                        // loop services
                        for service in ["Cold Water", "Heated Water"] {
                            name_col.push(panel.panel_name.clone());
                            service_col.push(service.to_string());

                            // number of fixtures
                            let n_fixtures = match &service {
                                &"Cold Water" => panel.n_fixtures as i32,
                                &"Heated Water" => 0,
                                _ => 0,
                            };

                            n_fixtures_col.push(n_fixtures as i32);

                            n_apts_col.push(match panel.n_apts.trim().parse::<usize>() {
                                Ok(val) => val as i32,
                                _ => 0,
                            });

                            n_occs_col.push(match panel.n_occupants.trim().parse::<usize>() {
                                Ok(val) => val as i32,
                                _ => 0,
                            });

                            let flow_rate = match &service {
                                &"Cold Water" => match panel.dp_results.q_tw {
                                    Some(q) => Some(q),
                                    None => None,
                                },
                                &"Heated Water" => match panel.dp_results.q_hw {
                                    Some(q) => Some(q),
                                    None => None,
                                },
                                _ => None,
                            };

                            flow_rate_col.push(flow_rate);
                        }
                    }

                    // create dataframe
                    let result: PolarsResult<()> = (|| {
                        let df = DataFrame::new_infer_height(vec![
                            Column::new("Desc".into(), name_col),
                            Column::new("Service".into(), service_col),
                            Column::new("N Fixtures".into(), n_fixtures_col),
                            Column::new("N Apartments".into(), n_apts_col),
                            Column::new("N Occupants".into(), n_occs_col),
                            Column::new("Flow Rate".into(), flow_rate_col),
                        ])?;

                        if let Some(path) = FileDialog::new()
                            .add_filter("Excel workbook", &["xlsx"])
                            .set_file_name("demand.xlsx")
                            .save_file()
                        {
                            let mut writer = PolarsExcelWriter::new();
                            writer.write_dataframe(&df)?;
                            writer.save(path)?;
                        }

                        Ok(())
                    })();

                    if let Err(err) = result {
                        eprintln!("Export failed: {err}");
                    };
                }
            }

            Message::FileSave => {
                if self.current_file_name.as_os_str().is_empty() {
                    // launch save as
                    let _ = self.save_as();
                } else {
                    // save
                    let _ = self.save();
                }
            }

            Message::FileSaveAs => {
                let _ = self.save_as();
            }

            Message::FileOpen => {
                let _ = self.load();
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        // project details
        let pd_job = row![
            text("Job Number:").size(12).width(150).align_x(Right),
            text_input("Job number...", &self.project_job_number)
                .width(150)
                .on_input(Message::ProjectJobChanged),
            space().width(Fill),
            text("Revision:").size(12),
            text_input("Revision...", &self.project_revision)
                .width(100)
                .on_input(Message::ProjectRevChanged),
            text("Date:").size(12),
            text_input("Date...", &self.project_date)
                .width(100)
                .on_input(Message::ProjectDateChanged)
        ]
        .align_y(Center)
        .width(Fill)
        .spacing(5);

        let pd_client = row![
            text("Project Client:").size(12).width(150).align_x(Right),
            text_input("Enter the project client...", &self.project_client,)
                .on_input(Message::ProjectClientChanged),
        ]
        .align_y(Center)
        .width(Fill)
        .spacing(5);

        let pd_name = row![
            text("Project name:").size(12).width(150).align_x(Right),
            text_input("Enter the project name...", &self.project_name,)
                .on_input(Message::ProjectNameChanged),
        ]
        .align_y(Center)
        .width(Fill)
        .spacing(5);

        let pd_desc = row![
            text("Description:").size(12).width(150).align_x(Right),
            text_input(
                "Enter the project description...",
                &self.project_description,
            )
            .on_input(Message::ProjectDescriptionChanged),
        ]
        .align_y(Center)
        .width(Fill)
        .spacing(5);

        // water temp inputs
        let water_temp_inputs_row = row![
            text("Cold Water Temperature: ").size(12),
            text_input("Cold water temp", &self.temp_props.cold_water_temp_str)
                .on_input(Message::ColdWaterTempChanged)
                .align_x(Center)
                .width(70),
            text("°C").size(12),
            space().width(20),
            text("Heated Water Temperature: ").size(10),
            text_input("Heated water temp", &self.temp_props.heated_water_temp_str)
                .on_input(Message::HeatedWaterTempChanged)
                .align_x(Center)
                .width(70),
            text("°C").size(12),
        ]
        .spacing(5)
        .align_y(Center)
        .width(Fill);

        let mut fixture_settings_input_row = row![
            column![
                text("Fixture:").size(12).align_x(Right).width(150),
                text("Flow Rate (L/s):").size(12).align_x(Right),
                text("Target Temperature (°C):").size(12).align_x(Right),
                text("Heated Water Flow Rate:").size(12).align_x(Right),
                text("Cold Water Flow Rate:").size(12).align_x(Right),
            ]
            .spacing(15)
            .align_x(Right)
        ]
        .spacing(2)
        .width(Fill);

        for props in &self.fixture_props {
            let name = match &props.fixture_type {
                FixtureType::Shower => "Shower",
                FixtureType::Bath => "Bath",
                FixtureType::Toilet => "Toilet",
                FixtureType::TapBasin => "Tap-Basin",
                FixtureType::TapKitchen => "Tap-Kitchen",
                FixtureType::TapLaundry => "Tap-Laundry",
                FixtureType::WashingMachine => "Washing Machine",
                FixtureType::Dishwasher => "Dishwasher",
                // _ => "Fixture",
            };

            let temp_input: Element<'_, Message> = match props.fixture_type {
                FixtureType::WashingMachine => pick_list(
                    [
                        WaterUseType::ColdAndHeatedWater,
                        WaterUseType::HeatedWater,
                        WaterUseType::ColdWater,
                    ],
                    props.water_use_type.clone(),
                    |value| Message::WaterUseTypeChanged(props.fixture_type.clone(), value),
                )
                .width(80)
                //.text_size(10)
                .into(),
                FixtureType::Dishwasher => pick_list(
                    [
                        WaterUseType::ColdAndHeatedWater,
                        WaterUseType::HeatedWater,
                        WaterUseType::ColdWater,
                    ],
                    props.water_use_type.clone(),
                    |value| Message::WaterUseTypeChanged(props.fixture_type.clone(), value),
                )
                .width(80)
                //.text_size(10)
                .into(),
                _ => match props.water_use_type {
                    Some(WaterUseType::ColdAndHeatedWater) => {
                        text_input("", &props.temperature_str)
                            .width(70)
                            .align_x(Center)
                            .on_input(|value| {
                                Message::TemperatureChanged(props.fixture_type.clone(), value)
                            })
                            .into()
                    }
                    Some(WaterUseType::HeatedWater) => text_input(
                        "",
                        &format!("{:.1}", self.temp_props.heated_water_temp.unwrap()),
                    )
                    .width(70)
                    .align_x(Center)
                    .into(),
                    Some(WaterUseType::ColdWater) => text_input(
                        "",
                        &format!("{:.1}", self.temp_props.cold_water_temp.unwrap()),
                    )
                    .width(70)
                    .align_x(Center)
                    .into(),
                    _ => text_input("", "").into(),
                },
            };

            let cold_water_flow = format!("{:.2}L/s", props.flow_rate_cw(&self.temp_props));
            let heated_water_flow = format!("{:.2}L/s", props.flow_rate_hw(&self.temp_props));

            let inputs = column![
                text(name).size(10).align_x(Center),
                space().height(2),
                text_input("", &props.flow_rate_str)
                    .on_input(|value| Message::FlowRateChanged(props.fixture_type.clone(), value))
                    .width(70)
                    .align_x(Center),
                space().height(1),
                temp_input,
                space().height(3),
                text(heated_water_flow),
                space().height(5),
                text(cold_water_flow),
            ]
            .spacing(2)
            .width(100)
            .align_x(Center);

            fixture_settings_input_row = fixture_settings_input_row.push(inputs)
        }

        let mut demand_panel_col = column![].spacing(5).width(Fill);

        for (index, panel) in self.demand_panels.iter().enumerate() {
            let view = panel
                .view(&self.fixture_props)
                .map(move |message| Message::DemandPanel(index, message));
            demand_panel_col = demand_panel_col.push(view);
        }

        let current_file = if !self.current_file_name.as_os_str().is_empty() {
            let file_name = self
                .current_file_name
                .clone()
                .into_os_string()
                .into_string()
                .unwrap();
            text(format!("Current file: {}", &file_name)).size(10)
        } else {
            text("")
        };

        let save_btn = tooltip(
            button(
                svg("src/assets/icons/save.svg")
                    .width(15)
                    .style(|_, _| svg::Style {
                        color: Some(Color::from_rgb(0.188, 0.196, 0.286)),
                    }),
            )
            .on_press(Message::FileSave),
            "Save file",
            tooltip::Position::Top,
        );

        let saveas_btn = tooltip(
            button(
                svg("src/assets/icons/save_as.svg")
                    .width(15)
                    .style(|_, _| svg::Style {
                        color: Some(Color::from_rgb(0.188, 0.196, 0.286)),
                    }),
            )
            .on_press(Message::FileSaveAs),
            "Save file as",
            tooltip::Position::Top,
        );

        let open_btn = tooltip(
            button(
                svg("src/assets/icons/open.svg")
                    .width(15)
                    .style(|_, _| svg::Style {
                        color: Some(Color::from_rgb(0.188, 0.196, 0.286)),
                    }),
            )
            .on_press(Message::FileOpen),
            "Open file",
            tooltip::Position::Top,
        );

        let export_btn = tooltip(
            button(
                svg("src/assets/icons/excel.svg")
                    .width(15)
                    .style(|_, _| svg::Style {
                        color: Some(Color::from_rgb(0.188, 0.196, 0.286)),
                    }),
            )
            .on_press(Message::ExportToExcel),
            "Export demand panels to Excel",
            tooltip::Position::Top,
        );

        let header_row = row![
            text("Project Details").size(24),
            space().width(Fill),
            current_file,
            open_btn,
            save_btn,
            saveas_btn,
            export_btn,
        ]
        .spacing(5)
        .align_y(Center)
        .width(Fill);

        let demand_panel_header = row![
            text("Demand Panels").size(24),
            space().width(Fill),
            tooltip(
                button("+").on_press(Message::AddDemandPanel),
                "Add demand panel",
                tooltip::Position::Top,
            )
        ]
        .width(1400)
        .spacing(10);

        let content = column![
            header_row,
            pd_job,
            pd_client,
            pd_name,
            pd_desc,
            text("Water Temperature Settings").size(24),
            // cold water temp
            water_temp_inputs_row,
            text("Fixture Settings").size(24),
            fixture_settings_input_row,
            demand_panel_header,
            demand_panel_col,
        ]
        .width(1400)
        .padding(15)
        .spacing(12);
        //.into()

        let scroll_dir = scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::default().spacing(10),
            horizontal: scrollable::Scrollbar::default().spacing(10),
        };

        container(scrollable(content).direction(scroll_dir))
            //.width(2000)
            .into()
    }
}
