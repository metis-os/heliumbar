mod config;
mod ui;
mod utils;
mod widgets;

use std::process::id;

use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use tokio::time::Sleep;

use crate::{ui::elements::build_ui, widgets::workspace};

fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";

    let app = gtk::Application::builder().application_id(APP_ID).build();
    // println!("{:?}", std::thread::current().id().to_owned());
    app.connect_activate(build_ui);
    app.run()
} //main