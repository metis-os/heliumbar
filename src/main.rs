mod ui;

use gtk::gdk::*;
use gtk::prelude::*;
use gtk::Orientation;
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk_layer_shell;
use gtk_layer_shell::Edge;

fn main() -> glib::ExitCode {
    const APP_ID: &str = "com.heliumbar";
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);

    gtk_layer_shell::init_for_window(&window);

    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);

    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    gtk_layer_shell::set_anchor(&window, Edge::Top, true);
    gtk_layer_shell::set_anchor(&window, Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, Edge::Right, true);
    gtk_layer_shell::set_anchor(&window, Edge::Bottom, false);

    let display = Display::default().expect("Error happening");
    let monitor = display.monitor(0).expect("Error getting monitor");
    gtk_layer_shell::set_monitor(&window, &monitor);

    window.set_app_paintable(true);
    window.connect_draw(draw);

    build_widgets(&window);
    // window.present();
}

fn build_widgets(window: &ApplicationWindow) {
    let root = gtk::Box::new(Orientation::Horizontal, 0);
    let left = gtk::Box::new(Orientation::Horizontal, 0);
    let centered = gtk::Box::new(Orientation::Horizontal, 0);
    let right = gtk::Box::new(Orientation::Horizontal, 0);

    root.set_widget_name("root");
    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");
    root.set_center_widget(Some(&centered));

    root.pack_end(&right, false, true, 0);

    root.add(&left);

    let button_left = Button::builder().label("Left").build();
    let button_center = Button::builder().label(" Center").build();
    let button_right = Button::builder().label("Right").build();

    left.add(&button_left);
    centered.add(&button_center);
    right.add(&button_right);

    window.add(&root);
    window.show_all();
}

fn draw(_: &ApplicationWindow, context: &cairo::Context) -> Inhibit {
    context.set_source_rgba(0.1, 0.1, 0.1, 0.2);
    context.set_operator(cairo::Operator::Screen);
    context.paint().expect("ERROR drawing colors");
    Inhibit(false)
}
