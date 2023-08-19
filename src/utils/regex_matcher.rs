use json;

pub fn format(string: &str, json_data: &str) -> Option<String> {
    // let string = "Hello {name} is {sub} and we are good {main}";
    // let pattern = r"\{(.*)\}";
    // let re = regex::Regex::new(pattern).unwrap();

    // if let Some(matches) = re.captures(string) {
    //     println!("{}", matches.get(1).unwrap().as_str());
    // }
    // let json_data = "{\"name\":\"shyam\",\"sub\":\"english\",\"main\":\"good\"}";
    let json_parse = json::parse(&json_data);
    if let Err(err) = json_parse {
        println!("{}", err);
        return None;
    }
    let json_parse = json_parse.unwrap();

    let mut is_in_block = false;
    let mut word: String = String::new();
    let mut out = string.to_string();

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
                out = out.replace(
                    &format!("{{{}}}", word),
                    json_parse[&word].as_str().unwrap_or(""),
                );
                word.clear();
            }
        }
    } //for loop

    return Some(out);
    // for capture in re.captures_iter(string) {
    //     println!("{}", capture.get(1).unwrap().as_str());
    // }
}
