use crate::aoc_utils::read_input;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

fn calc_sum(line: &str, part: u64) -> u64 {
    let operations: Vec<char> = line.chars().filter(|c| c != &' ').collect();
    let mut solvable_operations: Vec<String> = vec![];
    let mut start = 0;
    let mut in_brackets: u32 = 0;
    for (index, operation) in operations.iter().enumerate() {
        if operation == &'(' {
            if in_brackets == 0 {
                start = index;
            }
            in_brackets += 1;
        }
        if in_brackets == 0 {
            solvable_operations.push(operation.to_string());
        }
        if operation == &')' {
            in_brackets -= 1;
            if in_brackets == 0 {
                let slice: String = operations[start + 1..index].into_iter().collect();
                let s = calc_sum(slice.as_str(), part).to_string().to_owned();
                solvable_operations.push(s);
            }
        }
    }

    let mut sum: u64 = 0;
    let mut operation: char = ' ';
    if part == 1 {
        for op in solvable_operations {
            if op == "+" || op == "*" {
                operation = op.chars().nth(0).unwrap();
            } else {
                let num: u64 = op.parse().unwrap();
                match operation {
                    '+' => {
                        sum = sum + num;
                    }
                    '*' => {
                        sum = sum * num;
                    }
                    _ => {
                        sum = num;
                    }
                }
            }
        }
    } else {
        let mut solvable_operations2: Vec<String> = vec![];
        let mut num = 0;
        // First solve the +'s and build a shadow with the *'s
        for op in solvable_operations {
            if op == "+" || op == "*" {
                operation = op.chars().nth(0).unwrap();
            } else {
                num = op.parse().unwrap();
                match operation {
                    '+' => {
                        sum = sum + num;
                    }
                    '*' => {
                        if sum > 0 {
                            solvable_operations2.push(sum.to_string());
                        }
                        solvable_operations2.push(String::from("*"));
                        sum = num;
                        operation = ' ';
                    }
                    _ => {
                        sum = num;
                    }
                }
            }
        }
        if sum > 0 {
            solvable_operations2.push(sum.to_string());
        } else {
            if num > 0 {
                solvable_operations2.push(num.to_string());
            }
        }
        sum = 0;
        // Now solve the *'s
        for op in solvable_operations2 {
            if op == "*" {
                operation = op.chars().nth(0).unwrap();
            } else {
                num = op.parse().unwrap();
                match operation {
                    '*' => {
                        if sum == 0 {
                            sum = num
                        } else {
                            sum = sum * num;
                        }
                    }
                    _ => {
                        sum = num;
                    }
                }
            }
        }
    }
    return sum;
}

fn part1(input: &String) {
    let mut total_sum = 0;

    for line in input.lines() {
        total_sum += calc_sum(line, 1);
    }
    println!("Part 1: {}", total_sum);
}

fn part2(input: &String) {
    let mut total_sum = 0;

    for line in input.lines() {
        total_sum += calc_sum(line, 2);
    }
    println!("Part 2: {}", total_sum);
}
