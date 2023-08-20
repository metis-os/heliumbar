mod builder;
mod config;
mod modules;
mod network;
mod utils;
mod widgets;

use gtk::prelude::{ApplicationExt, ApplicationExtManual};

use crate::builder::layer_builder::build_ui;

fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";

    let app = gtk::Application::builder().application_id(APP_ID).build();
    // println!("{:?}", std::thread::current().id().to_owned());
    app.connect_activate(build_ui);
    app.run()
} //main
