use super::Config;

pub(crate) fn parser_to_config(config_string: &str) -> Config {
    toml::from_str(config_string).expect("Bad config file, can't parse.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_config_from_toml_file() {
        let config_string: &str = "[settings] \
                timeout = 1 \
                [[ui]] \
                text = 'SYSTEM' \
                block = 'system' \
                [[ui.item]] \
                func = 'hostname' \
                text = 'Hostname:' \
                [[ui.item]] \
                block = 'kernel' \
                text = 'Kernel:' ";
        let config = parser_to_config(config_string);
        assert!(config.ui.unwrap().len() == 1);
    }
}
