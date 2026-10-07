use iced::widget::{button, column, container, row, rule, space, text, text_input};
use iced::{Border, Center, Color, Element, Fill, Right, Shadow, Task, Vector};
//use short_id::short_id_ordered;

use crate::components::pipe_size_panel::{Message as PipeSizePanelMessage, PipeSizePanel};
use crate::props::core::{DemandPanelResults, FixtureProps, FlowType, TempProps};
use crate::props::fixtures::{Fixture, FixtureType};

#[derive(Debug)]
pub struct DemandPanel {
    //panel_id: String,
    panel_name: String,
    is_expanded: bool,
    n_apts: String,
    n_occupants: String,
    n_fixtures: usize,
    fixtures: [Fixture; 8],
    q_total: String,
    q_tw: String,
    q_cw: String,
    q_hw: String,
    p0: String,
    dp_results: DemandPanelResults,
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
}

impl DemandPanel {
    // pub fn update_props(&mut self, end_use_props: [&EndUseProps; 8]) {
    //     // updates props
    //     self.end_use_props = Some(end_use_props);
    // }
    pub fn new() -> Self {
        Self {
            //panel_id: short_id_ordered(),
            panel_name: String::from("New Demand Panel"),
            is_expanded: true,
            n_apts: String::new(),
            n_occupants: String::new(),
            n_fixtures: 0,
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
            q_tw: String::new(),
            q_cw: String::new(),
            q_hw: String::new(),
            p0: String::new(),
            dp_results: DemandPanelResults::default(),
            pipe_size_panel: PipeSizePanel::default(),
            pipe_size_panel_hw: PipeSizePanel::default(),
            // pipe_size_panel_cw: PipeSizePanel::default(),
        }
    }

    pub fn calc_total_flow_rate<'a>(&mut self, fixture_props: &'a [FixtureProps; 8]) {
        let mut total_flow = 0.0;

        for fixture in &self.fixtures {
            let ft = fixture.fixture_type();

            if let Some(props) = fixture_props.iter().find(|f| &f.fixture_type == ft) {
                let n = fixture.qty() as f64;
                let q = props.flow_rate.unwrap_or(0.0);
                total_flow += n * q;
            }
        }

        self.q_total = format!("{:.2}L/s", total_flow);
        self.dp_results.q_total = Some(total_flow);
    }

    fn cacl_p0<'a>(&mut self) {
        let mut p0_init = false;
        let mut p0 = 1.0;

        for fixture in &self.fixtures {
            // only include if fixtury qty is greater than zero
            if fixture.qty() > 0 {
                let n = fixture.qty() as f64;
                let p = fixture.p_value();

                if p0_init {
                    p0 *= (1.0 - p).powf(n);
                } else {
                    // initialise P0
                    p0 = (1.0 - p).powf(n);
                    p0_init = true;
                }
            }
        }

        self.p0 = format!("{:.2}%", p0 * 100.0);
        self.dp_results.p0 = Some(p0);
    }

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
        if self.n_apts.len() > 0 && self.n_occupants.len() > 0 {
            // functions to refresh results
            self.calc_total_flow_rate(fixture_props);
            self.cacl_p0();
            //println!("total demands");
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
            //println!("cold water demands");
            match self.calc_q99(fixture_props, temp_props, FlowType::ColdWaterDemand) {
                Ok(Some(q)) => {
                    self.q_cw = format!("{:.2}L/s", q);
                    self.dp_results.q_cw = Some(q);
                }
                _ => {
                    self.q_cw = "N/A".to_string();
                    self.dp_results.q_cw = None;
                }
            }
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
                    let qty = qty.trim().parse::<usize>();
                    fixture.set_qty(qty.clone());
                    let fixture_qty = self
                        .fixtures
                        .iter()
                        .map(|f| f.qty() as usize)
                        .sum::<usize>();

                    self.n_fixtures = fixture_qty;
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
        }

        Task::none()
    }

    pub fn view<'a>(&self, fixture_props: &'a [FixtureProps; 8]) -> Element<'_, Message> {
        let panel_name: Element<'_, Message> = if self.is_expanded {
            text_input("Enter panel name...", &self.panel_name)
                .width(300)
                .on_input(Message::PanelNameChanged)
                .into()
        } else {
            if self.panel_name.len() > 0 {
                text(&self.panel_name).width(300).into()
            } else {
                text("Demand Panel").width(300).into()
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

        let results_col = column![
            text("Results").size(16).width(Fill).align_x(Center),
            // fixtures
            text(format!("Fixtures: {}", self.n_fixtures))
                .size(12)
                .width(Fill)
                .align_x(Right),
            // total flow rate
            text(format!("Total Flow Rate: {}", &self.q_total))
                .size(12)
                .width(Fill)
                .align_x(Right),
            // peak hour stagnation probability
            text(format!("Probability of Stagnation: {}", &self.p0))
                .size(12)
                .width(Fill)
                .align_x(Right),
            // 99th percentile demands
            text(format!(
                "99th Percentile Demand - Cold & Heated Water: {}",
                &self.q_tw
            ))
            .size(12)
            .width(Fill)
            .align_x(Right),
            text(format!(
                "99th Percentile Demand - Heated Water: {}",
                &self.q_hw
            ))
            .size(12)
            .width(Fill)
            .align_x(Right),
            text(format!(
                "99th Percentile Demand - Cold Water: {}",
                &self.q_cw
            ))
            .size(12)
            .width(Fill)
            .align_x(Right),
            //button("Delete").on_press(Message::Delete),
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
        .height(200)
        .width(Fill);

        let content_col = column![
            inputs_row,
            // rule::horizontal(1),
            text("Pipe Sizing: Total Water Demand").size(12),
            self.pipe_size_panel
                .view(&self.dp_results.q_tw)
                .map(move |message| Message::PipeSizePanel(message)),
            space().height(5),
            text("Pipe Sizing: Heated Water Demand").size(12),
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
                button(text("Delete").size(12)).on_press(Message::Delete),
            ],
        ]
        .spacing(5)
        .width(Fill);

        let content: Element<'_, Message> = if self.is_expanded {
            container(content_col).into()
        } else {
            space().height(0).into()
        };

        let header_row = row![
            panel_name,
            space().width(Fill),
            text(format!("Apartments: {}", &self.n_apts)),
            text(format!("Occupants: {}", &self.n_occupants)),
            text(format!("Fixtures: {}", &self.n_fixtures)),
            toggle_btn,
        ]
        .spacing(10)
        .width(Fill);

        container(
            column![header_row, content,].spacing(12).width(Fill), // .into()
        )
        .padding(10)
        .align_x(Center)
        .style(|_theme| container::Style {
            // background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: Color::from_rgb8(180, 180, 180),
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
