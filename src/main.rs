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

    app.run()
} //main

fn watcher_file() {
    let path = "/sys/class/backlight/amdgpu_bl1/brightness";
    let mut inotify = inotify::Inotify::init().unwrap();
    let watch = inotify.watches().add(path, inotify::WatchMask::MODIFY);
    if let Err(err) = watch {
        println!("{}", err);
        return;
    }
    // let watch = watch.unwrap();
    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify.read_events_blocking(&mut buffer);
        if let Err(err) = events {
            println!("{}", err);
        }
        //for loop
    }
} //watcher
