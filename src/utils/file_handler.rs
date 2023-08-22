use std::fs::File;
use std::io::{Read, Seek};
pub fn read_file_for_monitor(file: &mut File, buffer: &mut [u8]) -> String {
    match file.seek(std::io::SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
            return "".to_string();
        }
    } //match

    let stat = file.read(buffer);
    if let Err(err) = stat {
        println!("{}", err);
        return "".to_string();
    }
    let stat = stat.unwrap();
    if stat > 0 {
        String::from_utf8_lossy(&buffer[0..stat]).trim().to_string()
    } else {
        "".to_string()
    }
}
