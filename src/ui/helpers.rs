use std::any::{Any, TypeId};

use crate::config;

pub fn is_interactive(config: &config::Config) -> bool {
    config.settings.decoration || config.settings.resizable
}

pub fn is_string(obj: &dyn Any) -> bool {
    TypeId::of::<String>() == obj.type_id()
}

