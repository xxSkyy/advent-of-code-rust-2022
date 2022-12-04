use std::fs;

pub fn load_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Failed to read the file")
}
