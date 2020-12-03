use std::fs::File;
use std::io::Write;
use std::{env, fs};

pub fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading file");
}

pub fn download_input(filename: &str, day: u32) {
    let mut data = Vec::new();
    let mut easy = curl::easy::Easy::new();
    let session = env::var("ADVENT_SESSION").expect("ADVENT_SESSION env variable not found");
    easy.url(&format!("https://adventofcode.com/2020/day/{}/input", day))
        .unwrap();

    // cookie
    let mut sess: String = String::from("session=");
    sess.push_str(&session);
    easy.cookie(&sess).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    // Convert it to `String`
    let body = String::from_utf8(data).expect("body is not valid UTF8!");

    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(filename).unwrap();
    file.write_all(body.as_bytes()).expect("write failed");
}
