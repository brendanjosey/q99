use directories::BaseDirs;
use iced::widget::{button, column, text};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn settings_path() -> Result<PathBuf, Box<dyn Error>> {
    let base = BaseDirs::new().ok_or("could not find a config directory")?;
    Ok(base.config_dir().join("Demands").join("settings.json"))
}

fn write_settings(path: &Path, settings: &SettingsV1) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let settings_file = SettingsFile {
        format_version: 1,
        state: serde_json::to_value(settings)?,
    };

    let json = serde_json::to_string_pretty(&settings_file)?;

    fs::write(path, json)?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct SettingsFile {
    format_version: u32,
    state: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
struct SettingsV1 {
    dark_mode: bool,
}

#[derive(Default, Deserialize, Serialize)]
pub struct State {
    pub dark_mode: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
}

impl State {
    // save function
    fn save_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings = SettingsV1 {
            dark_mode: self.dark_mode,
        };

        let file = SettingsFile {
            format_version: 1,
            state: serde_json::to_value(settings)?,
        };

        let pref_path = settings_path()?;

        let json = serde_json::to_vec_pretty(&file)?;
        std::fs::write(pref_path, json)?;
        Ok(())
    }

    pub fn load_or_create() -> Result<Self, Box<dyn Error>> {
        let path = settings_path()?;

        let state = match fs::read_to_string(&path) {
            Ok(_) => {
                // read file
                let bytes = std::fs::read(&path)?;
                let file: SettingsFile = serde_json::from_slice(&bytes)?;

                if file.format_version != 1 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unsupported save version: {}", file.format_version),
                    )
                    .into());
                }

                let settings: Self = serde_json::from_value(file.state)?;

                settings
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // create new struct
                let settings = State { dark_mode: false };

                // create new settings struct
                let settings_v1 = SettingsV1 {
                    dark_mode: settings.dark_mode,
                };

                // write new settings file
                let _ = write_settings(&path, &settings_v1);

                // return new struct
                settings
            }

            Err(_) => State { dark_mode: false },
        };

        Ok(state)
    }

    // pub fn new() -> Self {
    //     // load settings
    //     //
    //     Self { dark_mode: true }
    // }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleTheme => {
                if self.dark_mode {
                    self.dark_mode = false
                } else {
                    self.dark_mode = true
                }

                //update settings
                let _ = self.save_settings();
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
