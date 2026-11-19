use iced::widget::{button, column, container, row, rule, space, svg, text, text_input};
use iced::{Border, Center, Color, Element, Fill, Right, Shadow, Task, Theme, Vector};
//use short_id::short_id_ordered;

use crate::components::demand_panel::Message::Delete;
use crate::components::pipe_size_panel::{Message as PipeSizePanelMessage, PipeSizePanel};
use crate::props::core::{DemandPanelResults, FixtureProps, FlowType, TempProps, WaterUseType};
use crate::props::fixtures::{Fixture, FixtureType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DemandPanel {
    //panel_id: String,
    pub panel_name: String,
    is_expanded: bool,
    pub n_apts: String,
    pub n_occupants: String,
    pub n_fixtures: usize,
    pub n_fixtures_hw: usize,
    // TODO
    // pub n_fixturues_cw_div: usize,
    pub fixtures: [Fixture; 8],
    q_total: String,
    q_total_hw: String,
    q_tw: String,
    // q_cw: String, for later extensions as designeers become more proficient with new method.
    q_hw: String,
    p0: String,
    p0_hw: String,
    pub dp_results: DemandPanelResults,
    pipe_size_panel: PipeSizePanel,
    pipe_size_panel_hw: PipeSizePanel,
    //pipe_size_panel_cw: PipeSizePanel,
}

#[derive(Debug, Clone)]
pub enum Message {
    PanelNameChanged(String),
    ExpandContract,
    AptsChanged(String),
    OccChanged(String),
    FixtureQtyChanged(FixtureType, String),
    PipeSizePanel(PipeSizePanelMessage),
    PipeSizePanelHw(PipeSizePanelMessage),
    // PipeSizePanelCw(PipeSizePanelMessage),
    Delete,
    MovePanelUp,
    MovePanelDown,
}

impl DemandPanel {
    // pub fn update_props(&mut self, end_use_props: [&EndUseProps; 8]) {
    //     // updates props
    //     self.end_use_props = Some(end_use_props);
    // }
    pub fn new() -> Self {
        Self {
            //panel_id: short_id_ordered(),
            panel_name: String::new(),
            is_expanded: true,
            n_apts: String::new(),
            n_occupants: String::new(),
            n_fixtures: 0,
            n_fixtures_hw: 0,
            fixtures: [
                Fixture::shower(),
                Fixture::bath(),
                Fixture::toilet(),
                Fixture::tap_basin(),
                Fixture::tap_kitchen(),
                Fixture::tap_laundry(),
                Fixture::washing_machine(),
                Fixture::dishwasher(),
            ],
            q_total: String::new(),
            q_total_hw: String::new(),
            q_tw: String::new(),
            //q_cw: String::new(),
            q_hw: String::new(),
            p0: String::new(),
            p0_hw: String::new(),
            dp_results: DemandPanelResults::default(),
            pipe_size_panel: PipeSizePanel::default(),
            pipe_size_panel_hw: PipeSizePanel::default(),
            // pipe_size_panel_cw: PipeSizePanel::default(),
        }
    }

    pub fn calc_total_flow_rate<'a>(&mut self, fixture_props: &'a [FixtureProps; 8]) {
        let mut total_flow = 0.0;
        let mut total_flow_hw = 0.0;

        for fixture in &self.fixtures {
            let ft = fixture.fixture_type();

            if let Some(props) = fixture_props.iter().find(|f| &f.fixture_type == ft) {
                // total flow
                let n = fixture.qty() as f64;
                let q = props.flow_rate.unwrap_or(0.0);
                total_flow += n * q;
                // total hw flow
                if props.water_use_type == Some(WaterUseType::ColdAndHeatedWater)
                    || props.water_use_type == Some(WaterUseType::HeatedWater)
                {
                    total_flow_hw += n * q;
                }
            }
        }

        self.q_total = format!("{:.2}L/s", total_flow);
        self.q_total_hw = format!("{:.2}L/s", total_flow_hw);
        self.dp_results.q_total = Some(total_flow);
        self.dp_results.q_total_hw = Some(total_flow_hw);
    }

    pub fn set_water_use_types<'a>(&mut self, fixture_props: &'a [FixtureProps; 8]) {
        for fixture in &mut self.fixtures {
            let ft = fixture.fixture_type();
            if let Some(props) = fixture_props.iter().find(|f| &f.fixture_type == ft) {
                let wut_fixture = fixture.water_use_type().clone();
                let wut_props = props.water_use_type.clone().unwrap();
                if wut_fixture != wut_props {
                    // update water use type
                    fixture.set_water_use_type(wut_props);
                }
            }
        }
    }

    pub fn calc_fixture_qty(&mut self) {
        // total fixture qty
        let fixture_qty = self.fixtures.iter().map(|f| f.qty()).sum::<usize>();
        self.n_fixtures = fixture_qty;

        // heated water fixture qty
        let fixture_qty_hw: usize = self
            .fixtures
            .iter()
            .map(|f| {
                if f.water_use_type() == &WaterUseType::ColdAndHeatedWater
                    || f.water_use_type() == &WaterUseType::HeatedWater
                {
                    f.qty()
                } else {
                    0
                }
            })
            .sum();

        self.n_fixtures_hw = fixture_qty_hw;
    }
    // fn cacl_p0(&mut self) {
    //     let mut p0_init = false;
    //     let mut p0 = 1.0;

    //     for fixture in &self.fixtures {
    //         // only include if fixtury qty is greater than zero
    //         if fixture.qty() > 0 {
    //             let n = fixture.qty() as f64;
    //             let p = fixture.p_value();

    //             if p0_init {
    //                 p0 *= (1.0 - p).powf(n);
    //             } else {
    //                 // initialise P0
    //                 p0 = (1.0 - p).powf(n);
    //                 p0_init = true;
    //             }
    //         }
    //     }

    //     self.p0 = format!("{:.2}%", p0 * 100.0);
    //     self.dp_results.p0 = Some(p0);
    // }

    fn calc_q99<'a>(
        &mut self,
        fixture_props: &'a [FixtureProps; 8],
        temp_props: &'a TempProps,
        flow_type: FlowType,
    ) -> Result<Option<f64>, &'static str> {
        // closure to get flow rate dependant of flow type

        // only one fixture
        if self.n_fixtures == 1 {
            if let Some(fixture) = &self.fixtures.iter().find(|f| f.qty() == 1) {
                if let Some(props) = fixture_props
                    .iter()
                    .find(|f| &f.fixture_type == fixture.fixture_type())
                {
                    return Ok(props.flow_rate);
                }
            }
        };

        // more than three fixtures
        let mut sum_npq = 0.0;
        let mut sum_var = 0.0;
        let mut p0 = 1.0;

        for fixture in &self.fixtures {
            if fixture.qty() != 0 {
                if let Some(props) = fixture_props
                    .iter()
                    .find(|f| &f.fixture_type == fixture.fixture_type())
                {
                    //let q = flow_rate(props);
                    //let q = props.flow_rate.unwrap_or(0.0);
                    let q = match flow_type {
                        FlowType::TotalWaterDemand => props.flow_rate_tw(),
                        FlowType::HeatedWaterDemand => props.flow_rate_hw(temp_props),
                        FlowType::ColdWaterDemand => props.flow_rate_cw(temp_props),
                    };

                    //println!("q: {:.2}", q);

                    let n = fixture.qty() as f64;
                    let p = fixture.p_value();

                    if q > 0.0 {
                        sum_npq += n * p * q;
                        sum_var += n * p * (1.0 - p) * q.powf(2.0);

                        p0 *= (1.0 - p).powf(n);
                    };
                }
            };
        }

        match flow_type {
            FlowType::TotalWaterDemand => {
                self.p0 = format!("{:.2}%", p0 * 100.0);
                self.dp_results.p0 = Some(p0);
            }
            FlowType::HeatedWaterDemand => {
                self.p0_hw = format!("{:.2}%", p0 * 100.0);
                self.dp_results.p0_hw = Some(p0);
            }
            FlowType::ColdWaterDemand => {}
        }
        let f1 = 1.0 / (1.0 - p0);
        let f2 = (1.0 + p0) * 2.326;
        let f3 = (1.0 - p0) * sum_var - p0 * sum_npq.powf(2.0);

        let q99 = f1 * (sum_npq + f2 * f3.sqrt());

        Ok(Some(q99))
    }

    pub fn refresh_results<'a>(
        &mut self,
        fixture_props: &'a [FixtureProps; 8],
        temp_props: &'a TempProps,
    ) {
        // update water use types
        self.set_water_use_types(fixture_props);

        // update fixture qtys
        self.calc_fixture_qty();

        // update total flow rates and Q99 values
        if !self.n_apts.is_empty() && !self.n_occupants.is_empty() {
            // functions to refresh results
            self.calc_total_flow_rate(fixture_props);
            // self.cacl_p0();
            // println!("total demands");
            // update total water Q99
            match self.calc_q99(fixture_props, temp_props, FlowType::TotalWaterDemand) {
                Ok(Some(q)) => {
                    self.q_tw = format!("{:.2}L/s", q);
                    self.dp_results.q_tw = Some(q);
                }
                _ => {
                    self.q_tw = "N/A".to_string();
                    self.dp_results.q_tw = None;
                }
            }
            //println!("heated water demands");
            // update heated water Q99
            match self.calc_q99(fixture_props, temp_props, FlowType::HeatedWaterDemand) {
                Ok(Some(q)) => {
                    self.q_hw = format!("{:.2}L/s", q);
                    self.dp_results.q_hw = Some(q);
                }
                _ => {
                    self.q_hw = "N/A".to_string();
                    self.dp_results.q_hw = None;
                }
            }
            //todo - add back in when industry advances...
            // match self.calc_q99(fixture_props, temp_props, FlowType::ColdWaterDemand) {
            //     Ok(Some(q)) => {
            //         self.q_cw = format!("{:.2}L/s", q);
            //         self.dp_results.q_cw = Some(q);
            //     }
            //     _ => {
            //         self.q_cw = "N/A".to_string();
            //         self.dp_results.q_cw = None;
            //     }
            // }
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PanelNameChanged(name) => {
                self.panel_name = name;
            }

            Message::ExpandContract => {
                self.is_expanded = !self.is_expanded;
            }

            Message::AptsChanged(apts) => {
                self.n_apts = apts.clone();
                let apts = self.n_apts.trim().parse().unwrap_or(1);
                let occs = self.n_occupants.trim().parse().unwrap_or(1);

                // update p_values
                for i in &mut self.fixtures {
                    i.set_p_value(apts, occs);
                }
            }

            Message::OccChanged(occs) => {
                self.n_occupants = occs.clone();
                let apts = self.n_apts.trim().parse().unwrap_or(1);
                let occs = self.n_occupants.trim().parse().unwrap_or(1);

                // update p_values
                for i in &mut self.fixtures {
                    i.set_p_value(apts, occs);
                }
            }

            Message::FixtureQtyChanged(fixture_type, qty) => {
                if let Some(fixture) = self
                    .fixtures
                    .iter_mut()
                    .find(|fixture| fixture.fixture_type() == &fixture_type)
                {
                    // set new qty to fixture
                    let qty = qty.trim().parse::<usize>();
                    fixture.set_qty(qty.clone());
                    // update total fixture qty
                    self.calc_fixture_qty();
                }
            }

            Message::PipeSizePanel(message) => {
                let _ = self.pipe_size_panel.update(message);
            }

            Message::PipeSizePanelHw(message) => {
                let _ = self.pipe_size_panel_hw.update(message);
            }

            // Message::PipeSizePanelCw(message) => {
            //     let _ = self.pipe_sizw_panel_cw.update(message);
            // }
            Message::Delete => {}

            Message::MovePanelUp => {}
            Message::MovePanelDown => {}
        }

        Task::none()
    }

    pub fn view<'a>(&self, fixture_props: &'a [FixtureProps; 8]) -> Element<'_, Message> {
        let panel_name: Element<'_, Message> = if self.is_expanded {
            text_input("Enter panel name...", &self.panel_name)
                //.width(300)
                .on_input(Message::PanelNameChanged)
                .into()
        } else {
            if !self.panel_name.is_empty() {
                text(&self.panel_name).into()
            } else {
                text("Demand Panel").into()
            }
        };

        let toggle_btn_content = if self.is_expanded {
            //text(">").align_x(Center)
            text("hide").align_x(Center).size(10)
        } else {
            //text("↓").align_x(Center)
            text("show").align_x(Center).size(10)
        };

        let toggle_btn = button(toggle_btn_content)
            .on_press(Message::ExpandContract)
            .width(50);

        let apts_row = row![
            text("Apartments:").size(12).width(100).align_x(Right),
            text_input("Apts", &self.n_apts)
                .width(70)
                .align_x(Center)
                .on_input(Message::AptsChanged)
        ]
        .spacing(10)
        .align_y(Center)
        .width(Fill);

        let occupants_row = row![
            text("Occupants:").size(12).width(100).align_x(Right),
            text_input("Occs", &self.n_occupants)
                .width(70)
                .align_x(Center)
                .on_input(Message::OccChanged)
        ]
        .spacing(10)
        .align_y(Center)
        .width(Fill);

        let ave_occ = match (
            self.n_apts.trim().parse::<f32>(),
            self.n_occupants.trim().parse::<f32>(),
        ) {
            (Ok(apts), Ok(occs)) => text(format!("Average Occupants: {:.1}", occs / apts)).size(12),
            _ => text("Average Occupants: N/A").size(12),
        };

        let building_inputs = column![
            text("Building Inputs").size(16).width(Fill).align_x(Center),
            apts_row,
            occupants_row,
            ave_occ,
        ]
        .spacing(5)
        .width(200)
        .align_x(Center);

        let mut fixtures_row = row![].spacing(2).width(Fill);

        for fixture in &self.fixtures {
            let fixture_type = fixture.fixture_type();

            let name = match &fixture_type {
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

            let mut flow_rate: f64 = 0.0;
            if let Some(props) = fixture_props
                .iter()
                .find(|f| &f.fixture_type == fixture_type)
            {
                flow_rate = props.flow_rate.unwrap_or(0.0);
            };

            let ti = match fixture.qty() {
                0 => "",
                _ => &format!("{}", fixture.qty()),
            };

            let input = column![
                text(name).size(10).width(Fill).align_x(Center),
                text_input("Qty", ti)
                    .width(70)
                    .align_x(Center)
                    .on_input(move |value| {
                        Message::FixtureQtyChanged(fixture_type.clone(), value)
                    }),
                text(format!("p: {:.3}%", fixture.p_value() * 100.0))
                    .size(10)
                    .width(Fill)
                    .align_x(Center),
                text(format!("flow rate: {:.2}L/s", flow_rate))
                    .size(10)
                    .width(Fill)
                    .align_x(Center),
            ]
            .spacing(5)
            .align_x(Center)
            .width(100);

            fixtures_row = fixtures_row.push(input);
        }

        let fixture_inputs = column![
            text("Fixtures & Appliances")
                .size(16)
                .width(Fill)
                .align_x(Center),
            fixtures_row,
        ]
        .spacing(5)
        .align_x(Center)
        .width(800);

        let n_fixture = |ws, n| {
            if n == 0 {
                text(format!("{}:", ws)).size(12).width(75) //.align_x(Right)
            } else {
                text(format!("{}: {}", ws, n)).size(12).width(75)
                // .align_x(Right)
            }
        };
        // result rows
        let res_n_fixtures_row = row![
            text("Fixtures:").width(Fill).align_x(Right).size(12),
            n_fixture("CW", self.n_fixtures),
            n_fixture("HW", self.n_fixtures_hw),
        ]
        .align_y(Center)
        .spacing(5);

        let res_total_flow_row = row![
            text("Total Flow Rate:").width(Fill).align_x(Right).size(12),
            text(format!("CW: {}", self.q_total)).size(12).width(75),
            // .align_x(Right),
            text(format!("HW: {}", self.q_total_hw)).size(12).width(75),
            // .align_x(Right),
        ]
        .align_y(Center)
        .spacing(5);

        let res_p0_row = row![
            text("Stagnation Probability:")
                .width(Fill)
                .align_x(Right)
                .size(12),
            text(format!("CW: {}", self.p0)).size(12).width(75),
            // .align_x(),
            text(format!("HW: {}", self.p0_hw)).size(12).width(75),
            // .align_x(Right),
        ]
        .align_y(Center)
        .spacing(5);

        let res_q99_row = row![
            text("Peak Demand:").width(Fill).align_x(Right).size(12),
            text(format!("CW: {}", self.q_tw)).size(12).width(75),
            // .align_x(),
            text(format!("HW: {}", self.q_hw)).size(12).width(75),
            // .align_x(Right),
        ]
        .align_y(Center)
        .spacing(5);

        let results_col = column![
            text("Results").size(16).width(Fill).align_x(Center),
            // fixtures
            res_n_fixtures_row,
            // total flow rate
            res_total_flow_row,
            // peak hour stagnation probability
            res_p0_row,
            // 99th percentile demands
            res_q99_row,
        ]
        .spacing(5)
        .align_x(Center)
        .width(375);

        let inputs_row = row![
            building_inputs,
            rule::vertical(1),
            fixture_inputs,
            rule::vertical(1),
            results_col,
            space().width(Fill)
        ]
        .spacing(10)
        .align_y(Center)
        .height(140)
        .width(Fill);

        let trash_icon = svg(svg::Handle::from_memory(include_bytes!(
            "../../src/assets/icons/trash.svg"
        )))
        .width(15)
        .style(|theme: &Theme, _| svg::Style {
            color: Some(button::primary(theme, button::Status::Active).text_color),
        });

        let up_icon = svg(svg::Handle::from_memory(include_bytes!(
            "../../src/assets/icons/up.svg"
        )))
        .width(15)
        .style(|theme: &Theme, _| svg::Style {
            color: Some(button::primary(theme, button::Status::Active).text_color),
        });

        let down_icon = svg(svg::Handle::from_memory(include_bytes!(
            "../../src/assets/icons/down.svg"
        )))
        .width(15)
        .style(|theme: &Theme, _| svg::Style {
            color: Some(button::primary(theme, button::Status::Active).text_color),
        });

        let delete_btn = button(trash_icon).on_press(Message::Delete);
        let up_btn = button(up_icon).on_press(Message::MovePanelUp);
        let down_btn = button(down_icon).on_press(Message::MovePanelDown);

        let content_col = column![
            inputs_row,
            // rule::horizontal(1),
            text("Pipe Sizing: Cold Water Demand"),
            self.pipe_size_panel
                .view(&self.dp_results.q_tw)
                .map(move |message| Message::PipeSizePanel(message)),
            space().height(5),
            text("Pipe Sizing: Heated Water Demand"),
            self.pipe_size_panel_hw
                .view(&self.dp_results.q_hw)
                .map(move |message| Message::PipeSizePanelHw(message)),
            space().height(5),
            // text("Pipe Sizing: Cold Water Demand").size(12),
            // self.pipe_sizw_panel_cw
            //     .view(&self.dp_results.q_cw)
            //     .map(move |message| Message::PipeSizePanelCw(message)),
            row![
                space().width(Fill),
                delete_btn,
                //button(text("Delete").size(12)).on_press(Message::Delete),
            ],
        ]
        .spacing(5)
        .width(Fill);

        let content: Element<'_, Message> = if self.is_expanded {
            container(content_col).into()
        } else {
            space().height(0).into()
        };

        let apts_text = if self.n_apts.is_empty() {
            text("Apartments: N/A")
        } else {
            text(format!("Apartments: {}", &self.n_apts))
        };

        let occs_text = if self.n_apts.is_empty() {
            text("Occupants: N/A")
        } else {
            text(format!("Occupants: {}", self.n_occupants))
        };

        let q99_cw_text = if self.q_tw.is_empty() {
            text("")
        } else {
            text(format!("Q₉₉ - Cold Water: {}", self.q_tw))
        };

        let q99_hw_text = if self.q_hw.is_empty() {
            text("")
        } else {
            text(format!("Q₉₉ - Heated Water: {}", self.q_hw))
        };

        let fixture_qty_header = if self.n_fixtures == 0 {
            text("Fixtures: N/A")
        } else {
            text(format!("Fixtures: {}", self.n_fixtures))
        };

        let header_row = row![
            panel_name,
            rule::vertical(1),
            fixture_qty_header,
            apts_text,
            occs_text,
            space().width(Fill),
            q99_cw_text,
            rule::vertical(1),
            q99_hw_text,
            toggle_btn,
            up_btn,
            down_btn,
        ]
        .spacing(10)
        .align_y(Center)
        .width(Fill);

        container(
            column![header_row, content,].spacing(12).width(Fill), // .into()
        )
        .padding(10)
        .align_x(Center)
        .style(|theme| container::Style {
            // background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: theme.palette().text, //Color::from_rgb8(180, 180, 180),
                width: 1.0,
                radius: 8.0.into(),
            },

            shadow: Shadow {
                color: Color::from_rgba8(0, 0, 0, 0.25),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 10.0,
            },

            ..Default::default()
        })
        .into()
    }
}
