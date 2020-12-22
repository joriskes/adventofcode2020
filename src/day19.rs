use crate::aoc_utils::read_input;
use regex::Regex;
use std::collections::HashMap;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    let mut parsing_rules = true;
    let mut rules: HashMap<u64, String> = HashMap::new();
    let mut messages: Vec<String> = vec![];

    for line in input.lines() {
        if line.trim() == "" {
            parsing_rules = false;
        } else {
            if parsing_rules {
                let splitres: Vec<&str> = line.split(":").collect();
                let index: u64 = splitres[0].trim().parse().unwrap();
                let splitrule = splitres[1].trim();

                rules.insert(index, String::from(splitrule));
            } else {
                messages.push(String::from(line));
            }
        }
    }

    part1(&rules, &messages);
    part2(&rules, &messages);
}

fn parse_index(rules: &HashMap<u64, String>, index: u64) -> String {
    let rule = rules.get(&index).unwrap();
    let rule_split: Vec<&str> = rule.trim().split_whitespace().collect();
    let mut newrules: Vec<String> = vec![String::from(""), String::from("")];
    let mut index = 0;
    for r in rule_split {
        if r == "|" {
            index = 1;
        } else {
            let i: i32 = r.parse().unwrap_or(-1);
            if i > -1 {
                let s = parse_index(rules, i as u64);
                newrules[index] += s.as_str();
            } else {
                newrules[index] += format!("{}", r.replace("\"", "")).as_str();
            }
        }
    }
    if newrules[1] != "" {
        return format!("({}|{})", newrules[0], newrules[1]);
    } else {
        return format!("{}", newrules[0]);
    }
}

fn parse_rule(rules: &mut HashMap<u64, String>) {
    let rules_immutable = rules.clone();
    for (own_index, rule) in rules.iter_mut() {
        let new_rule = parse_index(&rules_immutable, *own_index);
        *rule = new_rule;
    }
}

fn part1(rules: &HashMap<u64, String>, messages: &Vec<String>) {
    let mut rule_clone = rules.clone();
    parse_rule(&mut rule_clone);
    let created_regex = format!("^{}$", rule_clone.get(&0).unwrap());
    let regex = Regex::new(&created_regex).unwrap();
    let mut match_count = 0;
    for message in messages {
        if regex.is_match(message) {
            match_count += 1;
        }
    }
    println!("Part 1: {}", match_count);
}

fn part2(rules: &HashMap<u64, String>, messages: &Vec<String>) {
    let mut rule_clone = rules
        .iter()
        .map(|(index, rule)| {
            let mut res_rule = String::from(rule);
            if index == &8 {
                res_rule = String::from("42 +");
            }
            if index == &11 {
                res_rule = String::from("42 ( 42 ( 42 ( 42 ( 42 31 )* 31 )* 31 )* 31 )* 31");
            }
            return (*index, res_rule);
        })
        .collect();
    parse_rule(&mut rule_clone);
    let created_regex = format!("^{}$", rule_clone.get(&0).unwrap());
    let regex = Regex::new(&created_regex).unwrap();
    let mut match_count = 0;
    for message in messages {
        if regex.is_match(message) {
            match_count += 1;
        }
    }
    println!("Part 2: {}", match_count);
}
