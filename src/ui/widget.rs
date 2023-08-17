// Create a widget in the menus

use gtk::gdk::keys::constants::comma;
use gtk::prelude::*;
use gtk::Orientation;
use gtk::{ApplicationWindow, Button};

use crate::config;
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

pub fn build_widgets(window: &ApplicationWindow) {
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

    let configs = config::user_config::read_config();
    if build_config_else_default(&centered, &configs) {
        render_custom_widgets(left, right, centered, configs.unwrap());
    }

    window.add(&root);
    window.show_all();
}

pub enum Align {
    LEFT,
    CENTER,
    RIGHT,
}
pub struct WidgetConfig {
    pub text: String,
    // pub type_of_widget: String,
    pub align: Align,
    pub command: String,
    pub refresh_rate: String,
    pub tooltip: String,
    pub name_of_widget: String,
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
        let text = value_json["text"].as_str().unwrap_or("").to_string();
        let type_of_widget = value_json["type"].as_str().unwrap_or("").to_string();
        let align = check_alignment(&value_json["align"].as_str().unwrap_or("").to_string());
        let command = value_json["command"].as_str().unwrap_or("").to_string();
        let refresh_rate = value_json["refresh-rate"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let tooltip = value_json["tooltip"].as_str().unwrap_or("").to_string();
        let name_of_widget = key.to_string();

        let data = WidgetConfig {
            text,
            // type_of_widget,
            align,
            command,
            refresh_rate,
            tooltip,
            name_of_widget,
        };

        if type_of_widget == "label" {
            LabelWidget::build_label(&left, &centered, &right, data)
        } //if case
    } //for
}
