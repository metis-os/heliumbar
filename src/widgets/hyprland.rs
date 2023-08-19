use gtk::traits::ContainerExt;

use crate::ui::widget::{Align, WidgetConfig};
use crate::utils::{command, regex_matcher};
use glib::MainContext;
use gtk::prelude::*;

use super::workspace::listen;

pub fn build_label(left: &gtk::Box, center: &gtk::Box, right: &gtk::Box, config: WidgetConfig) {
    let original: String = config.format;
    let mut text = original.clone();
    let out = command::run(&"hyprctl activewindow -j".to_string())
        .trim()
        .to_string();
    if let Some(data) = regex_matcher::format("{title}", &out) {
        text = data;
    }
    //if json

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
    let mut workspace: String = String::new();
    let mut window: String = String::new();
    std::thread::spawn(move || rt.unwrap().block_on(listen(sender)));

    //listen for the socket
    receiver.attach(None, move |(name, value)| {
        // println!("{}", name);
        if params.contains(&name) {
            let text = if name == "activewindow" {
                window = value.trim().to_string();
                original
                    .replace(&format!("{{{}}}", name), &value)
                    .replace("{workspace}", &workspace)
            } else {
                workspace = value.trim().to_string();
                original
                    .replace(&format!("{{{}}}", name), &value)
                    .replace(&format!("{{{}}}", "activewindow"), &window)
            };

            // let text = &original.replace(&format!("{{{}}}", name), &value);
            label.set_text(&text.trim());
        }
        glib::ControlFlow::Continue
    });
}

pub fn get_params(string: &String) -> Vec<String> {
    let mut is_in_block = false;
    let mut word: String = String::new();
    let mut array = Vec::<String>::new();

    for c in string.chars() {
        if c == '{' {
            is_in_block = true;
            continue;
        } //if {}

        if is_in_block {
            if c != '}' {
                word.push(c);
            } else {
                is_in_block = false;
                // println!("{}", word);
                array.push(word.clone());
                word.clear();
            }
        }
    } //for loop

    return array;
}
