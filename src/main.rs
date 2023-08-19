use gtk::prelude::{ApplicationExt, ApplicationExtManual};
mod config;
mod modules;
mod ui;
mod utils;
mod widgets;

#[tokio::main]
async fn main() -> gtk::glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(crate::ui::elements::build_ui);
    app.run()
}
