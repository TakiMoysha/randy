use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

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

fn find_first_default_config() -> Option<PathBuf> {
    let paths = build_full_paths();
    paths.iter().find(|f| f.exists()).cloned()
}

pub fn load_default_config() {
    let path = find_first_default_config().expect("Unable to find default config");
    load_config(path)
}

// ###
fn try_get_file() -> Option<String> {
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
    serde_yml::from_reader(reader).expect("Unable to parse config file")
}

// pub fn get_config(yaml_str: &str) -> Vec<ConfigString> {
// let yaml = match ::load_from_str(yaml_str) {
//     Ok(y) => y,
//     Err(_) => panic!("Unable to parse config YAML"),
// };
//
// return yaml;
// }

pub fn load_config(path: PathBuf) {}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn should_get_all_paths() {
        let paths = build_full_paths();
        println!("!!!!!{:?}", paths);
        assert!(!paths.is_empty());
    }

    #[test]
    fn should_return_default_config() {
        std::env::set_var(
            "XDG_CONFIG_HOME",
            std::env::current_dir().unwrap().join("config"),
        );
        let path = find_first_default_config();
        assert!(path.is_some(), "Can't find default config, it exists?");
    }

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
