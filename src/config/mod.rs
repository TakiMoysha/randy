use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

// const SUPPORTED_FORMATS: [&str; 1] = ["yml"];
const SUPPORTED_FORMATS: [&str; 1] = ["toml"];
// const SUPPORTED_FORMATS: [&str; 2] = ["yml", "toml"];
const DEFAULT_NAME: [&str; 2] = ["randy", ".randy"];
const DEFAULT_PATHS: [&str; 3] = ["XDG_CONFIG_HOME", "HOME", "/etc/"];

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
        eprintln!("{:?}", paths);
        eprintln!("Please put a randy.yml config file in one of those places.");
        eprintln!("Exmples: https://github.com/iphands/randy/tree/main/config");
        std::process::exit(1);
    })
}

pub fn load_config(path: PathBuf) {
    println!("Using config file: {}", path.display());
    let config = parse_config(path);
    println!("Config: {:#?}", config);
}

fn parse_config(path: PathBuf) -> Config {
    let file = File::open(&path).expect("Unable to open config file");
    let mut reader = BufReader::new(file);
    let extension = path.extension().unwrap().to_str().unwrap();
    if extension.eq("toml") {
        let mut buffer = String::new();
        reader
            .read_to_string(&mut buffer)
            .expect("Unable to parse config file");
        toml::from_str(&buffer).expect("Unable to parse config file")
    } else if extension.eq("yml") {
        // serde_yml::from_reader(reader).expect("Unable to parse config file");
        todo!("!!!!! YAML YET NOT IMPLEMENTED !!!!");
    } else {
        panic!("Unknown config format")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    settings: Settings,
    ui: Option<Vec<Ui>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    timeout: Option<u8>,
    color_bar: Option<String>,
    color_label: Option<String>,
    color_text: Option<String>,
    decoration: Option<bool>,
    resizable: Option<bool>,
    skip_taskbar: Option<bool>,
    color_background: Option<String>,
    font_size: Option<String>,
    xpos: Option<u16>,
    ypos: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ui {
    r#type: String,
    text: Option<String>,
    limit: Option<u8>,
    items: Option<Vec<UiItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UiItem {
    func: String,
    text: String,
}

// ###
fn get_file() -> String {
    // let config_path = try_get_file().expect("!!!!");
    // println!("Using config file: {}", config_path);
    // return match fs::read_to_string(&config_path) {
    //     Ok(s) => s,
    //     Err(_) => panic!("Unable to open/read {}", config_path),
    // };
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_config() -> PathBuf {
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
    fn should_return_default_config() {
        let path = load_test_config();
        println!("{:?}", path);
        assert!(path.exists(), "Can't find default config, it exists?");
    }

    #[test]
    fn should_parse_config() {
        let path = load_test_config();
        let config = parse_config(path);
        println!("{:#?}", config);
        // assert!(!config.is_empty());
    }
}
