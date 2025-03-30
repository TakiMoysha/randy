use std::path::PathBuf;

use directories::ProjectDirs;

use crate::stubs::APP_ID;

fn get_project_dir() -> Option<ProjectDirs> {
    let app_id = APP_ID.split('.').collect::<Vec<&str>>();
    if let Some(proj_dirs) = ProjectDirs::from(app_id[0], app_id[1], app_id[2]) {
        return Some(proj_dirs);
    }
    None
}

pub(crate) fn load_user_config_path() -> Option<PathBuf> {
    match get_project_dir() {
        Some(proj_dirs) => {
            let config_file = proj_dirs.config_dir().join("config.toml");
            if !config_file.exists() {
                return None;
            }
            Some(config_file)
        }
        None => None,
    }
}

pub(crate) fn load_user_style_file() -> Option<PathBuf> {
    match get_project_dir() {
        Some(proj_dirs) => {
            let style_file = proj_dirs.config_dir().join("style.toml");
            if !style_file.exists() {
                return None;
            }
            Some(style_file)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_config_from_xdg_path() {
        let path = load_user_config_path();
        assert!(path.is_some());
        assert!(path.unwrap().ends_with(".config/smp-wayland/config.toml"));
    }
}
