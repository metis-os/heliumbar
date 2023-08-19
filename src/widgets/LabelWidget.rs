use gtk::traits::ContainerExt;

use crate::ui::widget::{Align, WidgetConfig};
use crate::utils::{command, regex_matcher};
use glib::MainContext;
use gtk::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::process::{Command, Stdio};

pub fn build_label(left: &gtk::Box, center: &gtk::Box, right: &gtk::Box, config: WidgetConfig) {
    let original: String = config.format;
    let mut text = original.clone();
    if config.command.len() > 0 && config.refresh_rate == 0 {
        let out = command::run(&config.command).trim().to_string();
        if config.is_json {
            if let Some(data) = regex_matcher::format(&original, &out) {
                text = data;
            }
        }
        //if json
        else {
            text = original.replace("{}", &out);
        }
    } //if command

    let label = gtk::Label::builder().label(text).build();
    label.style_context().add_class(&config.name_of_widget);
    match config.align {
        Align::CENTER => center.add(&label),
        Align::LEFT => left.add(&label),
        Align::RIGHT => right.add(&label),
    }

    if config.refresh_rate > 0 && config.command.len() > 0 {
        update_widget(
            label,
            original,
            config.is_json,
            config.refresh_rate,
            &config.command,
        );
    }
    // println!("lenght of command{}", config.command.len());
}

pub fn update_widget(
    label: gtk::Label,
    original: String,
    is_json: bool,
    refresh_rate: i64,
    command: &str,
) {
    let child = Command::new("zsh")
        .arg("-c")
        .arg(&format!(
            "while true; do;{};sleep {};done",
            command, refresh_rate
        ))
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
        if is_json {
            if let Some(out) = regex_matcher::format(&original, &data) {
                label.set_text(&out);
            }
        } else {
            label.set_text(&original.replace("{}", &data));
        }
        glib::ControlFlow::Continue
    });
}
