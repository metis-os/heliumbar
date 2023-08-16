// Create a widget in the menus

use gtk::prelude::*;
use gtk::Orientation;
use gtk::{ApplicationWindow, Button};

use crate::config;

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

    let info = crate::components::informations::Info::new().unwrap();

    // let button_left = Button::builder()
    //     .label(&format!("{}", info.os_release))
    //     .build();
    // let button_center = Button::builder().label(&format!("{}", info.uptime)).build();
    // let button_right = Button::builder().label(&format!("{}", info.kernel)).build();

    // left.add(&button_left);
    // centered.add(&button_center);
    // right.add(&button_right);

    window.add(&root);
    render_config(left, right, centered);
    window.show_all();
}

pub fn render_config(left: gtk::Box, right: gtk::Box, centered: gtk::Box) {
    let configs = config::user_config::read_config().unwrap();
    if !configs.has_key("widgets") {
        return;
    };

    let widgets = configs["widgets"].entries();
    for (key, value_json) in widgets {
        let text = value_json["text"].as_str().unwrap();
        let type_of_widget = value_json["type"].as_str().unwrap();
        let align = value_json["align"].as_str().unwrap();
        let _command = value_json["command"].as_str().unwrap();
        let _refresh = value_json["refresh-rate"].as_str().unwrap();
        let _tooltip = value_json["tooltip"].as_str().unwrap();
        let name_of_widget = key;

        if type_of_widget == "button" {
            let button = gtk::Button::builder()
                .label(text)
                .name(name_of_widget)
                .build();

            if align == "center" {
                centered.add(&button);
            } else if align == "right" {
                right.add(&button);
            } else {
                left.add(&button);
            } //esle
        } //if button
    } //for
}
