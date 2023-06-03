use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct SaveData {
    data: Vec<u8>,
}

impl SaveData {
    fn new(data: Vec<u8>) -> SaveData {
        SaveData { data }
    }
}

pub fn read_gen2_save(file_path: &str) -> Result<SaveData, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Could not open file")),
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => Ok(SaveData::new(buffer)),
        Err(_) => Err(String::from("Could not read file")),
    }
}

pub fn write_gen2_save(file_path: &str, save_file: &SaveData) -> Result<(), String> {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Could not create file")),
    };

    match file.write_all(&save_file.data) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Could not write to file")),
    }
}
