use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use libc::system;

const SUPPORTED_FORMATS: [&str; 2] = ["yml", "toml"];
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

fn parse_config(path: PathBuf) -> HashMap<String, serde_yml::Value> {
    let file = File::open(path).expect("Unable to open config file");
    let reader = BufReader::new(file);
    serde_yml::from_reader(reader).expect("Unable to parse config file")
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
mod config_tests {
    use super::*;

    #[test]
    fn should_get_all_paths() {
        let paths = build_full_paths();
        assert!(!paths.is_empty());
    }

    #[test]
    fn should_return_default_config() {
        std::env::set_var(
            "XDG_CONFIG_HOME",
            std::env::current_dir().unwrap().join("config"),
        );
        let path = find_default_config();
        println!("{:?}", path);
        assert!(path.exists(), "Can't find default config, it exists?");
    }

    #[test]
    fn should_read_file_config() {
        let wd = std::env::current_dir().unwrap();
        let path = wd.join("randy.yml");
        assert!(path.exists(), "{} does not exist", path.display());
        let config = parse_config(path);
        assert!(!config.is_empty());
    }

    #[test]
    fn should_check_all_standard_paths() {}

    #[test]
    fn should_read_yml_config() {}
}
