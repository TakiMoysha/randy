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
mod fs;
mod parser;
mod stubs;
mod ui;

use config::Config;
use stubs::APP_ID;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,
}

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
        Some(config_path) => {
            println!("Using config file: {}", config_path);
            Config::load_config(Some(PathBuf::from(config_path)))
        }
        None => Config::load_config(None),
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
