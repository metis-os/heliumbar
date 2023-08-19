// Create a widget in the menus

use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk::Orientation;
// use gtk_layer_shell::Edge;

use crate::config;
use crate::utils;
use crate::widgets::ButtonWidget;
use crate::widgets::LabelWidget;

fn build_config_else_default(
    centered: &gtk::Box,
    configs: &Result<json::JsonValue, String>,
) -> bool {
    if let Err(error) = configs {
        let label = gtk::Label::builder().label(error).margin_start(40).build();
        centered.add(&label);
        return false;
    }
    let configs = configs.as_ref().unwrap();

    if !configs.has_key("widgets") {
        let label = gtk::Label::builder()
            .label(
                "No widgets found in your config,please add some widgets to show in the status bar",
            )
            .margin_start(40)
            .build();
        centered.add(&label);
        return false;
    };
    return true;
}

fn load_css() {
    let user = std::env::var("HOME");
    if let Err(err) = user {
        println!("{}", err);
        return;
    }
    let mut path = user.unwrap();
    path.push_str(&utils::constants::CONFIG_STYLE);

    let provider = gtk::CssProvider::new();
    if let Err(err) = provider.load_from_path(&path) {
        println!("{}", err);
        return;
    }

    let screen = gtk::gdk::Screen::default();
    if let None = screen {
        return;
    }

    gtk::StyleContext::add_provider_for_screen(
        &screen.unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}

pub fn build_widgets(window: &ApplicationWindow, orientation: Orientation) {
    // let orientation = Orientation::Horizontal;
    let root = gtk::Box::new(orientation, 0);
    let left = gtk::Box::new(orientation, 0);
    let centered = gtk::Box::new(orientation, 0);
    let right = gtk::Box::new(orientation, 0);

    root.style_context().add_class("root");
    left.style_context().add_class("left");
    centered.style_context().add_class("center");
    right.style_context().add_class("right");

    root.set_widget_name("root");
    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");

    root.set_center_widget(Some(&centered));
    root.pack_end(&right, false, true, 0);
    root.add(&left);

    let configs = config::user_config::read_config();
    if build_config_else_default(&centered, &configs) {
        render_custom_widgets(left, right, centered, configs.unwrap());
    }

    window.add(&root);
    load_css();
    window.show_all();
}

pub enum Align {
    LEFT,
    CENTER,
    RIGHT,
}
pub struct WidgetConfig {
    pub format: String,
    // pub type_of_widget: String,
    pub align: Align,
    pub command: String,
    pub refresh_rate: i64,
    pub tooltip: String,
    pub name_of_widget: String,
    pub is_json: bool,
}

pub fn check_alignment(align: &String) -> Align {
    if align == "left" {
        return Align::LEFT;
    } else if align == "right" {
        return Align::RIGHT;
    } else if align == "center" {
        return Align::CENTER;
    } else {
        return Align::LEFT;
    }
}

pub fn render_custom_widgets(
    left: gtk::Box,
    right: gtk::Box,
    centered: gtk::Box,
    configs: json::JsonValue,
) {
    let widgets = configs["widgets"].entries();
    for (key, value_json) in widgets {
        let format = value_json["format"].as_str().unwrap_or("").to_string();
        let type_of_widget = value_json["type"].as_str().unwrap_or("").to_string();
        let align = check_alignment(&value_json["align"].as_str().unwrap_or("").to_string());
        let command = value_json["command"].as_str().unwrap_or("").to_string();
        let refresh_rate = value_json["refresh-rate"].as_i64().unwrap_or(0);
        let tooltip = value_json["tooltip"].as_str().unwrap_or("").to_string();
        let is_json = value_json["is_json"].as_bool().unwrap_or(false);
        let name_of_widget = key.to_string();

        let data = WidgetConfig {
            format,
            // type_of_widget,
            align,
            command,
            refresh_rate,
            is_json,
            tooltip,
            name_of_widget,
        };

        if type_of_widget == "label" {
            LabelWidget::build_label(&left, &centered, &right, data);
        } else if type_of_widget == "button" {
            ButtonWidget::build_button(&left, &centered, &right, data);
        } else {
            LabelWidget::build_label(&left, &centered, &right, data);
        }
    } //for
}
