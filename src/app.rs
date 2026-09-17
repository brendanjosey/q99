// src/app.rs
use iced::widget::{container, row};
use iced::{Element, Fill, Task, Theme};

use crate::screens::{demands, settings};
use crate::components::{sidebar};

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
            settings: settings::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(screen) => {
                self.active_screen = screen;
                Task::none()
            }

            Message::Demands(message) => self
                .demands
                .update(message)
                .map(Message::Demands),

            Message::Settings(message) => 
                self.settings.update(message)
                .map(Message::Settings),
                // Task::none()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = sidebar::view(self.active_screen)
            .map(Message::Navigate);

        let screen: Element<'_, Message> = match self.active_screen {
            Screen::Demands => self
                .demands
                .view()
                .map(Message::Demands),

            Screen::Settings => self
                .settings
                .view()
                .map(Message::Settings),
        };

        row![
            sidebar,
            container(screen)
                .padding(24)
                .width(Fill)
                .height(Fill),
        ]
        .width(Fill)
        .height(Fill)
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
