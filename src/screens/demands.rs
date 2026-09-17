use iced::widget::{column, text, text_input};
use iced::{Element, Task};

use crate::components::demand_panel::{DemandPanel, Message as DemandPanelMessage};
use crate::props::core::EndUseProps;

//#[derive(Default)]
pub struct State {
    project_name: String,
    project_description: String,
    demand_panel: DemandPanel,
    end_use_props: [EndUseProps; 8],
}

impl Default for State {
    fn default() -> Self {
        Self {
            project_name: String::new(),
            project_description: String::new(),
            demand_panel: DemandPanel::default(),
            end_use_props: EndUseProps::all(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ProjectNameChanged(String),
    ProjectDescriptionChanged(String),
    DemandPanel(DemandPanelMessage),
    //EndUsePropsChanged,
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

            Message::DemandPanel(message) => {
                let _ = self.demand_panel.update(message).map(Message::DemandPanel);
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Project Details").size(24),
            text("Project name"),
            text_input("Enter the project name", &self.project_name,)
                .on_input(Message::ProjectNameChanged),
            text("Description"),
            text_input("Enter a description", &self.project_description,)
                .on_input(Message::ProjectDescriptionChanged),
            text("Fixture Settings").size(24),
            text("Demand Panels").size(24),
            self.demand_panel
                .view(&self.end_use_props)
                .map(Message::DemandPanel),
        ]
        .spacing(12)
        .into()
    }
}
