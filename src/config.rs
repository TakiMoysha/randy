use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const SUPPORTED_FORMATS: [&str; 2] = ["yml", "toml"];
const DEFAULT_NAME: [&str; 2] = ["randy", ".randy"];
const DEFAULT_PATHS: [&str; 3] = ["XDG_CONFIG_HOME", "HOME", "/etc"];

fn build_full_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for path in DEFAULT_PATHS {
        if let Ok(path) = std::env::var(path) {
            for name in DEFAULT_NAME {
                for format in SUPPORTED_FORMATS {
                    paths.push(PathBuf::from(format!("{path}/{name}.{format}")));
                }
            }
        } else {
            for name in DEFAULT_NAME {
                for format in SUPPORTED_FORMATS {
                    paths.push(PathBuf::from(format!("{path}/{name}.{format}")));
                }
            }
        }
    }
    paths
}

/// Find the default config file or exit with code 1
pub fn find_default_config() -> PathBuf {
    let paths = build_full_paths();
    let path = paths.iter().find(|f| f.exists()).cloned();

    path.unwrap_or_else(|| {
        eprintln!("Could not find a randy.yml config file.\n Checked:");
        paths.iter().for_each(|f| eprintln!("\t - {}", f.display()));
        eprintln!("Please put a randy.yml config file in one of those places or use --config.");
        eprintln!("Exmples: https://github.com/iphands/randy/tree/main/config");
        std::process::exit(1);
    })
}

// defaults
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub ui: Option<Vec<Ui>>,
}

fn default_as_false() -> bool {
    false
}
fn default_as_true() -> bool {
    true
}
fn default_text_size() -> String {
    String::from("large")
}
fn default_text_font_family() -> String {
    String::from("monospace")
}
fn default_base_opacity() -> f64 {
    1.0_f64
}
fn default_bar_height() -> String {
    String::from("10px")
}
fn default_color_text() -> String {
    String::from("#e1eeeb")
}
fn default_color_bar() -> String {
    String::from("#e1eeff")
}
fn default_color_bar_high() -> String {
    String::from("#ffaaaa")
}
fn default_color_bar_med() -> String {
    String::from("#ffeeaa")
}
fn default_color_borders() -> String {
    String::from("#e1eeeb")
}
fn default_color_label() -> String {
    String::from("#87d7ff")
}
fn default_color_background() -> String {
    String::from("rgba(0, 0, 0, 0.5)")
}
fn default_color_trough() -> String {
    String::from("rgba(0, 0, 0, 0)")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    timeout: Option<u8>,
    #[serde(default = "default_color_text")]
    pub color_text: String,
    #[serde(default = "default_color_bar")]
    pub color_bar: String,
    #[serde(default = "default_color_bar_med")]
    pub color_bar_med: String,
    #[serde(default = "default_color_bar_high")]
    pub color_bar_high: String,
    #[serde(default = "default_color_label")]
    pub color_label: String,
    #[serde(default = "default_color_background")]
    pub color_bg: String,
    #[serde(default = "default_color_borders")]
    pub color_borders: String,
    #[serde(default = "default_color_trough")]
    pub color_trough: String,
    #[serde(default = "default_as_false")]
    pub decoration: bool,
    #[serde(default = "default_as_false")]
    pub resizable: bool,
    #[serde(default = "default_text_size")]
    pub font_size: String,
    #[serde(default = "default_text_font_family")]
    pub font_family: String,
    #[serde(default = "default_base_opacity")]
    pub base_opacity: f64,
    #[serde(default = "default_bar_height")]
    pub bar_height: String,
    // depreated for gtk-4, wayland
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ui {
    r#type: String,
    text: Option<String>,
    limit: Option<u8>,
    items: Option<Vec<UiItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
// pub struct Widget {
pub struct UiItem {
    func: Option<String>,
    text: Option<String>,
}

pub fn load_config(path: PathBuf) -> Config {
    println!("Using config file: {}", path.display());
    let file = std::fs::read_to_string(&path).expect("Unable to read config file");

    match path.extension().and_then(|s| s.to_str()) {
        Some("yml") => serde_yml::from_str(&file).expect("Can't parse config file"),
        Some("toml") => toml::from_str(&file).expect("Can't parse config file"),
        _ => panic!("Unknown config format: {:?}", path.extension()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_config_from_env() -> PathBuf {
        std::env::set_var(
            "XDG_CONFIG_HOME",
            std::env::current_dir().unwrap().join("config"),
        );
        find_default_config()
    }

    #[test]
    fn should_get_all_paths() {
        let paths = build_full_paths();
        assert!(!paths.is_empty());
    }

    #[test]
    fn should_return_default_yaml_config() {
        let path = load_test_config_from_env();
        assert!(path.extension().unwrap() == "yml", "Bad format: {:?}", path);
        assert!(path.exists(), "Can't find default config, it exists?");
    }

    #[ignore = "how set default extension config?"]
    #[test]
    fn should_return_default_toml_config() {}

    #[test]
    fn should_parse_config() {
        let path = load_test_config_from_env();
        let config = load_config(path);
        println!("{:#?}", config);
        // assert!(config);
    }
}
