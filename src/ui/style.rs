use std::fs::read_to_string;

use crate::config;
use gtk::gdk::{self, prelude::DisplayExt};

pub fn get_hydrated_css(config: &config::Config, composited: bool) -> String {
    println!("[DEBUG] is composited: {:?}", composited);
    // !TODO: swap to config.css_path
    let path = "./resources/app.css";
    let css = read_to_string(path).expect("Can't find style file.");
    let base_opacity = format!("{:1.4}", &config.settings.base_opacity);

    css.replace("{ bar_height }", &config.settings.bar_height)
        .replace("{ base_opacity }", &base_opacity)
        .replace("{ color }", &config.settings.color_text)
        .replace("{ color_background }", &config.settings.color_bg)
        .replace("{ color_borders }", &config.settings.color_borders)
        .replace("{ color_bar }", &config.settings.color_bar)
        .replace("{ color_bar_med }", &config.settings.color_bar_med)
        .replace("{ color_bar_high }", &config.settings.bar_height)
        .replace("{ color_label }", &config.settings.color_label)
        .replace("{ color_trough }", &config.settings.color_trough)
        .replace("{ font_family }", &config.settings.font_family)
        .replace("{ font_size_top }", &config.settings.font_size)
        .replace("{ font_size }", &config.settings.font_size)
}

pub fn load_css(config: &config::Config) {
    let provider = gtk::CssProvider::new();

    if let Some(display) = gdk::Display::default() {
        let css = get_hydrated_css(config, display.is_composited());
        provider.load_from_data(&css);
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        println!("loaded css: {:#?}", ());
    }
}

#[cfg(test)]
mod tests {}
