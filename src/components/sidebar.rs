use iced::widget::{button, column, container, image, row, space, text};
use iced::{Center, Color, Element, Fill};

use crate::app::Screen;

pub fn view(active_screen: Screen) -> Element<'static, Screen> {
    // navigation button
    let navigation_button = |icon_path, desc, target| {
        let btn_width = 30;
        let r = row![image(icon_path).width(btn_width), text(desc)]
            .spacing(2)
            .align_y(Center)
            .width(Fill);

        let button = if active_screen == target {
            button(r).on_press(target)
        } else {
            button(r).on_press(target).style(|theme, status| {
                let mut style = button::secondary(theme, status);
                //style.background = Some(Color::from_rgb8(128, 128, 128).into());
                //style.background = Some(Color::from_rgb8(192, 192, 192).into());
                style.background = Some(Color::from_rgb8(64, 64, 64).into());
                //style.text_color = Color::WHITE;
                style
            })
        };

        button
    };

    let sidebar = column![
        text("Demands").size(22),
        // horizontal_rule(1),
        // button(image("src/assets/icons/drop.png").width(50)).on_press(Screen::Demands),
        navigation_button("src/assets/icons/home.png", "Home", Screen::Settings),
        navigation_button("src/assets/icons/drop.png", "Demands", Screen::Demands),
        space().height(Fill),
        navigation_button("src/assets/icons/setting.png", "Settings", Screen::Settings),
    ]
    .spacing(8)
    .align_x(Center)
    .width(Fill);

    container(sidebar)
        .padding(16)
        .width(175)
        .height(Fill)
        .into()
}
