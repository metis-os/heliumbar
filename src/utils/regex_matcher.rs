pub fn format(string: &str, json_data: &str) -> Option<String> {
    let json_parse = json::parse(&json_data);
    if let Err(err) = json_parse {
        println!("{}", err);
        return None;
    }
    let json_parse = json_parse.unwrap();

    let mut is_in_block = false;
    let mut word: String = String::new();
    let mut out = string.to_string();
    let mut temp;

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
                let data: Vec<&str> = word.split(".").collect();
                if data.len() == 1 {
                    temp = json_parse[&word].to_string();
                } else if data.len() == 2 {
                    temp = json_parse[data[0]][data[1]].to_string();
                } else {
                    temp = json_parse[data[0]][data[1]][data[2]].to_string();
                }
                out = out.replace(&format!("{{{}}}", word), &temp);
                word.clear();
            }
        }
    } //for loop

    return Some(out);
    // for capture in re.captures_iter(string) {
    //     println!("{}", capture.get(1).unwrap().as_str());
    // }
}
