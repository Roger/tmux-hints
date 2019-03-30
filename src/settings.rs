use std::fs::File;
use std::io::Read;

use once_cell::sync::OnceCell;
use crate::color::Color;

use serde::{Deserialize, Serialize};

static INSTANCE: OnceCell<Settings> = OnceCell::INIT;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Highlight {
    pub unselected: Color,
    pub selected: Color,
}

impl Default for Highlight {
    fn default() -> Self {
        Self {
            unselected: Color { foreground: 6, ..Default::default() },
            selected: Color { background: 6, ..Default::default() },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub opener: String,
    pub show_position: bool,
    pub hint: Highlight,
    pub position: Highlight,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            opener: "xdg-open".to_string(),
            show_position: true,
            hint: Default::default(),
            position: Default::default(),
        }
    }
}

impl Settings {
    pub fn global() -> &'static Settings {
        INSTANCE.get().expect("Settings are not initialized")
    }

    pub fn serialize() -> String {
        toml::to_string_pretty(Self::global()).expect("Can't serialize")
    }

    // TODO: handle more initialization errors and use default config
    pub fn init() {
        if INSTANCE.get().is_some() {
            panic!("Settings already initialized");
        }

        let base_dir = dirs::config_dir().expect("Can't find home config directory");
        let file_path = base_dir.join("tmux-hints.toml");
        let mut toml_str = String::new();

        // Read only if the settings file exists
        // if the toml_str is empty will use defaults for the settings
        if let Ok(mut toml_file) = File::open(file_path) {
            toml_file.read_to_string(&mut toml_str).expect("Can't read settings");
        }

        let settings = toml::from_str(&toml_str).expect("Invalid configuration");
        INSTANCE.set(settings).unwrap();
    }
}
