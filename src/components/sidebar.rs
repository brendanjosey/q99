use iced::widget::{button, column, container, text};
use iced::{Element, Fill};

use crate::app::Screen;

pub fn view(active_screen: Screen) -> Element<'static, Screen> {

    let navigation_button = |label, target| {
        let label = if active_screen == target {
            format!("● {label}")
        } else {
            format!("  {label}")
        };

        button(text(label))
            .on_press(target)
            .padding(12)
            .width(Fill)
    };

    let sidebar = column![
        text("My Application").size(22),
        // horizontal_rule(1),
        navigation_button(
            "Demand Estimates",
            Screen::Demands,
        ),

        navigation_button(
            "Settings",
            Screen::Settings,
        ),
    ]
    .spacing(8)
    .width(Fill);

    container(sidebar)
        .padding(16)
        .width(220)
        .height(Fill)
        .into()
}
