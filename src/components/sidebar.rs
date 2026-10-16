use iced::widget::{button, column, container, image, row, space, svg, text};
use iced::{Border, Center, Color, Element, Fill};

use crate::app::Screen;

pub fn view(active_screen: Screen) -> Element<'static, Screen> {
    // navigation button
    // let navigation_button = |icon_path, desc, target| {
    //     let btn_width = 30;
    //     let r = row![image(icon_path).width(btn_width), text(desc)]
    //         .spacing(2)
    //         .align_y(Center)
    //         .width(Fill);
    //
    //     let button = if active_screen == target {
    //         button(r).on_press(target)
    //     } else {
    //         button(r).on_press(target).style(|theme, status| {
    //             let mut style = button::secondary(theme, status);
    //             //style.background = Some(Color::from_rgb8(128, 128, 128).into());
    //             //style.background = Some(Color::from_rgb8(192, 192, 192).into());
    //             style.background = Some(Color::from_rgb8(64, 64, 64).into());
    //             //style.text_color = Color::WHITE;
    //             style
    //         })
    //     };
    //
    //     button
    // };
    //
    let navigation_button = |icon_path, target| {
        let btn = if active_screen == target {
            button(svg(icon_path).style(|_, _| svg::Style {
                color: Some(Color::from_rgb(0.188, 0.196, 0.286)),
            }))
            .on_press(target)
        } else {
            button(svg(icon_path).style(|_, _| svg::Style {
                color: Some(Color::from_rgb(0.188, 0.196, 0.286)),
            }))
            .on_press(target)
            .style(|theme, status| {
                let mut style = button::secondary(theme, status);
                //style.background = Some(Color::from_rgb8(128, 128, 128).into());
                //style.background = Some(Color::from_rgb8(192, 192, 192).into());
                style.background = Some(Color::from_rgb8(0, 0, 0).into());
                //style.text_color = Color::WHITE;
                style
            })
        };

        btn
    };

    let sidebar = column![
        // horizontal_rule(1),
        // button(image("src/assets/icons/drop.png").width(50)).on_press(Screen::Demands),
        navigation_button("src/assets/icons/home.svg", Screen::Settings),
        navigation_button("src/assets/icons/q99.svg", Screen::Demands),
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
        .style(|_theme| container::Style {
            // background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: Color::from_rgb8(180, 180, 180),
                width: 1.0,
                radius: 8.0.into(),
            },
            // shadow.. not sure if I want this.
            // shadow: Shadow {
            //     color: Color::from_rgba8(0, 0, 0, 0.25),
            //     offset: Vector::new(0.0, 4.0),
            //     blur_radius: 10.0,
            // },
            ..Default::default()
        })
        .into()
}
