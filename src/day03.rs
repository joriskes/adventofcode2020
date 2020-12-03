use crate::aoc_utils::read_input;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    let mapped: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    part1(&mapped);
    part2(&mapped);
}

const TREE: char = '#';

fn count_trees_on_slope(mapped: &Vec<Vec<char>>, step_x: usize, step_y: usize) -> u32 {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut tree_count = 0;
    let map_max_x = mapped[start_y].len();
    while start_y < mapped.len() {
        if mapped[start_y][start_x] == TREE {
            tree_count += 1;
        }

        start_x += step_x;
        start_y += step_y;
        if start_x >= map_max_x {
            start_x -= map_max_x;
        }
    }

    return tree_count;
}

fn part1(mapped: &Vec<Vec<char>>) {
    println!("Part1: {}", count_trees_on_slope(&mapped, 3, 1))
}

const TO_CHECK: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn part2(mapped: &Vec<Vec<char>>) {
    let mut tree_count: u128 = 0;
    TO_CHECK.iter().for_each(|(step_x, step_y)| {
        if tree_count == 0 {
            tree_count = count_trees_on_slope(&mapped, *step_x, *step_y) as u128;
        } else {
            tree_count *= count_trees_on_slope(&mapped, *step_x, *step_y) as u128;
        }
    });
    println!("Part2: {}", tree_count)
}
