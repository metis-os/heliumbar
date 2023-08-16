// use json;
//hello

pub fn read_config() -> Option<json::JsonValue> {
    let user = std::env::var("HOME").expect("Unable to find the user name");
    let mut path = String::from(&user);
    path.push_str("/.config/heliumbar/helium.conf");
    println!("{}", path);
    let data = std::fs::read_to_string(&path).unwrap();

    let config = json::parse(&data);
    if config.is_err() {
        println!("Error occur parsing the json. Switching to default conf");
        return Some(config.unwrap());
    }
    return Some(config.unwrap());
}
