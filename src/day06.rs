use crate::aoc_utils::read_input;
use std::collections::HashMap;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

fn input_to_forms(input: &String) -> Vec<String> {
    // Merge form groups into single strings
    let mut res: Vec<String> = vec![];
    let mut index: usize = 0;
    res.push(String::from(""));
    for line in input.lines() {
        if line.trim() != "" {
            if res[index] != "" {
                res[index].push_str(" ");
            }
            res[index].push_str(line);
        } else {
            res.push(String::from(""));
            index += 1;
        }
    }
    return res;
}

fn part1(input: &String) {
    let forms = input_to_forms(input);
    let mut char_count = 0;
    for form in forms {
        for i in 0..26 {
            let char = char::from(97 + i);
            if form.find(char).is_some() {
                char_count += 1;
            }
        }
    }
    println!("Part 1: {}", char_count);
}

fn part2(input: &String) {
    let forms = input_to_forms(input);
    let mut total_count = 0;
    for form in forms {
        // Set individual char counts into a hashmap
        let groups: Vec<&str> = form.split(" ").collect();
        let mut char_list = HashMap::new();
        for group in groups.iter() {
            for i in 0..26 {
                let char = char::from(97 + i).to_string();
                if group.find(char.as_str()).is_some() {
                    let stat = char_list.entry(char.as_str().to_owned()).or_insert(0);
                    *stat += 1;
                }
            }
        }
        // Test the char counts in the hasmap for the group length
        for (_char, count) in char_list.iter() {
            if *count == groups.len() {
                total_count += 1;
            }
        }
    }
    println!("Part 2: {}", total_count);
}
