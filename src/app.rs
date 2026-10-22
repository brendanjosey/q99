use iced::Alignment::Center;
use iced::widget::{column, container, row, svg, text};
use iced::{Element, Fill, Task, Theme, border};

use crate::components::sidebar;
use crate::screens::{demands, settings};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Demands,
    Settings,
}

pub struct App {
    active_screen: Screen,

    // Each screen retains its state when navigating away.
    demands: demands::State,
    // Settings are app-wide state.
    settings: settings::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Screen),
    Demands(demands::Message),
    Settings(settings::Message),
}

impl App {
    pub fn new() -> Self {
        Self {
            active_screen: Screen::Demands,
            demands: demands::State::default(),
            settings: settings::State::load_or_create().unwrap_or_default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(screen) => {
                self.active_screen = screen;
                Task::none()
            }

            Message::Demands(message) => self.demands.update(message).map(Message::Demands),

            Message::Settings(message) => self.settings.update(message).map(Message::Settings),
            // Task::none()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = sidebar::view(self.active_screen).map(Message::Navigate);

        let screen: Element<'_, Message> = match self.active_screen {
            Screen::Demands => self.demands.view().map(Message::Demands),

            Screen::Settings => self.settings.view().map(Message::Settings),
        };

        let logo = svg("src/assets/icons/q_xcix_logo.svg")
            .width(50)
            .style(|theme: &Theme, _| svg::Style {
                color: Some(theme.palette().text),
            });

        let top_bar = column![
            text("")
            // logo,
            // text(" Cold and Heated Water Demand Estimation Tool for Residential Buildings")
            // .size(12),
        ]
        .width(Fill);
        // .align_x(Center);

        let bottom_bar = row![text("version: 0.1.0 (Beta)").size(10),]
            .width(Fill)
            .align_y(Center);

        let body = container(screen)
            .padding(24)
            .width(Fill)
            .height(Fill)
            .style(|theme: &Theme| {
                container::Style::default()
                    .border(border::color(theme.palette().text).width(1).rounded(8))
            });

        let sidebar_body = row![
            sidebar, body,
            //container(screen).padding(24).width(Fill).height(Fill),
        ]
        .width(Fill)
        .height(Fill)
        .spacing(5);
        //.padding(10);
        //.into()

        column![top_bar, sidebar_body, bottom_bar]
            .width(Fill)
            .height(Fill)
            .padding(10)
            .spacing(10)
            .into()
    }

    pub fn theme(&self) -> Theme {
        if self.settings.dark_mode {
            // Theme::Dark
            Theme::TokyoNight
        } else {
            Theme::TokyoNightLight
        }
    }
}
