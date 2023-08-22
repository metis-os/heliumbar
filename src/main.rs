mod builder;
mod config;
mod modules;
mod network;
mod utils;
mod widgets;

use gtk::prelude::{ApplicationExt, ApplicationExtManual};

use crate::builder::layer_builder::build_ui;
use std::{path::Path, sync::mpsc::channel, time::Duration};

fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";

    let app = gtk::Application::builder().application_id(APP_ID).build();
    // println!("{:?}", std::thread::current().id().to_owned());
    app.connect_activate(build_ui);
    // let (sender, receiver) = glib::MainContext::channel::<String>(glib::Priority::DEFAULT);
    // let (sender, receiver) = channel();
    std::thread::spawn(move || {
        watcher_file();
    });
    app.run()
} //main

fn watcher_file() {
    let path = "/sys/class/power_supply/BAT0/status";
} //watcher
