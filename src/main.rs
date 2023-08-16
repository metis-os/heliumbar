mod components;
mod ui;
mod config;

use gtk::prelude::{ApplicationExt, ApplicationExtManual};

fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";

    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        ui::elements::build_ui(app);
    });

    config::user_config::read_config();
    app.run()

}
