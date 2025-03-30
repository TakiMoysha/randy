use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::{fs::load_user_config_path, parser::parser_to_config};

// defaults
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub ui: Option<Vec<UiContainer>>,
}

#[serde_inline_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub timeout: Option<u8>,

    #[serde_inline_default("#e1eeeb")]
    pub color_text: &'static str,
    #[serde_inline_default("#e1eeff")]
    pub color_bar: &'static str,
    #[serde_inline_default("#ffeeaa")]
    pub color_bar_med: &'static str,
    #[serde_inline_default("#ffaaaa")]
    pub color_bar_high: &'static str,
    #[serde_inline_default("#87d7ff")]
    pub color_label: &'static str,
    #[serde_inline_default("#e1eeeb")]
    pub color_borders: &'static str,
    #[serde_inline_default("rgba(0, 0, 0, 0.5)")]
    pub color_bg: &'static str,
    #[serde_inline_default("rgba(0, 0, 0, 0)")]
    pub color_trough: &'static str,
    #[serde_inline_default(false)]
    pub decoration: bool,
    #[serde_inline_default(false)]
    pub resizable: bool,
    #[serde_inline_default("large")]
    pub font_size: &'static str,
    #[serde_inline_default("monospace")]
    pub font_family: &'static str,
    #[serde_inline_default(1.0_f64)]
    pub base_opacity: f64,
    #[serde_inline_default("10px")]
    pub bar_height: &'static str,
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
        let path = if let Some(path) = path {
            path
        } else {
            load_user_config_path().expect("Unable to load config file")
        };
        let content = std::fs::read_to_string(&path).expect("Unable to read config file");
        let config = parser_to_config(&content);
        config
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
