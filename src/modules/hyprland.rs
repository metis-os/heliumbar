use std::collections::HashMap;

use gtk::traits::ContainerExt;

use crate::builder::widgets_builder::{Align, WidgetConfig};
use crate::network::hyprland_socket::listen;
use crate::utils::{command, regex_matcher};
use glib::MainContext;
use gtk::prelude::*;

pub fn build_label(left: &gtk::Box, center: &gtk::Box, right: &gtk::Box, config: WidgetConfig) {
    let original: String = config.format;
    let mut text = original.clone();
    text = text
        .replace("{workspace}", "{workspace.id}")
        .replace("{activewindow}", "{title}");
    let out = command::run(&"hyprctl activewindow -j".to_string())
        .trim()
        .to_string();
    if let Some(data) = regex_matcher::format(&text, &out) {
        text = data;
    }
    //if json
    // println!("{}", text);
    let label = gtk::Label::builder().label(text).build();
    label.style_context().add_class(&config.name_of_widget);
    match config.align {
        Align::CENTER => center.add(&label),
        Align::LEFT => left.add(&label),
        Align::RIGHT => right.add(&label),
    }

    update_widget(label, original);
} //build_label

pub fn update_widget(label: gtk::Label, original: String) {
    let (sender, receiver) = MainContext::channel::<(String, String)>(glib::Priority::DEFAULT);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build();

    if let Err(err) = rt {
        println!("{}", err);
        return;
    }
    let params = get_params(&original);
    if params.len() == 0 {
        return;
    }
    std::thread::spawn(move || rt.unwrap().block_on(listen(sender)));

    //listen for the socket
    hyprland_signal_receiver(receiver, params, original, label);
}

fn hyprland_signal_receiver(
    receiver: glib::Receiver<(String, String)>,
    mut params: HashMap<String, String>,
    original: String,
    label: gtk::Label,
) {
    let mut format_text: String = String::new();
    receiver.attach(None, move |(name, value)| {
        // println!("{}", name);

        if params.contains_key(&name) {
            params.insert(name.trim().to_string(), value.trim().to_string());
            format_text = original.clone();
            for (key, value) in params.clone().into_iter() {
                format_text = format_text.replace(&format!("{{{}}}", key), &value);
            }
            label.set_text(&format_text.trim());
        }
        glib::ControlFlow::Continue
    });
}

fn get_params(string: &String) -> HashMap<String, String> {
    let mut is_in_block = false;
    let mut word: String = String::new();
    // let mut array = Vec::<String>::new();
    let mut params: HashMap<String, String> = HashMap::new();

    for char in string.chars() {
        if char == '{' {
            is_in_block = true;
            continue;
        } //if {}

        if is_in_block {
            if char != '}' {
                word.push(char);
            } else {
                is_in_block = false;
                // println!("{}", word);
                params.insert(word.clone(), "".to_string());
                // array.push(word.clone());
                word.clear();
            }
        }
    } //for loop

    return params;
}
