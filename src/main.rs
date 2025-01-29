mod stubs;

#[macro_use]
extern crate lazy_static;
extern crate gio;
extern crate gtk;

// #[macro_use]
// mod macros;
// mod deets;
// mod file_utils;
use gio::prelude::*;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

const SPACING: i32 = 3;

struct Cpu {
    mhz: gtk::Label,
    progress: gtk::ProgressBar,
    pct_label: gtk::Label,
}

struct TopRow {
    name: gtk::Label,
    pid: gtk::Label,
    pct: gtk::Label,
}

use stubs::ConfigString;

struct UiStash {
    batts: HashMap<String, Battery>,
    cpus: Vec<Cpu>,
    fs: HashMap<String, (gtk::Label, gtk::ProgressBar)>,
    net: HashMap<String, (gtk::Label, gtk::Label)>,
    system: HashMap<ConfigString, (gtk::Label, Option<gtk::ProgressBar>)>,
    top_mems: Vec<TopRow>,
    top_cpus: Vec<TopRow>,
}

struct Battery {
    lbl_pct: gtk::Label,
    lbl_status: gtk::Label,
    str_battery: String,
    str_plugged: String,
    str_pct_template: String,
}

struct NetDevCache {
    last_bytes: u64,
    last_instant: Instant,
}

lazy_static! {
    static ref FRAME_COUNT: Mutex<u64> = Mutex::new(0);
}

// mod css;

// mod _helpers;
use clap::{command, Parser};
use gtk::{
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use std::path::PathBuf;

mod config;
mod ui;

use config::{find_default_config, load_config};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,
}

const APP_ID: &str = "org.ahands.randy";

fn main() -> ExitCode {
    let args = Args::parse();

    let config = match args.config {
        Some(v) => load_config(PathBuf::from(v)),
        None => load_config(find_default_config()),
    };

    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .flags(Default::default())
        .build();

    app.connect_activate(ui::build_ui);
    // application.connect_activate(|app| {
    //     build_ui(app, config::get_config(&args.config));
    // });

    app.run()
}
