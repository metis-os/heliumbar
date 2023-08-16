// HELIUMBAR ui / ux

use crate::ui::widget::build_widgets;
use gtk::gdk::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk_layer_shell;
use gtk_layer_shell::Edge;

pub fn build_ui(app: &Application) {
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
}

pub fn draw(_: &ApplicationWindow, context: &cairo::Context) -> Inhibit {
    context.set_source_rgba(0.1, 0.1, 0.1, 0.2);
    context.set_operator(cairo::Operator::Screen);
    context.paint().expect("ERROR drawing colors");
    Inhibit(false)
}
