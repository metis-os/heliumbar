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

pub fn get_particular_dir_path(path: String, file_to_search: String) -> Option<String> {
    let file_dirs = std::fs::read_dir(path);
    if let Err(err) = file_dirs {
        println!("{}", err);
        return None;
    }
    for each in file_dirs.unwrap() {
        if let Ok(dir) = each {
            let dir = dir.path();
            if dir.is_dir() {
                let found_dir = dir.to_str();
                if let Some(found_dir) = found_dir {
                    let final_path = format!("{}/{}", found_dir, file_to_search);
                    if std::path::Path::new(&final_path).exists() {
                        return Some(found_dir.to_string());
                    } //if path
                } //findal
            } //if dir
        } //found some files
    }

    return None;
}
