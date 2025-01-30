use std::collections::HashMap;
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

// lazy_static! {
//     static ref FRAME_COUNT: Mutex<u64> = Mutex::new(0);
// }

use clap::{command, Parser};
use gtk::{
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use std::path::PathBuf;

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

fn main() -> ExitCode {
    let args = Cli::parse();

    let config = match args.config {
        Some(config_path) => load_config(PathBuf::from(config_path)),
        None => load_config(find_default_config()),
    };

    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        ui::build_ui(app, &config);
    });

    app.run_with_args(&Vec::<String>::new())
}
