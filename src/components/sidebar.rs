use iced::widget::{button, column, container, space, svg};
use iced::{Center, Element, Fill, Theme, border};

use crate::app::Screen;

pub fn view(active_screen: Screen) -> Element<'static, Screen> {
    let navigation_button = |icon_path, target| {
        let icon = svg(icon_path).style(|theme: &Theme, _| svg::Style {
            color: Some(button::primary(theme, button::Status::Active).text_color),
        });

        let btn = if active_screen == target {
            button(icon).on_press(target)
        } else {
            button(icon).on_press(target).style(|theme, _| {
                let style = button::primary(theme, button::Status::Disabled);
                // let mut style = button::secondary(theme, status);
                // style.background ;
                // //style.text_color = Color::WHITE;
                style
            })
        };

        btn
    };

    let sidebar = column![
        // horizontal_rule(1),
        // button(image("src/assets/icons/drop.png").width(50)).on_press(Screen::Demands),
        //navigation_button("src/assets/icons/home.svg", Screen::Settings),
        navigation_button("src/assets/icons/q_xcix_icon.svg", Screen::Demands),
        space().height(Fill),
        navigation_button("src/assets/icons/settings.svg", Screen::Settings),
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
