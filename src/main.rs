mod builder;
mod config;
mod modules;
mod network;
mod utils;
mod widgets;

use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use utils::command::run;

use crate::builder::layer_builder::build_ui;
use stray::{
    message::{
        menu::{MenuType, TrayMenu},
        tray::{IconPixmap, StatusNotifierItem},
        NotifierItemCommand,
    },
    NotifierItemMessage, StatusNotifierWatcher,
};
fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";

    let app = gtk::Application::builder().application_id(APP_ID).build();
    // println!("{:?}", std::thread::current().id().to_owned());
    app.connect_activate(build_ui);
    // tray();
    app.run()
} //main

fn tray() {
    let (_sender, receiver) = tokio::sync::mpsc::channel(50);

    std::thread::spawn(move || {
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

        tokio_runtime.block_on(async {
            let watcher = StatusNotifierWatcher::new(receiver).await.unwrap();

            let mut notifier_host = watcher.create_notifier_host("Hybrid").await;
            if let Err(err) = notifier_host {
                println!("Error::{}", err);
                return;
            }
            let mut notifier_host = notifier_host.unwrap();
            while let Ok(msg) = notifier_host.recv().await {
                match msg {
                    NotifierItemMessage::Update {
                        address,
                        item,
                        menu,
                    } => {
                        println!("update:{}{:?}", address, (*item).icon_name);
                    } //on update,
                    NotifierItemMessage::Remove { address } => {
                        println!("Removed:{}", address);
                    } //remove
                } //match msg
            } //while
        }) //runtime async fun
    });
}

////////////////////////
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