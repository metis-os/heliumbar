mod utils;
use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;
use gtk::{glib, Application};

fn main() -> glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        utils::ui::build_ui(app);
    });
    app.run()
}
