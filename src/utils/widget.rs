// Create a widget in the menus

use gtk::prelude::*;
use gtk::Orientation;
use gtk::{ApplicationWindow, Button};

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

    let info = crate::utils::informations::Info::new().unwrap();

    let button_left = Button::builder().label(&format!("{}", info.os_release)).build();
    let button_center = Button::builder().label(&format!("{}", info.uptime)).build();
    let button_right = Button::builder().label(&format!("{}", info.kernel)).build();

    left.add(&button_left);
    centered.add(&button_center);
    right.add(&button_right);

    window.add(&root);
    window.show_all();
}
