use crate::aoc_utils::download_input;
use std::{env, fs, io};

pub mod aoc_utils;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

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
        3 => day03::run(input_filename),
        4 => day04::run(input_filename),
        5 => day05::run(input_filename),
        6 => day06::run(input_filename),
        7 => day07::run(input_filename),
        8 => day08::run(input_filename),
        9 => day09::run(input_filename),
        10 => day10::run(input_filename),
        11 => day11::run(input_filename),
        12 => day12::run(input_filename),
        13 => day13::run(input_filename),
        _ => println!("Unknown day {}", day),
    }
}
