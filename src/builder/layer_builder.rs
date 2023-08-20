// HELIUMBAR ui / ux

use crate::builder::widgets_builder::build_widgets;
use crate::config;
use gtk::gdk::*;
use gtk::prelude::*;
use gtk::Orientation;
use gtk::{Application, ApplicationWindow};
use gtk_layer_shell;
use gtk_layer_shell::Edge;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    gtk_layer_shell::init_for_window(&window);

    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);

    gtk_layer_shell::auto_exclusive_zone_enable(&window);
    window.set_app_paintable(true);

    let config = read_window_config();

    let mut orientation = Orientation::Horizontal;

    if let Some(data) = config {
        match &data.position {
            Edge::Left => orientation = Orientation::Vertical,
            Edge::Right => orientation = Orientation::Vertical,
            _ => (),
        }
        align_layer(&window, &data.position);
        window.connect_draw(move |win, context| draw(&win, context, &data));
    } else {
        align_layer(&window, &Edge::Left);
    }

    let display = Display::default().expect("Error happening");
    let monitor = display.monitor(0).expect("Error getting monitor");
    gtk_layer_shell::set_monitor(&window, &monitor);

    window.connect_destroy(|_| gtk::main_quit());
    build_widgets(&window, orientation);
}

pub fn align_layer(window: &ApplicationWindow, align: &Edge) {
    gtk_layer_shell::set_anchor(window, Edge::Left, true);
    gtk_layer_shell::set_anchor(window, Edge::Right, true);
    gtk_layer_shell::set_anchor(window, Edge::Top, true);
    gtk_layer_shell::set_anchor(window, Edge::Bottom, true);
    match align {
        Edge::Top => {
            gtk_layer_shell::set_anchor(window, Edge::Bottom, false);
        }
        Edge::Bottom => {
            gtk_layer_shell::set_anchor(window, Edge::Top, false);
        }
        Edge::Left => {
            gtk_layer_shell::set_anchor(window, Edge::Right, false);
        }
        Edge::Right => {
            gtk_layer_shell::set_anchor(window, Edge::Left, false);
        }
        _ => {
            gtk_layer_shell::set_anchor(window, Edge::Bottom, false);
        }
    } //match
}
pub struct LayerConfig {
    alpha: f64,
    color: (f64, f64, f64),
    position: Edge,
}

pub fn draw(_: &ApplicationWindow, context: &cairo::Context, config: &LayerConfig) -> Inhibit {
    context.set_source_rgba(config.color.0, config.color.1, config.color.2, config.alpha);
    context.set_operator(cairo::Operator::Screen);
    context.paint().expect("ERROR drawing colors");
    Inhibit(false)
}

pub fn extract_color(color: &str) -> Option<(f64, f64, f64)> {
    if color.len() != 7 || !color.starts_with("#") {
        return None;
    }
    let r_color = u8::from_str_radix(&color[1..3], 16).ok()? as f64 / 255 as f64;
    let g_color = u8::from_str_radix(&color[3..5], 16).ok()? as f64 / 255 as f64;
    let b_color = u8::from_str_radix(&color[5..7], 16).ok()? as f64 / 255 as f64;
    Some((r_color, g_color, b_color))
}

pub fn read_window_config() -> Option<LayerConfig> {
    let config = config::user_config::read_config();
    if let Err(_) = config {
        return None;
    }
    let config = config.unwrap();
    let alpha: f64 = config["alpha"].as_f64().unwrap_or_default();
    let color: String = config["background"]
        .as_str()
        .unwrap_or("#000000")
        .to_string();
    let position: String = config["align"].as_str().unwrap_or("top").to_string();
    let pos: Edge;
    if position == "top" {
        pos = Edge::Top;
    } else if position == "bottom" {
        pos = Edge::Bottom;
    } else if position == "left" {
        pos = Edge::Left;
    } else if position == "right" {
        pos = Edge::Right;
    } else {
        pos = Edge::Top;
    }

    let mut sep_col = extract_color(&color);
    if let None = sep_col {
        sep_col = Some((0.0, 0.0, 0.0));
    }
    Some(LayerConfig {
        alpha,
        color: sep_col.unwrap(),
        position: pos,
    })
}
