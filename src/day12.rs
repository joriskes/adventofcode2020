use crate::aoc_utils::read_input;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

struct Entity {
    x: i32,
    y: i32,
    r: i32,
}

fn part1(input: &String) {
    let mut ship = Entity {
        x: 0,
        y: 0,
        r: 90, // Ship starts facing east
    };
    for line in input.lines() {
        let mut dir: char = line.chars().take(1).next().unwrap();
        let val_string: String = line.chars().skip(1).collect();
        let val: i32 = val_string.parse().unwrap();

        if dir == 'F' {
            ship.r = (ship.r + 360) % 360;
            match ship.r {
                0 => dir = 'N',
                90 => dir = 'E',
                180 => dir = 'S',
                270 => dir = 'W',
                _ => {
                    dir = 'N';
                    println!("Unsupported ship rotation {}", ship.r);
                }
            }
        }

        match dir {
            'N' => {
                ship.y -= val;
            }
            'S' => {
                ship.y += val;
            }
            'E' => {
                ship.x += val;
            }
            'W' => {
                ship.x -= val;
            }
            'L' => {
                ship.r -= val;
            }
            'R' => {
                ship.r += val;
            }
            _ => {
                println!("Unkown direction {}", dir);
            }
        }
    }

    println!("Part 1: {}", ship.x.abs() + ship.y.abs());
}

fn part2(input: &String) {
    let mut ship = Entity { x: 0, y: 0, r: 0 };
    let mut waypoint = Entity { x: 10, y: -1, r: 0 };
    for line in input.lines() {
        let dir: char = line.chars().take(1).next().unwrap();
        let val_string: String = line.chars().skip(1).collect();
        let val: i32 = val_string.parse().unwrap();

        match dir {
            'N' => {
                waypoint.y -= val;
            }
            'S' => {
                waypoint.y += val;
            }
            'E' => {
                waypoint.x += val;
            }
            'W' => {
                waypoint.x -= val;
            }
            'L' => match val {
                90 => {
                    let q = waypoint.y;
                    waypoint.y = waypoint.x;
                    waypoint.x = q * -1;
                    waypoint.x *= -1;
                    waypoint.y *= -1;
                }
                180 => {
                    waypoint.x *= -1;
                    waypoint.y *= -1;
                }
                270 => {
                    let q = waypoint.y;
                    waypoint.y = waypoint.x;
                    waypoint.x = q * -1;
                }
                _ => {
                    println!("Unsupported waypoint rotation {}", waypoint.r);
                }
            },
            'R' => match val {
                90 => {
                    let q = waypoint.y;
                    waypoint.y = waypoint.x;
                    waypoint.x = q * -1;
                }
                180 => {
                    waypoint.x *= -1;
                    waypoint.y *= -1;
                }
                270 => {
                    let q = waypoint.y;
                    waypoint.y = waypoint.x;
                    waypoint.x = q * -1;
                    waypoint.x *= -1;
                    waypoint.y *= -1;
                }
                _ => {
                    println!("Unsupported waypoint rotation {}", waypoint.r);
                }
            },
            'F' => {
                ship.x += waypoint.x * val;
                ship.y += waypoint.y * val;
            }
            _ => {
                println!("Unkown direction {}", dir);
            }
        }
    }
    println!("Part 2: {}", ship.x.abs() + ship.y.abs());
}
