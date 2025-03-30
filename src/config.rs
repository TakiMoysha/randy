use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{fs::load_user_config_path, parser::parser_to_config};

// defaults
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub ui: Option<Vec<UiContainer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub timeout: u8,
    pub style: String,

    pub color_text: String,
    pub color_bar: String,
    pub color_bar_med: String,
    pub color_bar_high: String,
    pub color_label: String,
    pub color_borders: String,
    pub color_bg: String,
    pub color_trough: String,
    pub decoration: bool,
    pub resizable: bool,
    pub font_size: String,
    pub font_family: String,
    pub base_opacity: f64,
    pub bar_height: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            timeout: 1,
            style: "./static/app.css".to_string(),
            color_text: "#e1eeeb".to_string(),
            color_bar: "#e1eeff".to_string(),
            color_bar_med: "#ffeeaa".to_string(),
            color_bar_high: "#ffaaaa".to_string(),
            color_label: "#87d7ff".to_string(),
            color_borders: "#e1eeeb".to_string(),
            color_bg: "rgba(0, 0, 0, 0.5)".to_string(),
            color_trough: "rgba(0, 0, 0, 0)".to_string(),
            decoration: false,
            resizable: false,
            font_size: "large".to_string(),
            font_family: "monospace".to_string(),
            base_opacity: 1.0_f64,
            bar_height: "10px".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiContainer {
    block: String,
    limit: Option<u8>,
    text: Option<String>,
    items: Option<Vec<UiBlock>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiBlock {
    text: Option<String>,
}

impl Config {
    pub fn load_config(path: Option<PathBuf>) -> Config {
        let path: Option<PathBuf> = path.or(load_user_config_path());

        if let Some(path) = path {
            let content = std::fs::read_to_string(&path).expect("Unable to read config file");
            parser_to_config(&content)
        } else {
            Config::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let default_ui: Vec<UiContainer> = vec![];
        Config {
            settings: Settings::default(),
            ui: Some(default_ui),
        }
    }
}
