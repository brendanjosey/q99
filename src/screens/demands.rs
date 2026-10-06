use std::fmt::Debug;

use iced::widget::{
    button, column, container, pick_list, row, scrollable, space, text, text_input,
};
use iced::{Center, Element, Fill, Right, Task};

use crate::components::demand_panel::{DemandPanel, Message as DemandPanelMessage};
use crate::props::core::{FixtureProps, TempProps, WaterUseType};
use crate::props::fixtures::FixtureType;

//#[derive(Default)]
pub struct State {
    project_name: String,
    project_description: String,
    demand_panels: Vec<DemandPanel>,
    temp_props: TempProps,
    fixture_props: [FixtureProps; 8],
}

impl Default for State {
    fn default() -> Self {
        Self {
            project_name: String::new(),
            project_description: String::new(),
            demand_panels: Vec::new(),
            temp_props: TempProps::default(),
            fixture_props: FixtureProps::all(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ProjectNameChanged(String),
    ProjectDescriptionChanged(String),
    ColdWaterTempChanged(String),
    HeatedWaterTempChanged(String),
    FlowRateChanged(FixtureType, String),
    TemperatureChanged(FixtureType, String),
    WaterUseTypeChanged(FixtureType, WaterUseType),
    DemandPanel(usize, DemandPanelMessage),
    AddDemandPanel, //EndUsePropsChanged,
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ProjectNameChanged(name) => {
                self.project_name = name;
            }

            Message::ProjectDescriptionChanged(description) => {
                self.project_description = description;
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
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        //
        let water_temp_inputs_row = row![
            text("Cold Water Temperature: "),
            text_input("Cold water temp", &self.temp_props.cold_water_temp_str)
                .on_input(Message::ColdWaterTempChanged)
                .align_x(Center)
                .width(70),
            text("°C"),
            space().width(20),
            text("Heated Water Temperature: "),
            text_input("Heated water temp", &self.temp_props.heated_water_temp_str)
                .on_input(Message::HeatedWaterTempChanged)
                .align_x(Center)
                .width(70),
            text("°C"),
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

            // let temp_input = match props.water_use_type {
            //     Some(WaterUseType::ColdAndHeatedWater) => text_input("", &props.temperature_str)
            //         .width(70)
            //         .align_x(Center)
            //         .on_input(|value| {
            //             Message::TemperatureChanged(props.fixture_type.clone(), value)
            //         }),
            //     Some(WaterUseType::HeatedWater) => text_input(
            //         "",
            //         &format!("{:.1}", self.temp_props.heated_water_temp.unwrap()),
            //     )
            //     .width(70)
            //     .align_x(Center),
            //     Some(WaterUseType::ColdWater) => text_input(
            //         "",
            //         &format!("{:.1}", self.temp_props.cold_water_temp.unwrap()),
            //     )
            //     .width(70)
            //     .align_x(Center),
            //     _ => text_input("", ""),
            // };

            let cold_water_flow = format!("{:.2}L/s", props.flow_rate_cw(&self.temp_props));
            let heated_water_flow = format!("{:.2}L/s", props.flow_rate_hw(&self.temp_props));

            // let cold_water_flow = match props.water_use_type {
            //     Some(WaterUseType::HeatedWater) => String::from("N/A"),
            //     _ => format!(
            //         "{:.2}L/s",
            //         calc_diversified_flow_rate(
            //             props.flow_rate,
            //             props.temperature,
            //             self.temp_props.cold_water_temp,
            //             self.temp_props.heated_water_temp,
            //             true
            //         )
            //     ),
            // };

            // let heated_water_flow = match props.water_use_type {
            //     Some(WaterUseType::ColdWater) => String::from("N/A"),
            //     _ => format!(
            //         "{:.2}L/s",
            //         calc_diversified_flow_rate(
            //             props.flow_rate,
            //             props.temperature,
            //             self.temp_props.cold_water_temp,
            //             self.temp_props.heated_water_temp,
            //             false,
            //         )
            //     ),
            // };

            let inputs = column![
                text(name).size(10).align_x(Center),
                space().height(2),
                text_input("", &props.flow_rate_str)
                    .on_input(|value| Message::FlowRateChanged(props.fixture_type.clone(), value))
                    .width(70)
                    .align_x(Center),
                space().height(1),
                temp_input,
                // text_input("", &props.temperature_str)
                //     .on_input(|value| Message::TemperatureChanged(
                //         props.fixture_type.clone(),
                //         value
                //     ))
                //     .width(70)
                //     .align_x(Center),
                space().height(3),
                text(heated_water_flow),
                space().height(5),
                text(cold_water_flow),
            ]
            .spacing(2)
            .width(100)
            .align_x(Center);

            // if props.fixture_type == FixtureType::WashingMachine
            //     || props.fixture_type == FixtureType::Dishwasher
            // {
            //     inputs = inputs.push(text("Water Use").size(10).align_x(Center).width(Fill));
            //     inputs = inputs.push(
            //         pick_list(
            //             [
            //                 WaterUseType::ColdAndHeatedWater,
            //                 WaterUseType::HeatedWater,
            //                 WaterUseType::ColdWater,
            //             ],
            //             props.water_use_type.clone(),
            //             |value| Message::WaterUseTypeChanged(props.fixture_type.clone(), value),
            //         )
            //         .width(100)
            //         .text_size(10),
            //     );
            // }

            fixture_settings_input_row = fixture_settings_input_row.push(inputs)
        }

        let mut demand_panel_col = column![].spacing(5).width(Fill);

        for (index, panel) in self.demand_panels.iter().enumerate() {
            let view = panel
                .view(&self.fixture_props)
                .map(move |message| Message::DemandPanel(index, message));
            demand_panel_col = demand_panel_col.push(view);
        }

        let content = column![
            text("Project Details").size(24),
            text("Project name"),
            text_input("Enter the project name", &self.project_name,)
                .on_input(Message::ProjectNameChanged),
            text("Description"),
            text_input("Enter a description", &self.project_description,)
                .on_input(Message::ProjectDescriptionChanged),
            text("Water Temperature Settings").size(24),
            // cold water temp
            water_temp_inputs_row,
            text("Fixture Settings").size(24),
            fixture_settings_input_row,
            button("Add Demand Panel").on_press(Message::AddDemandPanel),
            text("Demand Panels").size(24),
            demand_panel_col,
        ]
        .spacing(12);
        //.into()

        container(
            scrollable(content).direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::default(),
            )),
        )
        .width(Fill)
        .into()
    }
}
