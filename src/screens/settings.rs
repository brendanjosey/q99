use iced::widget::{button, column, text};
use iced::{Element, Task};

#[derive(Default)]
pub struct State {
    pub dark_mode: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme
}

impl State {

    pub fn new() -> Self {
        Self { dark_mode: true }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleTheme => {
                if self.dark_mode {
                    self.dark_mode = false
                } else {
                    self.dark_mode = true
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Settings").size(32),
            button("Toggle Theme").on_press(Message::ToggleTheme)
        ]
        .spacing(12)
        .into()
    }
}
