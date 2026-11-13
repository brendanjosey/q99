#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod app;
mod components;
mod props;
mod screens;
use app::App;


fn main() -> iced::Result {

    let icon = Some(
        iced::window::icon::from_file_data(include_bytes!("assets/app.ico"), None).expect("invalid window icon")
    );

    let settings = iced::window::Settings {
        icon,
        ..Default::default()
    };

    iced::application(App::new, App::update, App::view)
        .title("Q99")
        .theme(App::theme)
        .window(settings)
        .run()
}
