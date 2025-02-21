// use std::time::Instant;
// use std::{collections::HashMap, rc::Rc};

// struct Cpu {
//     mhz: gtk::Label,
//     progress: gtk::ProgressBar,
//     pct_label: gtk::Label,
// }

// struct TopRow {
//     name: gtk::Label,
//     pid: gtk::Label,
//     pct: gtk::Label,
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

use std::{path::PathBuf, rc::Rc};

use clap::{command, Parser};

use gtk::gio::prelude::*;
use gtk::glib::ExitCode;
use gtk::prelude::*;

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

    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_startup(move |app| on_startup(app, &clone_config));
    app.connect_activate(move |app| ui::build_ui(app, &config));
    app.run_with_args(&Vec::<String>::new())
}

fn on_startup(app: &gtk::Application, config: &config::Config) {
    // ======================================
    // let about = gtk::gio::ActionEntry::builder("about")
    //     .activate(|_, _, _| println!("About msg"))
    //     .build();
    // app.add_action_entries([about]);

    // ======================================
    ui::load_css(config);
}
