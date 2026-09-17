// src/main.rs
mod app;
mod screens;
mod components;
mod props;
// mod services;
// mod api;
// mod components;

use app::App;

fn main() -> iced::Result {
    iced::application(
        App::new,
        App::update,
        App::view,
    )
    .title("Application Title")
    .theme(App::theme)
    .run()
}
