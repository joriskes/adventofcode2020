use crate::aoc_utils::{input_into_numbers, read_input};
use std::u32::MAX;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);
    let input_vec = input_into_numbers(input);

    part1(&input_vec);
    part2(&input_vec);
}

fn part1(input: &Vec<u32>) {
    let mut in_jolt = 0;
    let mut diff_list: [u32; 3] = [0; 3];
    diff_list[0] = 0;
    diff_list[1] = 0;
    diff_list[2] = 0;
    let mut sorted = input.clone();
    sorted.sort();

    for jolt in sorted {
        if jolt > in_jolt && jolt < in_jolt + 4 {
            let diff: usize = (jolt - in_jolt - 1) as usize;
            in_jolt = jolt;
            diff_list[diff] += 1;
        }
    }
    // Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating is 22 jolts (always a difference of 3).
    diff_list[2] += 1;

    println!("Part 1 {}", diff_list[0] * diff_list[2]);
}

fn part2(input: &Vec<u32>) {
    let mut sorted = input.clone();
    // Add start jolts
    sorted.push(0);
    sorted.sort();
    // Add final jolts
    sorted.push(sorted.get(sorted.len() - 1).unwrap() + 3);

    let mut path_count: Vec<u64> = vec![];
    for _p in 0..sorted.len() {
        path_count.push(0);
    }
    path_count[0] = 1;
    for (index, jolt) in sorted.iter().enumerate() {
        for i in 1..4 {
            if index + i < sorted.len() {
                let next = sorted.get(index + i).unwrap_or(&MAX);
                if next < &MAX && next - jolt <= 3 {
                    path_count[index + i] += path_count[index];
                }
            }
        }
    }
    println!("Part 2 {}", path_count.pop().unwrap());
}
