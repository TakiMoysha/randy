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

fn default_as_false() -> bool {
    false
}
fn default_as_true() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,
    pub ui: Option<Vec<Ui>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    timeout: Option<u8>,
    color_bar: Option<String>,
    color_label: Option<String>,
    color_text: Option<String>,
    #[serde(default = "default_as_false")]
    pub decoration: bool,
    #[serde(default = "default_as_false")]
    pub resizable: bool,
    #[serde(default = "default_as_true")]
    pub skip_taskbar: bool,
    color_background: Option<String>,
    font_size: Option<String>,
    #[serde(default)] // depreated for gtk-4, wayland
    pub xpos: i32,
    #[serde(default)] // depreated for gtk-4, wayland
    pub ypos: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timeout: None,
            color_bar: None,
            color_label: None,
            color_text: None,
            decoration: false,
            resizable: false,
            skip_taskbar: true,
            color_background: None,
            font_size: None,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ui {
    r#type: String,
    text: Option<String>,
    limit: Option<u8>,
    items: Option<Vec<UiItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiItem {
    func: Option<String>,
    text: Option<String>,
}

pub fn load_config(path: PathBuf) -> Config {
    println!("Using config file: {}", path.display());
    let mut reader = BufReader::new(File::open(&path).expect("Unable to open config file"));

    match path.extension().and_then(|s| s.to_str()) {
        Some("yml") => {
            let mut buf = String::new();
            reader.read_to_string(&mut buf).unwrap();
            serde_yml::from_str(&buf).unwrap()
        }
        Some("toml") => {
            let mut buf = String::new();
            reader.read_to_string(&mut buf).unwrap();
            toml::from_str(&buf).unwrap()
        }
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
