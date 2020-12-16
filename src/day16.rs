use crate::aoc_utils::read_input;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Field {
    label: String,
    valid_numbers: Vec<u64>,
    index_options: Vec<u64>,
}

fn read_rules(input: &String) -> Vec<Field> {
    let mut rules: Vec<Field> = vec![];
    for line in input.lines() {
        if line.trim() == "" {
            break;
        }
        let re = Regex::new(r"^([a-zA-Z\s]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let caps = re.captures(line).unwrap();
        assert_eq!(caps.len(), 6, "Invalid field received `{}`", line);

        let label = caps.get(1).map_or("", |m| m.as_str()).trim();
        let from1: u64 = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .trim()
            .parse()
            .expect("Invalid number (from 1)");
        let to1: u64 = caps
            .get(3)
            .map_or("", |m| m.as_str())
            .trim()
            .parse()
            .expect("Invalid number (to 1)");
        let from2: u64 = caps
            .get(4)
            .map_or("", |m| m.as_str())
            .trim()
            .parse()
            .expect("Invalid number (from 2)");
        let to2: u64 = caps
            .get(5)
            .map_or("", |m| m.as_str())
            .trim()
            .parse()
            .expect("Invalid number (to 2)");

        let mut numbers: Vec<u64> = vec![];
        for i in from1..=to1 {
            numbers.push(i);
        }
        for i in from2..=to2 {
            numbers.push(i);
        }

        // println!("Field: {}, {:?}", label, numbers);

        rules.push(Field {
            label: String::from(label),
            valid_numbers: numbers,
            index_options: vec![],
        })
    }
    return rules;
}

fn line_to_vec(line: &str) -> Vec<u64> {
    return line
        .split(',')
        .map(|n| n.parse::<u64>().unwrap_or(0))
        .collect();
}

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);
    let rules: Vec<Field> = read_rules(&input);
    let mut nearby: Vec<Vec<u64>> = vec![];
    let mut own: Vec<u64> = vec![];

    let mut reading: bool = false;
    for (index, line) in input.lines().into_iter().enumerate() {
        if reading {
            nearby.push(line_to_vec(line));
        }
        if line.contains("nearby tickets") {
            reading = true;
        }
        if line.contains("your ticket") {
            own = line_to_vec(input.lines().nth(index + 1).unwrap())
        }
    }

    part1(&rules, &nearby);
    part2(&rules, &own, &nearby);
}

fn part1(rules: &Vec<Field>, nearby: &Vec<Vec<u64>>) {
    let mut error_rate: u64 = 0;
    for near in nearby {
        for num in near {
            let mut found: bool = false;
            for rule in rules {
                if rule.valid_numbers.contains(num) {
                    found = true;
                    // println!("Found {} in {}", num, rule.label)
                }
            }
            if !found {
                error_rate += num;
            }
        }
    }

    println!("Part 1: {}", error_rate);
}

fn part2(rules: &Vec<Field>, own: &Vec<u64>, nearby: &Vec<Vec<u64>>) {
    let mut rules_indexed: Vec<Field> = rules.to_vec();

    // Fill index_options with all possible indexes so we can eliminate them later
    let l = own.len() as u64;
    for rule in rules_indexed.iter_mut() {
        for i in 0..l {
            rule.index_options.push(i)
        }
    }

    // Get all part 1 approved tickets
    let mut near_clean: Vec<_> = vec![];
    for near in nearby {
        let mut found_count: u64 = 0;
        for num in near {
            let mut found: bool = false;
            for rule in rules {
                if rule.valid_numbers.contains(num) {
                    found = true;
                    // println!("Found {} in {}", num, rule.label)
                }
            }
            if found {
                found_count += 1;
            }
        }
        if found_count == near.len() as u64 {
            near_clean.push(near.to_vec());
        }
    }

    // Set possible indexes for each rule
    for rule in rules_indexed.iter_mut() {
        for near in &near_clean {
            // Find indexes that abide to this rule
            let mut indexes: Vec<u64> = vec![];
            for (index, num) in near.iter().enumerate() {
                if rule.valid_numbers.contains(num) {
                    indexes.push(index as u64)
                }
            }
            if indexes.len() > 0 {
                // Remove indexes from rule that are not abiding
                let mut new_indexes: Vec<u64> = vec![];
                for index_option in &rule.index_options {
                    if indexes.contains(&index_option) {
                        new_indexes.push(*index_option);
                    }
                }
                rule.index_options = new_indexes;
            }
        }
    }

    // Now the index puzzle should be solvable by elimination
    let mut used_filters: Vec<u64> = vec![];
    loop {
        let mut done: bool = true;
        let mut filter_num = 0;
        let mut filter_num_found = false;
        for rule in rules_indexed.iter_mut() {
            // As long as we have rules with multiple indexes we're not done
            if rule.index_options.len() > 1 {
                done = false;
            }
            // This option now has only one index, remove that index from the other options
            // (if not already used)
            if rule.index_options.len() == 1 {
                if !used_filters.contains(&rule.index_options[0]) {
                    filter_num = rule.index_options[0];
                    filter_num_found = true;
                }
            }
        }

        if filter_num_found {
            for rule_mut in rules_indexed.iter_mut() {
                if rule_mut.index_options.len() > 1 {
                    rule_mut.index_options = rule_mut
                        .index_options
                        .iter()
                        .cloned()
                        .filter(|i| *i != filter_num)
                        .collect();
                }
            }
            used_filters.push(filter_num);
        } else {
            println!("Could not find a filter num?!");
            done = true;
        }
        if done {
            break;
        }
    }

    // The rules now have an index, calculate output
    let mut result: u64 = 1;
    for rule in rules_indexed {
        if rule.label.contains("departure") {
            result *= own[rule.index_options[0] as usize];
        }
    }

    println!("Part 2: {}", result);
}
