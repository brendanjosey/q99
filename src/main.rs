mod app;
mod components;
mod props;
mod screens;
use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Application Title")
        .theme(App::theme)
        .run()
}
