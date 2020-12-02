use crate::aoc_utils::read_input;
use regex::Regex;

pub fn run() {
    let input = read_input("input/day02.txt");

    part1(&input);
    part2(&input);
}

fn extract_policy(policy: &str) -> (u32, u32, &str) {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]+)$").unwrap();
    let caps = re.captures(policy).unwrap();
    assert_eq!(caps.len(), 4, "Invalid policy received `{}`", policy);
    // println!("Caps: {:?}", caps);

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

    return (from, to, char_to_find);
}

fn part1(input: &String) {
    let mut ok_pass_count = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        assert_eq!(parts.len(), 2, "Invalid input received {}", line);

        let policy = parts[0].trim();
        let password = parts[1].trim();

        let (from, to, char_to_find) = extract_policy(policy);

        let mut char_count = 0;
        for char in password.chars() {
            if char.to_string() == char_to_find {
                char_count += 1;
            }
        }
        if char_count >= from && char_count <= to {
            ok_pass_count += 1;
        }
    }
    println!("Part 1: {}", ok_pass_count);
}
fn part2(input: &String) {
    let mut ok_pass_count = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        assert_eq!(parts.len(), 2, "Invalid input received {}", line);

        let policy = parts[0].trim();
        let password = parts[1].trim();

        let (p1, p2, char_to_find) = extract_policy(policy);

        assert!(
            password.len() >= p2 as usize,
            "Invalid index in policy {}",
            policy
        );

        let char1 = password.chars().nth((p1 - 1) as usize).unwrap().to_string();
        let char2 = password.chars().nth((p2 - 1) as usize).unwrap().to_string();

        if (char1 == char_to_find && char2 != char_to_find)
            || (char1 != char_to_find && char2 == char_to_find)
        {
            ok_pass_count += 1;
        }
    }
    println!("Part 2: {}", ok_pass_count);
}
