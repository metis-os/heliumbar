use crate::utils::regex_matcher;
use gtk::prelude::*;
use std::collections::HashMap;

//

pub fn listen(receiver: glib::Receiver<(String, String)>, original: String, label: gtk::Label) {
    let mut params = regex_matcher::get_params(&original);
    let mut format_text: String = String::new();
    //reciver is here
    receiver.attach(None, move |(name, value)| {
        // println!("{}", name);

        if params.contains_key(&name) {
            //
            params.insert(name.trim().to_string(), value.trim().to_string());
            format_text = original.clone();
            for (key, value) in params.clone().into_iter() {
                format_text = format_text.replace(&format!("{{{}}}", key), &value);
            }
            label.set_text(&format_text);
        }
        glib::ControlFlow::Continue
    });
}
