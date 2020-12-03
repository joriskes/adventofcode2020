use crate::aoc_utils::download_input;
use std::{env, fs, io};

pub mod aoc_utils;
pub mod day01;
pub mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day_str = String::new();

    if args.len() >= 2 {
        // Get day string from args
        day_str = args[1].clone();
    } else {
        // Ask for day
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day_str)
            .expect("Failed to read line");
    }
    let day: u32 = day_str
        .trim()
        .parse()
        .expect("Day input needs to be a number");

    let formatted_filename = format!("input/day{:02}.txt", day);
    let input_filename = formatted_filename.as_str();

    if !fs::metadata(input_filename).is_ok() {
        download_input(input_filename, day);
        println!("Day {} input download to {} done", day, input_filename);
    }

    println!("Running day {}", day);
    match day {
        1 => day01::run(input_filename),
        2 => day02::run(input_filename),
        _ => println!("Unkown day {}", day),
    }
}
