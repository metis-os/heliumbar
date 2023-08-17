use gtk::traits::ContainerExt;

use crate::ui::widget::{Align, WidgetConfig};
use crate::utils::command;
use gtk::prelude::*;
pub fn build_button(left: &gtk::Box, center: &gtk::Box, right: &gtk::Box, config: WidgetConfig) {
    let mut text: String = config.text;
    if config.command.len() > 0 {
        text = command::run(&config.command);
    }

    let label = gtk::Button::builder().label(text).build();

    label.style_context().add_class(&config.name_of_widget);
    match config.align {
        Align::CENTER => center.add(&label),
        Align::LEFT => left.add(&label),
        Align::RIGHT => right.add(&label),
    }

    // println!("lenght of command{}", config.command.len());
}
