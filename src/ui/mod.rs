use std::any::Any;
use std::collections::HashMap;

use gtk::prelude::*;
use gtk::ApplicationWindow;

use crate::config;
use crate::stubs::SPACING;

mod helpers;
mod widgets;
pub use widgets::battery::BatteryWidget;


pub fn mount_widgets(stash: &mut UiStash, layout: &gtk::Box, widgets: &[gtk::Widget]) {
    for widget in widgets.iter() {
        // let label = Some(i["text"].as_str().unwrap());
        let frame = gtk::Frame::builder().build();
        frame.style_context().add_class("frame");
        layout.append(&frame);

        let inner_box = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
        inner_box.style_context().add_class("innerbox");
        frame.set_child(Some(&inner_box));

        println!("TYPE: {:?}", widget.type_id());

        // if !i["type"].is_badvalue() {
        //     let limit = i["limit"].as_i64().unwrap_or(5);
        //     match i["type"].as_str().unwrap() {
        //         "battery" => add_batt(
        //             &inner_box,
        //             i["items"].as_vec().unwrap_or(&Vec::new()),
        //             &mut stash.batts,
        //         ),
        //         "cpus" => add_cpus(
        //             &inner_box,
        //             &mut stash.cpus,
        //             i["split"].as_bool().unwrap_or(false),
        //         ),
        //         "mem_consumers" => add_consumers("MEM", limit, &inner_box, &mut stash.top_mems),
        //         "cpu_consumers" => add_consumers("CPU", limit, &inner_box, &mut stash.top_cpus),
        //         "filesystem" => add_filesystem(
        //             &inner_box,
        //             i["items"].as_vec().unwrap_or(&Vec::new()),
        //             &mut stash.fs,
        //         ),
        //         "net" => add_net(
        //             &inner_box,
        //             i["items"].as_vec().unwrap_or(&Vec::new()),
        //             &mut stash.net,
        //         ),
        //         "system" => {
        //             for item in i["items"].as_vec().unwrap_or(&Vec::new()) {
        //                 let val = add_standard(item, &inner_box);
        //                 stash.system.insert(item.clone(), val);
        //             }
        //         }
        //         _ => (),
        //     }
        // }
    }
}

pub struct UiStash {
    batts: HashMap<String, BatteryWidget>,
    // cpus: Vec<Cpu>,
    // fs: HashMap<String, (gtk::Label, gtk::ProgressBar)>,
    // net: HashMap<String, (gtk::Label, gtk::Label)>,
    // system: HashMap<ConfigString, (gtk::Label, Option<gtk::ProgressBar>)>,
    // top_mems: Vec<TopRow>,
    // top_cpus: Vec<TopRow>,
}

pub fn build_ui(app: &gtk::Application, config: &config::Config) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Randy")
        .build();

    window.set_resizable(config.settings.resizable);
    window.set_decorated(config.settings.decoration);
    window.set_default_size(400, 800);

    // gtk4 migrations:
    // - removed set_position()
    // - removed window.set_skip_taskbar_hint(config["settings"]["skip_taskbar"].as_bool().unwrap_or(true));
    // - removed window.set_decorated(config["settings"]["decoration"].as_bool().unwrap_or(false));
    // - removed window.set_keep_below(!_is_interactive(&config["settings"]));
    // - removed window.set_accept_focus(_is_interactive(&config["settings"]));

    // TODO:
    // let screen = window.get_screen().unwrap();
    // let visual = screen.get_rgba_visual().unwrap();
    // window.set_visual(Some(&visual));

    println!("Debug {:?}", helpers::is_interactive(config));
    println!("Debug {:#?}", &config.settings);

    // ==================================
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 10);
    layout.set_css_classes(&["conainer"]);

    // TODO: rewrite -> config.settings
    // NOTE: gtk4 added childs to all components, box is not needed
    // let root_box = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    // root_box.add_css_class("container");
    // layout.append(&root_box);

    // ===================================
    let mut stash = UiStash {
        batts: HashMap::new(),
        // system: HashMap::new(),
        // cpus: Vec::new(),
        // net: HashMap::new(),
        // top_mems: Vec::new(),
        // top_cpus: Vec::new(),
        // fs: HashMap::new(),
    };

    // List Widgets - a vertical list of widgets
    let battery_widget = BatteryWidget::new();
    layout.append(&battery_widget);
    let system_widget = BatteryWidget::new();
    layout.append(&system_widget);
    println!("DEBUG: widgets: {:#?}", &layout);

    // let frame = gtk::Frame::builder().build();
    // frame.style_context().add_class("frame");
    // layout.append(&frame);
    //
    // let inner_box = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    // inner_box.style_context().add_class("innerbox");
    // frame.set_child(Some(&inner_box));

    // mount_widgets(&mut stash, &layout, &widgets);
    // ===================================
    window.set_child(Some(&layout));
    window.present();
}
