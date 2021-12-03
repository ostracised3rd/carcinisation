use std::fs;


pub fn load_data(filename: &str) -> String {
    fs::read_to_string(filename).expect(&format!("file {} couldn't be read", filename))
}