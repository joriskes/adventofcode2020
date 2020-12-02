use crate::aoc_utils::read_input;
use regex::Regex;
const DEBUG: bool = false;

pub fn run() {
    let input = read_input("input/day02.txt");

    part1(&input);
    part2(&input);
}

fn check_policy(policy: &str, password: &str) -> bool {
    if DEBUG {
        println!("Checking: {} -> {}", policy, password);
    }

    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]+)$").unwrap();
    let caps = re.captures(policy).unwrap();
    assert_eq!(caps.len(), 4, "Invalid policy received `{}`", policy);
    if DEBUG {
        println!("Caps: {:?}", caps);
    }

    let from: u32 = caps
        .get(1)
        .map_or("", |m| m.as_str())
        .trim()
        .parse()
        .expect("Invalid number (from)");
    let to: u32 = caps
        .get(2)
        .map_or("", |m| m.as_str())
        .trim()
        .parse()
        .expect("Invalid number (from)");

    let char_to_find = caps.get(3).map_or("", |m| m.as_str());

    if DEBUG {
        println!("Test from:{}, to:{}, char:{}", from, to, char_to_find);
    }

    let mut char_count = 0;
    for char in password.chars() {
        if char.to_string() == char_to_find {
            char_count += 1;
        }
    }

    if DEBUG {
        println!("Password: {} count {}", password, char_count);
    }

    return char_count >= from && char_count <= to;
}

fn part1(input: &String) {
    let mut ok_pass_count = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        assert_eq!(parts.len(), 2, "Invalid input received {}", line);
        if check_policy(parts[0].trim(), parts[1].trim()) {
            ok_pass_count += 1;
        }
    }
    println!("Part 1: {}", ok_pass_count);
}
fn part2(_input: &String) {}
