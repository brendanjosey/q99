use iced::widget::{pick_list, row, space, text, text_input};
use iced::{Center, Element, Fill, Task};
use std::f64::consts::PI;

use crate::props::pipe_sizing::{CopperPipeDia, PexPipeDia, PipeMaterial};

#[derive(Default, Debug, Clone)]
pub struct PipeSizePanel {
    pipe_material: Option<PipeMaterial>,
    pipe_dia_copper: Option<CopperPipeDia>,
    pipe_dia_aus_pex: Option<PexPipeDia>,
    continuous_flow_rate_str: String,
    continuous_flow_rate: Option<f64>,
    // velocity_str: String,
    // velocity: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum Message {
    PipeMaterialChanged(PipeMaterial),
    CopperPipeDiaChanged(CopperPipeDia),
    AusPexDiaChanged(PexPipeDia),
    ContFlowChanged(String),
}

impl PipeSizePanel {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PipeMaterialChanged(value) => {
                self.pipe_material = Some(value);
            }

            Message::CopperPipeDiaChanged(value) => {
                self.pipe_dia_copper = Some(value);
            }

            Message::AusPexDiaChanged(value) => {
                self.pipe_dia_aus_pex = Some(value);
            }

            Message::ContFlowChanged(value) => {
                self.continuous_flow_rate_str = value.clone();

                match value.trim().parse::<f64>() {
                    Ok(q) => self.continuous_flow_rate = Some(q),
                    _ => self.continuous_flow_rate = None,
                }
            }
        }

        Task::none()
    }

    pub fn view<'a>(
        &self,
        q99: &'a Option<f64>,
        // q99_hw: Option<f64>,
        // q99_cw: Option<f64>,
    ) -> Element<'a, Message> {
        //
        let pipe_material = pick_list(
            [PipeMaterial::Copper, PipeMaterial::Pex],
            self.pipe_material.clone(),
            Message::PipeMaterialChanged,
        );

        let pipe_dia_dd: Element<'_, Message> = match self.pipe_material {
            Some(PipeMaterial::Copper) => pick_list(
                CopperPipeDia::ALL,
                self.pipe_dia_copper.clone(),
                Message::CopperPipeDiaChanged,
            )
            .width(150)
            .into(),
            Some(PipeMaterial::Pex) => pick_list(
                PexPipeDia::ALL,
                self.pipe_dia_aus_pex.clone(),
                Message::AusPexDiaChanged,
            )
            .width(150)
            .into(),
            _ => text("").width(150).into(),
        };

        let pipe_dia: Element<'_, Message> = match &self.pipe_material {
            Some(PipeMaterial::Copper) => match &self.pipe_dia_copper {
                Some(pipe) => text(format!("{:.1}mm", pipe.pipe_dia()))
                    .size(12)
                    .width(70)
                    .into(),
                _ => text("").width(70).into(),
            },
            Some(PipeMaterial::Pex) => match &self.pipe_dia_aus_pex {
                Some(pipe) => text(format!("{:.1}mm", pipe.pipe_dia()))
                    .size(12)
                    .width(70)
                    .into(),
                _ => text("").width(70).into(),
            },
            _ => text("").width(70).into(),
        };

        let cont_flow = text_input("", &self.continuous_flow_rate_str)
            .on_input(Message::ContFlowChanged)
            .width(70);

        let velocity = || {
            // pipe diameter
            let pipe_dia = match self.pipe_material {
                Some(PipeMaterial::Copper) => match &self.pipe_dia_copper {
                    Some(pipe) => Some(pipe.pipe_dia()),
                    None => None,
                },
                Some(PipeMaterial::Pex) => match &self.pipe_dia_aus_pex {
                    Some(pipe) => Some(pipe.pipe_dia()),
                    None => None,
                },
                None => None,
            };

            let q_cont = if self.continuous_flow_rate_str.len() == 0 {
                Some(0.0)
            } else {
                self.continuous_flow_rate
            };

            // calc velocity
            let v = match (q99, q_cont, pipe_dia) {
                (Some(q), Some(qc), Some(d)) => {
                    let a = 0.25 * PI * (d / 1000.0).powf(2.0);
                    let flow = (q + qc) / 1000.0;

                    text(format!(
                        "Velocity @ {:.2}L/s = {:.2}m/s",
                        (q + qc),
                        (flow / a)
                    ))
                    .size(12)
                    .width(230)
                }
                _ => text("Velocity: N/A").size(12).width(230),
            };

            v
        };

        row![
            text("Pipe Material:").size(12),
            pipe_material,
            space().width(20),
            text("Pipe Diameter:").size(12),
            pipe_dia_dd,
            pipe_dia,
            space().width(20),
            text("Continuouse Flow:").size(12),
            cont_flow,
            text("L/s").size(12),
            space().width(Fill),
            velocity(),
        ]
        .spacing(5)
        .width(1200)
        .align_y(Center)
        .into()
    }
}
