use std::fs::read_to_string;
use std::path::PathBuf;

use crate::config;
use gtk::gdk;
use gtk::glib::HasParamSpec;
use gtk::glib::object::ObjectExt;
use gtk::glib::value::ToValue;

fn get_css_by_settings(config: &config::Config) -> String {
    let base_opacity = format!("{:1.2}", &config.settings.base_opacity);
    let bar_height = format!("{}", &config.settings.bar_height);
    let color_text = format!("{}", &config.settings.color_text);
    let color_bg = format!("{}", &config.settings.color_bg);
    let color_borders = format!("{}", &config.settings.color_borders);
    let color_bar = format!("{}", &config.settings.color_bar);
    let color_bar_med = format!("{}", &config.settings.color_bar_med);
    let color_bar_high = format!("{}", &config.settings.color_bar_high);
    let color_label = format!("{}", &config.settings.color_label);
    let color_trough = format!("{}", &config.settings.color_trough);
    let font_family = format!("{}", &config.settings.font_family);
    let font_size = format!("{}", &config.settings.font_size);

    format!(
        ":root {{
        --base_opacity: {base_opacity};

        --bar_height: {bar_height};

        --color_text: {color_text};
        --color_bg: {color_bg};
        --color_borders: {color_borders};
        --color_bar: {color_bar};
        --color_bar_med: {color_bar_med};
        --color_bar_high: {color_bar_high};
        --color_label: {color_label};
        --color_trough: {color_trough};

        --font_family: {font_family};
        --font_size_top: {font_size};
        --font_size: {font_size};
        }}"
    )
}
pub fn load_css(config: &config::Config) {
    let provider = gtk::CssProvider::new();
    let css_from_settings = get_css_by_settings(config);
    let mut style = include_str!("style.css").to_string();
    style.push_str(&css_from_settings);
    style.shrink_to_fit();

    // let mut style = get_css_by_settings(config);
    // let default_style = include_str!("style.css").to_string();
    // style.push_str(&default_style);

    if let Some(display) = gdk::Display::default() {
        provider.load_from_data(&style);
        dbg!(provider.to_string());
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

#[cfg(test)]
mod tests {}
