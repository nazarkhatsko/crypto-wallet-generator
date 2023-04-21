use std::fs::File;
use std::io::{Read, Write};

pub fn read(path: &String) -> Result<String, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("File not found: {}", path)),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err(format!("Unable to read file: {}", path)),
    }
}

pub fn write(path: &String, contents: &String) -> Result<(), String> {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Unable to create file: {}", path)),
    };
    match file.write_all(contents.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Unable to write to file: {}", path)),
    }
}
