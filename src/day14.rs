use crate::aoc_utils::read_input;
use std::collections::HashMap;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

fn value_with_bitmask(value: u64, bitmask: &str) -> u64 {
    let bits: String = format!("{:0>36}", format!("{:b}", value));
    let mut result_bits: String = String::from("");

    for (index, char) in bitmask.chars().into_iter().enumerate() {
        let mut result_char = bits.chars().nth(index).unwrap_or('0');
        if char != 'X' {
            result_char = char;
        }
        result_bits.push(result_char);
    }
    return u64::from_str_radix(result_bits.as_str(), 2).unwrap_or(0);
}
fn part1(input: &String) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut bitmask = "";
    for line in input.lines() {
        let line_vec: Vec<&str> = line.split(" = ").collect();
        assert_eq!(line_vec.len(), 2);
        let operation = line_vec[0];
        let value = line_vec[1];

        if operation == "mask" {
            bitmask = value;
        } else {
            let addr = operation
                .replace("mem[", "")
                .replace("]", "")
                .parse::<u64>()
                .unwrap_or(0);
            let mem = memory.entry(addr).or_insert(0);
            *mem = value_with_bitmask(value.parse::<u64>().unwrap_or(0), bitmask);
        }
    }

    let mut sum = 0;
    for (_mem_addr, mem_value) in memory {
        sum += mem_value;
    }

    println!("Part 1: {}", sum);
}

fn set_memory_recursive(address_binary: String, value: u64) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();

    if address_binary.contains('X') {
        result.extend(set_memory_recursive(
            String::from(address_binary.replacen('X', "0", 1)),
            value,
        ));
        result.extend(set_memory_recursive(
            String::from(address_binary.replacen('X', "1", 1)),
            value,
        ));
    } else {
        let mem = u64::from_str_radix(address_binary.as_str(), 2).unwrap_or(0);
        result.insert(mem, value);
    }
    return result;
}

fn value_with_bitmask_part2(addr: u64, value: u64, bitmask: &str) -> HashMap<u64, u64> {
    let bits: String = format!("{:0>36}", format!("{:b}", addr));
    let mut result_bits: String = String::from("");
    let mut result: HashMap<u64, u64> = HashMap::new();

    for (index, char) in bitmask.chars().into_iter().enumerate() {
        let mut result_char = bits.chars().nth(index).unwrap_or('0');
        if char == '1' {
            result_char = '1';
        }
        if char == 'X' {
            result_char = 'X';
        }
        result_bits.push(result_char);
    }
    result.extend(set_memory_recursive(result_bits, value));
    return result;
}

fn part2(input: &String) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut bitmask = "";
    for line in input.lines() {
        let line_vec: Vec<&str> = line.split(" = ").collect();
        assert_eq!(line_vec.len(), 2);
        let operation = line_vec[0];
        let value = line_vec[1];

        if operation == "mask" {
            bitmask = value;
        } else {
            let addr = operation
                .replace("mem[", "")
                .replace("]", "")
                .parse::<u64>()
                .unwrap_or(0);

            memory.extend(value_with_bitmask_part2(
                addr,
                value.parse::<u64>().unwrap_or(0),
                bitmask,
            ))
        }
    }

    let mut sum = 0;
    for (_mem_addr, mem_value) in memory {
        sum += mem_value;
    }

    println!("Part 2: {}", sum);
}
