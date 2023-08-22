use std::collections::HashMap;

use gtk::traits::ContainerExt;
use json::{Error, JsonValue};

use crate::builder::widgets_builder::{Align, WidgetConfig};
use crate::network::hyprland_socket::listen;
use crate::utils::{command, regex_matcher};
use glib::MainContext;
use gtk::prelude::*;
// use super::workspace::listen;

fn setup_default(mut text: String) -> (String, Result<JsonValue, Error>) {
    text = text
        .replace("{workspace}", "{workspace.id}")
        .replace("{activewindow}", "{title}");
    let out = command::run(&"hyprctl activewindow -j".to_string())
        .trim()
        .to_string();
    let mut jsondata = json::parse(&out);
    if let Ok(json) = jsondata {
        jsondata = Ok(json);
    }
    if let Some(data) = regex_matcher::format(&text, &out) {
        text = data;
    }

    return (text, jsondata);
}

pub fn build_label(left: &gtk::Box, center: &gtk::Box, right: &gtk::Box, config: WidgetConfig) {
    let original: String = config.format;

    let (text, jsondata) = setup_default(original.clone());

    // println!("{}", text);
    let label = gtk::Label::builder().label(text).build();
    label.style_context().add_class(&config.name_of_widget);
    match config.align {
        Align::CENTER => center.add(&label),
        Align::LEFT => left.add(&label),
        Align::RIGHT => right.add(&label),
    }

    update_widget(label, original, jsondata);
} //build_label

pub fn update_widget(
    label: gtk::Label,
    original: String,
    jsondata: Result<json::JsonValue, Error>,
) {
    let (sender, receiver) = MainContext::channel::<(String, String)>(glib::Priority::DEFAULT);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build();

    if let Err(err) = rt {
        println!("{}", err);
        return;
    }
    let mut params = regex_matcher::get_params(&original);
    if params.len() == 0 {
        return;
    }
    std::thread::spawn(move || rt.unwrap().block_on(listen(sender)));

    //listen for the socket
    params = set_current_win_wor(params, jsondata);
    hyprland_signal_receiver(receiver, params, original, label);
}

fn set_current_win_wor(
    mut params: HashMap<String, String>,
    json: Result<json::JsonValue, Error>,
) -> HashMap<String, String> {
    if let Ok(data) = json {
        if params.contains_key("workspace") {
            params.insert(
                "workspace".to_string(),
                data["workspace"]["name"].as_str().unwrap_or("").to_string(),
            );
        } //if workspace
        if params.contains_key("activewindow") {
            params.insert(
                "activewindow".to_string(),
                data["title"].as_str().unwrap_or("").to_string(),
            );
        } //if workspace
    } //jsonvale

    return params;
}

fn hyprland_signal_receiver(
    receiver: glib::Receiver<(String, String)>,
    mut params: HashMap<String, String>,
    original: String,
    label: gtk::Label,
) {
    let mut format_text: String = String::new();
    //reciver is here
    receiver.attach(None, move |(name, value)| {
        // println!("{}", name);

        if params.contains_key(&name) {
            //
            if name == "activewindow" {
                if let Some((_class, title)) = value.split_once(",") {
                    params.insert(name.trim().to_string(), title.trim().to_string());
                }
            } else {
                params.insert(name.trim().to_string(), value.trim().to_string());
            }
            format_text = original.clone();
            for (key, value) in params.clone().into_iter() {
                format_text = format_text.replace(&format!("{{{}}}", key), &value);
            }
            label.set_text(&format_text);
        }
        glib::ControlFlow::Continue
    });
}
