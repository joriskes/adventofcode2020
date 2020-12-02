use crate::aoc_utils::read_input;

pub fn run() {
    let input = read_input("input/day01.txt");

    part1(&input);
    part2(&input);
}

fn search_sum(input: &String, sum_to_find: u32) -> (u32, u32) {
    let mut res1: u32;
    let mut res2: u32;

    for line1 in input.lines() {
        res1 = line1.trim().parse().expect("Invalid number");
        for line2 in input.lines() {
            res2 = line2.trim().parse().expect("Invalid number");
            if res1 + res2 == sum_to_find {
                return (res1, res2);
            }
        }
    }

    return (0, 0);
}
fn part1(input: &String) {
    let (r1, r2) = search_sum(input, 2020);
    println!("Part 1: result {}", r1 * r2);
}
fn part2(input: &String) {
    for line in input.lines() {
        let r1: u32 = line.trim().parse().expect("Invalid number");
        let (r2, r3) = search_sum(input, 2020 - r1);
        if r2 > 0 && r3 > 0 {
            println!("Part 2: result {}", r1 * r2 * r3);
            break;
        }
    }
}
