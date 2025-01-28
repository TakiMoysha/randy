use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use serde_yml::to_string;

use crate::stubs::ConfigString;

const DEFAULT_PATHS: [&str; 5] = [
    "$XDG_CONFIG_HOME/randy.yml",
    "$HOME/.randy.yml",
    "$HOME/randy.yml",
    "$XDG_CONFIG_HOME/.randy.yml",
    "/etc/randy.yml",
];

pub fn try_get_file() -> Option<String> {
    let home = std::env::var("HOME").unwrap_or("".to_string());
    if home != "" {
        let cfg = format!("{}/.randy.yml", home);
        if std::path::Path::new(&cfg).exists() {
            return Some(cfg);
        }
    }

    let xdg = std::env::var("XDG_CONFIG_HOME");
    if xdg.is_ok() {
        let cfg = format!("{}/randy.yml", xdg.unwrap());
        if std::path::Path::new(&cfg).exists() {
            return Some(cfg);
        }
    }

    if home != "" {
        let cfg = format!("{}/.config/randy.yml", home);
        if std::path::Path::new(&cfg).exists() {
            return Some(cfg);
        }
    }

    let cfg = "/etc/randy.yml";
    if std::path::Path::new(&cfg).exists() {
        return Some(cfg.to_string());
    }

    return None;
}

fn get_file() -> String {
    let config_path = try_get_file().expect(
        r#"Could not find a randy.yml config file.
Checked in this order:
- /.randy.yml
- /randy.yml
- /.config/randy.yml
- /etc/randy.yml

Please put a randy.yml config file in one of those places.
Exmples: https://github.com/iphands/randy/tree/main/config"#,
    );
    println!("Using config file: {}", config_path);
    return match fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(_) => panic!("Unable to open/read {}", config_path),
    };
}

pub fn read_config(path: PathBuf) -> HashMap<String, serde_yml::Value> {
    let file = File::open(path).expect("Unable to open config file");
    let reader = BufReader::new(file);
    return serde_yml::from_reader(reader).expect("Unable to parse config file");
}

// pub fn get_config(yaml_str: &str) -> Vec<ConfigString> {
// let yaml = match ::load_from_str(yaml_str) {
//     Ok(y) => y,
//     Err(_) => panic!("Unable to parse config YAML"),
// };
//
// return yaml;
// }

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn should_read_file_config() {
        let wd = std::env::current_dir().unwrap();
        let path = wd.join("randy.yml");
        assert!(path.exists(), "{} does not exist", path.display());
        let config = read_config(path);
        assert!(!config.is_empty());
    }

    #[test]
    fn should_check_all_standard_paths() {}

    #[test]
    fn should_read_yml_config() {}
}
