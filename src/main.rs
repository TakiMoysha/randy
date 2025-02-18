// use std::time::Instant;
// use std::{collections::HashMap, rc::Rc};

// struct Cpu {
//     mhz: gtk4::Label,
//     progress: gtk4::ProgressBar,
//     pct_label: gtk4::Label,
// }

// struct TopRow {
//     name: gtk4::Label,
//     pid: gtk4::Label,
//     pct: gtk4::Label,
// }

// struct Battery {
//     lbl_pct: gtk::Label,
//     lbl_status: gtk::Label,
//     str_battery: String,
//     str_plugged: String,
//     str_pct_template: String,
// }

// struct NetDevCache {
//     last_bytes: u64,
//     last_instant: Instant,
// }

use clap::{command, Parser};
use gio::prelude::ActionMapExtManual;
use gtk4::{
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt},
};
use std::{path::PathBuf, rc::Rc};

mod config;
mod stubs;
mod ui;

use config::{find_default_config, load_config};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,
}

const APP_ID: &str = "org.ahands.randy";

mod app {
    use crate::config;

    // glib::wrapper! {
    //     struct RandyApp(ObjectSubclass<impl::RandyAppImpl>)
    //         @extends gio::Application, gtk::Application,
    //         @implements gio::ApplicationGroup, gio::ActionMap, gtk::Root;
    // }

    // impl RandyApp {}

    // impl RandyApp {
    //     pub fn new(config: config::Config) -> RandyApp {
    //         RandyApp {
    //             config: Box::new(config),
    //         }
    //     }
    // }
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let config = Rc::new(match args.config {
        Some(config_path) => load_config(PathBuf::from(config_path)),
        None => load_config(find_default_config()),
    });
    let clone_config = config.clone();

    let app = gtk4::Application::builder().application_id(APP_ID).build();
    app.connect_startup(move |app| on_startup(app, &clone_config));
    app.connect_activate(move |app| ui::build_ui(app, &config));
    app.run_with_args(&Vec::<String>::new())
}

fn on_startup(app: &gtk4::Application, config: &config::Config) {
    let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| println!("About msg"))
        .build();

    app.add_action_entries([about]);

    // Loading CSS
    ui::style::load_css(config);
}
