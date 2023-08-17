// use json;
//hello

use json::Error;

use crate::utils;

pub fn read_file(path: &String) -> Option<String> {
    let data = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                println!("{} file is not found in your system", path);
                return None;
            } else if err.kind() == std::io::ErrorKind::PermissionDenied {
                println!("Permission denied to access the {} file", path);
                return None;
            } else {
                println!("Something went wrong reading the {} file", path);
                return None;
            }
        } //error
    }; //reading the string
    Some(data)
}

pub fn get_config_path() -> Option<String> {
    let user = std::env::var("HOME");
    if let Err(_) = user {
        println!("Unble to find the username of the system");
        return None;
    }
    let mut path = String::from(&user.unwrap());
    path.push_str(utils::constants::CONFIG_PATH);
    Some(path)
}

pub fn read_config() -> Result<json::JsonValue, String> {
    let path = get_config_path();

    if let None = path {
        return Err("Unable to find the config path".to_string());
    }
    let data = read_file(&path.unwrap());
    if let None = data {
        return Err("Unable to read the config file".to_string());
    }
    parse_config(data)
}

pub fn parse_config(data: Option<String>) -> Result<json::JsonValue, String> {
    let config = json::parse(&data.unwrap());

    if let Err(error) = config {
        println!("Error occur parsing the json. Switching to default conf");
        return Err(error.to_string());
    }
    return Ok(config.unwrap());
}
