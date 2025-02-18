use std::collections::HashMap;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

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

#[allow(dead_code)]
pub fn test_build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Randy")
        .default_width(200)
        .default_height(200)
        .build();

    window.present();
}

use crate::{config, stubs};

pub mod style;

mod widgets {
    // !TODO:
}

pub fn init_ui(stash: &mut UiStash, layout: &gtk4::Box, widgets: &Vec<gtk4::Widget>) {
    for item in widgets.iter() {
        let frame = gtk4::Frame::builder().build();
        layout.append(&frame);

        // match item {
        //     _ => todo!(),
        // }
    }
}

pub struct UiStash {
    // batts: HashMap<String, Battery>,
    // cpus: Vec<Cpu>,
    fs: HashMap<String, (gtk4::Label, gtk4::ProgressBar)>,
    net: HashMap<String, (gtk4::Label, gtk4::Label)>,
    // system: HashMap<ConfigString, (gtk::Label, Option<gtk::ProgressBar>)>,
    // top_mems: Vec<TopRow>,
    // top_cpus: Vec<TopRow>,
}

pub fn build_ui(app: &gtk4::Application, config: &config::Config) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Randy")
        .build();

    window.set_decorated(config.settings.decoration);
    window.set_resizable(config.settings.resizable);
    window.set_default_size(400, 800);

    // gtk4 migrations:
    // - removed set_position()
    // - removed window.set_skip_taskbar_hint(config["settings"]["skip_taskbar"].as_bool().unwrap_or(true));
    // - removed window.set_decorated(config["settings"]["decoration"].as_bool().unwrap_or(false));
    // - removed window.set_keep_below(!_is_interactive(&config["settings"]));
    // - removed window.set_accept_focus(_is_interactive(&config["settings"]));

    let cp = window.clipboard();

    // !NOTE: WIP
    // let screen = window.get_screen().unwrap();
    // let visual = screen.get_rgba_visual().unwrap();
    // window.set_visual(Some(&visual));

    println!("Debug {:?}", helpers::is_interactive(config));
    println!("Debug {:#?}", &config.settings);

    // ==================================
    let layout = gtk4::Box::new(gtk4::Orientation::Vertical, 10);

    layout.set_css_classes(&["conainer"]);

    // TODO: rewrite -> config.settings
    let SPACING = 10;
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, SPACING);
    vbox.add_css_class("container");

    let from_entry = gtk4::Entry::new();
    from_entry.set_text("Hello World");

    vbox.append(&from_entry);
    layout.append(&vbox);

    // ===================================
    let mut stash = UiStash {
        // batts: HashMap::new(),
        // system: HashMap::new(),
        // cpus: Vec::new(),
        net: HashMap::new(),
        // top_mems: Vec::new(),
        // top_cpus: Vec::new(),
        fs: HashMap::new(),
    };

    let widgets = vec![];
    init_ui(&mut stash, &vbox, &widgets);
    window.set_child(Some(&layout));
    // update_ui(&config.settings, stash);

    // ===================================
    window.present();
}
