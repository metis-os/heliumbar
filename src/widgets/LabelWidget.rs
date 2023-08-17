use gtk::traits::ContainerExt;

use crate::ui::widget::{Align, WidgetConfig};
use crate::utils::command;

pub fn build_label(left: &gtk::Box, center: &gtk::Box, right: &gtk::Box, config: WidgetConfig) {
    let mut text: String = config.text;
    if config.command.len() > 0 {
        text = command::run(&config.command);
    }

    let label = gtk::Label::builder().label(text).build();
    match config.align {
        Align::CENTER => center.add(&label),
        Align::LEFT => left.add(&label),
        Align::RIGHT => right.add(&label),
    }

    // println!("lenght of command{}", config.command.len());
}
