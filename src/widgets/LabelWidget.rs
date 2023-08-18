use gtk::traits::ContainerExt;

use crate::ui::widget::{Align, WidgetConfig};
use crate::utils::command;
use glib::MainContext;
use gtk::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::process::{Command, Stdio};

pub fn build_label(left: &gtk::Box, center: &gtk::Box, right: &gtk::Box, config: WidgetConfig) {
    let mut text: String = config.text;
    if config.command.len() > 0 {
        text = command::run(&config.command).trim().to_string();
    }

    let label = gtk::Label::builder().label(text).build();
    label.style_context().add_class(&config.name_of_widget);
    match config.align {
        Align::CENTER => center.add(&label),
        Align::LEFT => left.add(&label),
        Align::RIGHT => right.add(&label),
    }
    if config.refresh_rate > 0 {
        update_widget(label.clone());
    }
    // println!("lenght of command{}", config.command.len());
}

pub fn update_widget(label: gtk::Label) {
    let child = Command::new("zsh")
        .arg("-c")
        .arg("while true; do;echo $(date +\"%Y-%m-%d %H:%M:%S\");sleep 1;done")
        .stdout(Stdio::piped())
        .spawn();

    if let Err(error) = child {
        println!("{}", error);
        return;
    }
    let stdout = child.unwrap().stdout.take().unwrap();
    let (sender, receiver) = MainContext::channel::<String>(glib::Priority::DEFAULT);

    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(data) = line {
                sender.send(data.to_string()).unwrap();
            } //if
        }
    });

    receiver.attach(None, move |data| {
        label.set_text(&data);
        glib::ControlFlow::Continue
    });
}
