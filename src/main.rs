mod components;
mod config;
mod ui;
mod utils;
mod widgets;

use gtk::prelude::{ApplicationExt, ApplicationExtManual};

use crate::ui::elements::build_ui;

fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";

    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}
