use crate::aoc_utils::read_input;
use std::collections::HashMap;
use std::time::Instant;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let arrival: u64 = input.lines().nth(0).unwrap().parse().unwrap_or(0);
    let lines: Vec<&str> = input.lines().nth(1).unwrap().split(",").collect();

    let mut closest_line = "";
    let mut diff_minutes: u64 = 9999999;
    for line in lines {
        if line != "x" {
            let line_num_ori: u64 = line.parse().unwrap();
            let mut line_num: u64 = line_num_ori;
            while line_num < arrival {
                line_num += line_num_ori;
            }
            let diff = line_num - arrival;
            if diff < diff_minutes {
                diff_minutes = diff;
                closest_line = line;
            }
        }
    }
    println!(
        "Part 1: {}",
        closest_line.parse::<u64>().unwrap() * diff_minutes
    )
}

fn part2(input: &String) {
    let mut offsets: HashMap<u64, u64> = HashMap::new();
    let mut lowest_offset: u64 = 9999;
    let lines: Vec<&str> = input.lines().nth(1).unwrap().split(",").collect();
    let mut index_offset = 0;
    for line in lines {
        if line != "x" {
            let line_num = line.parse::<u64>().unwrap();
            if line_num < lowest_offset {
                lowest_offset = line_num;
            }
            offsets.insert(line_num, index_offset);
        }
        index_offset += 1;
    }
    let mut timestamp: u64 = lowest_offset;
    let start = Instant::now();
    loop {
        let mut ok_count = 0;

        for (line, index_offset) in offsets.iter() {
            let num = (timestamp + index_offset) / line;
            if line * num == timestamp + index_offset {
                ok_count += 1;
            } else {
                break;
            }
        }
        if ok_count == offsets.len() {
            break;
        }
        timestamp += lowest_offset;
        if timestamp / lowest_offset % 100_000_000 == 0 {
            println!("Timestamp {} ({:.2?})", timestamp, start.elapsed());
        }
    }
    println!("Part 2: {} ({:.2?})", timestamp, start.elapsed());
}
