use gtk::prelude::{ApplicationExt, ApplicationExtManual};
mod builder; 
mod config;
mod modules;
mod network;
mod utils;
mod widgets;

fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(crate::builder::layer_builder::build_ui);
    modules::power::power_info().unwrap_or_default();
    app.run()
}


