use gtk::gdk::Screen;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

// mod _helpers;

use crate::config;

pub mod css {
    use crate::config;

    pub fn load_css(config: &config::Config) {
        let conf_preset = &config.settings;
        println!("{:?}", conf_preset);
        let provider = gtk::CssProvider::new();
        // provider.load_from_file();
    }
}

mod _maybe {
    #[derive(Default)]
    pub struct UiBuilder {
        // config: config::Config,
    }

    impl UiBuilder {}
}

pub fn build_ui(app: &gtk::Application, config: &config::Config) {
    let window = gtk::ApplicationWindow::builder().application(app);

    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .default_width(200)
    //     .default_height(200)
    //     .title("Randy")
    //     .build();

    window.build().present();
}

//     let screen = window.get_screen().unwrap();
//
//     let css: &str = &get_css(&config["settings"], screen.is_composited());
//     let provider = gtk::CssProvider::new();
//     provider
//         .load_from_data(css.as_bytes())
//         .expect("Failed to load CSS");
//     gtk::StyleContext::add_provider_for_screen(
//         &screen,
//         &provider,
//         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
//     );
//
//     window.set_title("Randy");
//     window.set_decorated(config["settings"]["decoration"].as_bool().unwrap_or(false));
//     window.set_resizable(config["settings"]["resizable"].as_bool().unwrap_or(false));
//     window.set_position(gtk::WindowPosition::Center);
//     window.set_default_size(375, -1);
//     window.set_skip_taskbar_hint(config["settings"]["skip_taskbar"].as_bool().unwrap_or(true));
//     window.set_keep_below(!_is_interactive(&config["settings"]));
//     window.set_accept_focus(_is_interactive(&config["settings"]));
//     // println!("Debug {:?}", _is_interactive(&config["settings"]));
//     // println!("Debug {:?}", &config["settings"]);
//
//     window.realize();
//
//     let screen = window.get_screen().unwrap();
//     let visual = screen.get_rgba_visual().unwrap();
//     window.set_visual(Some(&visual));
//
//     if !config["settings"]["xpos"].is_badvalue() && !config["settings"]["ypos"].is_badvalue() {
//         window.move_(
//             config["settings"]["xpos"].as_i64().unwrap() as i32,
//             config["settings"]["ypos"].as_i64().unwrap() as i32,
//         );
//     }
//
//     let vbox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
//     vbox.get_style_context().add_class("container");
//
//     let mut stash = UiStash {
//         batts: HashMap::new(),
//         system: HashMap::new(),
//         cpus: Vec::new(),
//         net: HashMap::new(),
//         top_mems: Vec::new(),
//         top_cpus: Vec::new(),
//         fs: HashMap::new(),
//     };
//
//     init_ui(&mut stash, &vbox, &config["ui"]);
//     window.add(&vbox);
//     window.show_all();
//     update_ui(&config["settings"], stash);
// }
