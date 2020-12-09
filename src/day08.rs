use crate::aoc_utils::read_input;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

#[derive(Debug)] // Added so you can debug print this enum
enum OperationType {
    Nop,
    Acc,
    Jmp,
}

#[allow(dead_code)]
struct Instruction {
    line: i32,
    operation: OperationType,
    number: i32,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Bag")
            .field("token", &self.operation)
            .field("number", &self.number)
            .finish()
    }
}

fn string_to_operation_type(in_str: &str) -> OperationType {
    let operation;
    match in_str.to_lowercase().as_ref() {
        "acc" => operation = OperationType::Acc,
        "jmp" => operation = OperationType::Jmp,
        "nop" => operation = OperationType::Nop,
        _ => {
            println!(
                "Unkown operation {}, falling back to nop",
                in_str.to_lowercase()
            );
            operation = OperationType::Nop;
        }
    }
    return operation;
}

fn read_operation(line_number: i32, line: &str) -> Instruction {
    let parts: Vec<&str> = line.trim().split(' ').collect();
    assert_eq!(parts.len(), 2, "Invalid input received");
    let operation = string_to_operation_type(parts[0]);

    return Instruction {
        line: line_number,
        operation: operation,
        number: parts[1].parse().unwrap(),
    };
}

fn run_program(instructions: Vec<Instruction>) -> (i32, i32) {
    let mut acc: i32 = 0;
    let mut line: i32 = 0;
    let mut executed: Vec<i32> = vec![];
    loop {
        let instruction = &instructions[line as usize];
        executed.push(line);
        match instruction.operation {
            OperationType::Nop => {
                line += 1;
            }
            OperationType::Acc => {
                acc += instruction.number;
                line += 1;
            }
            OperationType::Jmp => {
                line += instruction.number;
            }
        }

        if line < 0 || line > instructions.len() as i32 {
            println!("Line number went outside of program");
            break;
        }

        if executed.contains(&line) {
            break;
        }
    }
    return (acc, line);
}

fn part1(input: &String) {
    let mut line_number: i32 = 0;
    let mut instructions: Vec<Instruction> = vec![];
    for line_str in input.lines() {
        instructions.push(read_operation(line_number, line_str));
        line_number += 1;
    }

    let (acc, _last_line) = run_program(instructions);
    println!("Part 1: {}", acc);
}

fn part2(_input: &String) {}
