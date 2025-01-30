use gio::prelude::*;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

// mod _helpers;

mod _maybe {
    #[derive(Default)]
    pub struct UiBuilder {
        // config: config::Config,
    }

    impl UiBuilder {}
}

mod helpers {
    use crate::config;

    pub fn is_interactive(config: &config::Config) -> bool {
        config.settings.decoration || config.settings.resizable
    }
}

pub fn test_build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Randy")
        .default_width(200)
        .default_height(200)
        .build();

    window.present();
}

use crate::config;

pub mod css {
    use crate::config;

    pub fn load_css(config: &config::Config) {
        let conf_preset = &config.settings;
        let provider = gtk4::CssProvider::new();

        // provider.load_from_file();
        //
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
    }
}

pub fn build_ui(app: &gtk4::Application, config: &config::Config) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Randy")
        .build();

    window.set_decorated(config.settings.decoration);
    window.set_resizable(config.settings.resizable);
    window.set_default_size(375, -1);

    println!("Debug {:?}", helpers::is_interactive(&config));
    println!("Debug {:#?}", &config.settings);

    // window.realize();
    // ===================================

    // ===================================
    window.present();
}

// ############## DEPRECATED

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
