use std::f32::consts::E;

// src/screens/project_details.rs
use iced::widget::{Column, button, column, container, row, rule, space, text, text_input};
use iced::{Border, Center, Color, Element, Fill, Right, Shadow, Task, Vector};
use short_id::short_id_ordered;

use crate::props::appliances::{Appliance, ApplianceType};
use crate::props::core::{EndUseProps, EndUseType};
use crate::props::fixtures::{Fixture, FixtureType};

// #[derive(Default)]
pub struct DemandPanel {
    panel_id: String,
    panel_name: String,
    is_expanded: bool,
    n_apts: String,
    n_occupants: String,
    n_fixtures: usize,
    q_total: Option<f64>,
    fixtures: [Fixture; 6],
    appliances: [Appliance; 2],
    //end_use_props: Option<[&EndUseProps; 8]>,
}

impl Default for DemandPanel {
    fn default() -> Self {
        Self {
            panel_id: short_id_ordered(),
            panel_name: String::from("New Demand Panel"),
            is_expanded: true,
            n_apts: String::from("1"),
            n_occupants: String::from("1"),
            n_fixtures: 0,
            q_total: None,
            fixtures: [
                Fixture::shower(),
                Fixture::bath(),
                Fixture::toilet(),
                Fixture::tap_basin(),
                Fixture::tap_kitchen(),
                Fixture::tap_laundry(),
            ],
            appliances: [Appliance::washing_machine(), Appliance::dishwasher()],
            //end_use_props: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PanelNameChanged(String),
    ExpandContract,
    AptsChanged(String),
    OccChanged(String),
    FixtureQtyChanged(FixtureType, String),
    ApplianceQtyChanged(ApplianceType, String),
}

impl DemandPanel {
    // pub fn update_props(&mut self, end_use_props: [&EndUseProps; 8]) {
    //     // updates props
    //     self.end_use_props = Some(end_use_props);
    // }

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

                    let appliance_qty = self
                        .appliances
                        .iter()
                        .map(|a| a.qty() as usize)
                        .sum::<usize>();

                    self.n_fixtures = fixture_qty + appliance_qty;
                }
            }

            Message::ApplianceQtyChanged(appliance_type, qty) => {
                if let Some(appliance) = self
                    .appliances
                    .iter_mut()
                    .find(|appliance| appliance.appliance_type() == &appliance_type)
                {
                    match qty.trim().parse() {
                        Ok(n) => {
                            // update appliance qty
                            appliance.set_qty(n);
                            // calc total number of fixtures and appliances
                            let fixture_qty = self
                                .fixtures
                                .iter()
                                .map(|f| f.qty() as usize)
                                .sum::<usize>();

                            let appliance_qty = self
                                .appliances
                                .iter()
                                .map(|a| a.qty() as usize)
                                .sum::<usize>();

                            self.n_fixtures = fixture_qty + appliance_qty;
                        }
                        _ => (),
                    }
                }
            }
        }

        Task::none()
    }

    pub fn view<'a>(&self, end_use_props: &'a [EndUseProps; 8]) -> Element<'_, Message> {
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
            text(">").align_x(Center)
        } else {
            text("↓").align_x(Center)
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
        .spacing(10)
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
                // _ => "Fixture",
            };

            let mut flow_rate: f64 = 0.0;
            for props in end_use_props {
                match &props.end_use_type {
                    EndUseType::Fixture(ft) => {
                        if ft == fixture_type {
                            flow_rate = props.flow_rate.unwrap_or(0.0);
                        }
                    }
                    _ => (),
                }
            }

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
                text(format!("p: {:.3}%", fixture.p_value()))
                    .size(10)
                    .width(Fill)
                    .align_x(Center),
                text(format!("flow rate: {:.2}L/s", flow_rate))
                    .size(10)
                    .width(Fill)
                    .align_x(Center),
            ]
            .spacing(2)
            .align_x(Center)
            .width(100);

            fixtures_row = fixtures_row.push(input);
        }

        let fixture_inputs = column![
            text("Fixtures").size(16).width(Fill).align_x(Center),
            fixtures_row,
        ]
        .spacing(5)
        .align_x(Center)
        .width(600);

        let mut appliance_row = row![].spacing(2).width(Fill);

        for appliance in &self.appliances {
            let appliance_type = appliance.appliance_type();

            let name = match appliance_type {
                ApplianceType::WashingMachine => "Washing Machine",
                ApplianceType::Dishwasher => "Dishwasher",
            };

            let input = column![
                text(name).size(10).width(Fill).align_x(Center),
                text_input("Qty", &format!("{}", appliance.qty()))
                    .width(70)
                    .align_x(Center)
                    .on_input(move |value| {
                        Message::ApplianceQtyChanged(appliance_type.clone(), value)
                    }),
                text(format!("p: {:.3}%", appliance.p_value()))
                    .size(10)
                    .width(Fill)
                    .align_x(Center),
            ]
            .spacing(5)
            .align_x(Center)
            .width(100);

            appliance_row = appliance_row.push(input);
        }

        let appliance_inputs = column![
            text("Appliances").size(16).width(Fill).align_x(Center),
            appliance_row,
        ]
        .spacing(5)
        .align_x(Center)
        .width(200);

        let inputs_row = row![
            building_inputs,
            rule::vertical(1),
            // fixtures
            fixture_inputs,
            //rule::vertical(1),
            //appliances
            appliance_inputs,
            rule::vertical(1),
            // space
            space().width(Fill),
            text("Results:").width(150),
        ]
        .spacing(10)
        .align_y(Center)
        .height(200)
        .width(Fill);

        let content: Element<'_, Message> = if self.is_expanded {
            container(inputs_row).into()
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

// fn input_col<'a>(
//     name: &'a str,
//     qty: &'a usize,
//     p_value: &'a f64,
//     on_input: impl Fn(String) -> Message + 'a,
// ) -> Column<'a, Message> {
//     column![
//         text(name).size(10).width(Fill).align_x(Center),
//         text_input("Qty", &format!("{}", qty))
//             .width(70)
//             .align_x(Center)
//             .on_input(on_input),
//         text(p_value.to_string())
//             .size(10)
//             .width(Fill)
//             .align_x(Center),
//     ]
//     .spacing(2)
//     .align_x(Center)
//     .width(100)
// }
