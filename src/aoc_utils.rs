use std::fs;

pub fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading file");
}
