use crate::aoc_utils::read_input;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    let mut input_vec: Vec<u32> = vec![];
    for line in input.lines() {
        input_vec.push(line.parse::<u32>().unwrap_or(0));
    }
    part1and2(&input_vec);
}

fn fits_preamble(preamble: &[u32], looking_for: u32) -> bool {
    for (i1, num1) in preamble.iter().enumerate() {
        for (i2, num2) in preamble.iter().enumerate() {
            if i1 != i2 && num1 + num2 == looking_for {
                return true;
            }
        }
    }
    return false;
}

fn part1and2(input: &Vec<u32>) {
    const PREAMBLE_SIZE: usize = 25;
    let mut p1res: u32 = 0;

    for i in 0..=input.len() - PREAMBLE_SIZE {
        if !fits_preamble(&input[i..=i + PREAMBLE_SIZE], input[i + PREAMBLE_SIZE + 1]) {
            p1res = input[i + PREAMBLE_SIZE + 1];
            println!("Part 1: {}", p1res);
            break;
        }
    }

    for i in 0..=input.len() {
        let mut smallest: u32 = input[i];
        let mut largest: u32 = input[i];
        let mut sum = input[i];
        let mut q = i + 1;
        while sum < p1res {
            sum += input[q];
            if input[q] < smallest {
                smallest = input[q]
            }
            if input[q] > largest {
                largest = input[q]
            }
            q += 1;
        }
        if sum == p1res {
            println!("Part 2: {}", smallest + largest);
            break;
        }
    }
}
