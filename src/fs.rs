use std::path::PathBuf;

use directories::ProjectDirs;

use crate::stubs::APP_ID;

pub(crate) fn load_user_config_path() -> Option<PathBuf> {
    let app_id = APP_ID.split('.').collect::<Vec<&str>>();

    if let Some(proj_dirs) = ProjectDirs::from(app_id[0], app_id[1], app_id[2]) {
        let config_file = proj_dirs.config_dir().join("config.toml");
        if config_file.exists() {
            return Some(config_file);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_config_from_xdg_path() {
        let path = load_user_config_path();
        assert!(path.is_some());
        assert!(path.unwrap().ends_with(".config/sidp/config.toml"));
    }
}
