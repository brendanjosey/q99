use iced::widget::{button, column, container, space, svg};
use iced::{Center, Element, Fill, Theme, border};

use crate::app::Screen;

pub fn view(active_screen: Screen) -> Element<'static, Screen> {
    let navigation_button = |icon_bytes: &'static [u8], target| {
        let handle = svg::Handle::from_memory(icon_bytes);
        let icon = svg(handle).style(|theme: &Theme, _| svg::Style {
            color: Some(button::primary(theme, button::Status::Active).text_color),
        });

        if active_screen == target {
            button(icon).on_press(target)
        } else {
            button(icon).on_press(target).style(|theme, _| {
                let style = button::primary(theme, button::Status::Disabled);
                // let mut style = button::secondary(theme, status);
                // style.background ;
                // //style.text_color = Color::WHITE;
                style
            })
        }
    };

    let sidebar = column![
        // horizontal_rule(1),
        // button(image("src/assets/icons/drop.png").width(50)).on_press(Screen::Demands),
        //navigation_button("src/assets/icons/home.svg", Screen::Settings),
        navigation_button(
            include_bytes!("../../src/assets/icons/q_xcix_icon.svg"),
            Screen::Demands
        ),
        space().height(Fill),
        navigation_button(
            include_bytes!("../../src/assets/icons/settings.svg"),
            Screen::Settings
        ),
    ]
    .spacing(8)
    .align_x(Center)
    .width(Fill);

    container(sidebar)
        .padding(16)
        .width(80)
        .height(Fill)
        .style(|theme: &Theme| {
            container::Style::default()
                .border(border::color(theme.palette().text).width(1).rounded(8))
        })
        .into()
}
