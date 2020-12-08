use crate::aoc_utils::read_input;
use regex::Regex;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

struct Bag {
    color: String,
    number: u32,
    parent_color: String,
}

// Add this to the struct so we can `println!("{:?}", bags)`
impl std::fmt::Debug for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Bag")
            .field("color", &self.color)
            .field("number", &self.number)
            .field("parent_color", &self.parent_color)
            .finish()
    }
}

fn read_bag(line: &str) -> Vec<Bag> {
    let mut result_bags: Vec<Bag> = vec![];
    let regex_string =
        r"(?P<start_prefix>[a-z]+) (?P<start_color>[a-z]+) bags contain (?P<contents>.*)+";
    let regex = Regex::new(&regex_string).unwrap();
    let capture_result = regex.captures(line).expect("Invalid bag line");
    let regex_string_contents =
        r"(?P<number>\d+) (?P<prefix>[a-z]+) (?P<color>[a-z]+) bags?,?\.?\s?";
    let regex_contents = Regex::new(&regex_string_contents).unwrap();
    for cap in regex_contents.captures_iter(&capture_result["contents"]) {
        result_bags.push(Bag {
            color: format!("{} {}", &cap["prefix"], &cap["color"]),
            number: cap["number"].parse::<u32>().unwrap_or(0),
            parent_color: format!(
                "{} {}",
                &capture_result["start_prefix"], &capture_result["start_color"]
            ),
        })
    }
    return result_bags;
}

fn count_bags(bags: &Vec<Bag>, needle: &str, provided_counted_bags: &Vec<String>) -> Vec<String> {
    let mut counted_bags: Vec<String> = vec![];
    counted_bags.extend(provided_counted_bags.iter().cloned());

    for bag in bags {
        if bag.color == needle {
            if !counted_bags.contains(&bag.parent_color) {
                let col = &bag.parent_color;
                counted_bags.push(String::from(col));
                counted_bags = count_bags(&bags, &bag.parent_color, &counted_bags);
            }
        }
    }
    return counted_bags;
}

fn part1(input: &String) {
    let mut bags: Vec<Bag> = vec![];

    for line in input.lines() {
        bags.extend(read_bag(line));
    }
    let counted: Vec<String> = vec![];
    let counted_bags = count_bags(&bags, "shiny gold", &counted);
    println!("Part 1: {}", counted_bags.len());
}

fn count_inside(bags: &Vec<Bag>, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for bag in bags {
        if bag.parent_color == needle {
            count += &bag.number * count_inside(&bags, &bag.color);
            count += &bag.number;
        }
    }
    return count;
}

fn part2(input: &String) {
    let mut bags: Vec<Bag> = vec![];

    for line in input.lines() {
        bags.extend(read_bag(line));
    }
    let count = count_inside(&bags, "shiny gold");
    println!("Part 2: {}", count);
}
