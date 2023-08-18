pub fn format() {
    let string = "Hello{name}{sub.first}";
    let pattern = r"\{(.*)\}";
    let re = regex::Regex::new(pattern).unwrap();

    // if let Some(matches) = re.captures(string) {
    //     println!("{}", matches.get(1).unwrap().as_str());
    // }

    for capture in re.captures_iter(string) {
        println!("{}", capture.get(1).unwrap().as_str());
    }
}
