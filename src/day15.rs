use crate::aoc_utils::read_input;
use std::collections::HashMap;
use std::time::Instant;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    let nums: Vec<u64> = input
        .split(',')
        .map(|n| n.parse::<u64>().unwrap_or(0))
        .collect();

    part1(&nums);
    part2(&nums);
}

fn last_number_spoken(nums: &Vec<u64>, turn_count: u64) -> u64 {
    let mut speak_log: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut turn: u64 = 1;
    let mut prev_spoken: u64 = 0;
    let mut spoken: u64 = 0;
    let start = Instant::now();

    for num in nums {
        speak_log.insert(*num, (0, turn));
        // println!("On turn: {} they speak {}", turn, *num);
        turn += 1;
        prev_spoken = *num;
    }

    while turn <= turn_count {
        if speak_log.contains_key(&prev_spoken) {
            // Number was spoken
            let stat = speak_log.entry(prev_spoken).or_insert((0, 0));
            let (from, to) = stat;
            // First time it was spoken
            if *from == 0 {
                spoken = 0;
            } else {
                spoken = *to - *from;
            }
        } else {
            // Number was not spoken yet
            speak_log.insert(prev_spoken, (0, turn));
            spoken = 0;
        }
        // println!("On turn: {} they speak {}", turn, spoken);
        // Update speak log
        let stat = speak_log.entry(spoken).or_insert((0, 0));
        let (from, to) = stat;
        // Update log
        *from = *to;
        *to = turn;

        // Next turn
        turn += 1;
        prev_spoken = spoken;
        if turn % 1000000 == 0 {
            // println!("Turn {} ({:.2?})", turn, start.elapsed());
        }
    }
    println!("Done ({:.2?})", start.elapsed());
    return spoken;
}

fn part1(nums: &Vec<u64>) {
    let spoken = last_number_spoken(nums, 2020);
    println!("Part 1: {}", spoken);
}

fn part2(nums: &Vec<u64>) {
    let spoken = last_number_spoken(nums, 30000000);
    println!("Part 2: {}", spoken);
}
