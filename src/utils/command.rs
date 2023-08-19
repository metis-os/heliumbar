//
pub fn run(command: &String) -> String {
    match std::process::Command::new(command).output() {
        Ok(output) => {
            return String::from_utf8(output.stdout).unwrap();
        }

        Err(err) => return err.to_string(),
    } //match
}
