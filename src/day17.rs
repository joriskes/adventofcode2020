use crate::aoc_utils::read_input;
use std::collections::HashMap;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

fn print_cubes(cubes: &HashMap<(i32, i32, i32), bool>) {
    for z in -6..=6 {
        println!("Z {}", z);
        for y in -6..=14 {
            for x in -6..=14 {
                if cubes.contains_key(&(x, y, z)) && cubes.get(&(x, y, z)).unwrap() == &true {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn count_neighbours(cubes: &HashMap<(i32, i32, i32), bool>, x: i32, y: i32, z: i32) -> i32 {
    let mut fx = x - 1;
    let mut fy = y - 1;
    let mut fz = z - 1;

    let mut count = 0;

    while fz <= z + 1 {
        while fy <= y + 1 {
            while fx <= x + 1 {
                if fx != x || fy != y || fz != z {
                    if cubes.contains_key(&(fx, fy, fz))
                        && cubes.get(&(fx, fy, fz)).unwrap() == &true
                    {
                        count += 1;
                    }
                }
                fx += 1;
            }
            fy += 1;
            fx = x - 1;
        }
        fz += 1;
        fy = y - 1;
        fx = x - 1;
    }

    return count;
}

fn run_cycle(cubes: &HashMap<(i32, i32, i32), bool>) -> HashMap<(i32, i32, i32), bool> {
    let mut new_cubes: HashMap<(i32, i32, i32), bool> = HashMap::new();

    for z in -6..=6 {
        for y in -6..=14 {
            for x in -6..=14 {
                let neighbour_count = count_neighbours(cubes, x, y, z);
                if cubes.contains_key(&(x, y, z)) && cubes.get(&(x, y, z)).unwrap() == &true {
                    if neighbour_count == 2 || neighbour_count == 3 {
                        // Cube is on and it stays on
                        let c = new_cubes.entry((x, y, z)).or_insert(true);
                        *c = true;
                    }
                } else {
                    // Cube is off, see if it needs to be on
                    if neighbour_count == 3 {
                        let c = new_cubes.entry((x, y, z)).or_insert(true);
                        *c = true;
                    }
                }
            }
        }
    }

    return new_cubes;
}

fn part1(input: &String) {
    let mut cubes: HashMap<(i32, i32, i32), bool> = HashMap::new();
    for (y, line) in input.lines().into_iter().enumerate() {
        for (x, char) in line.chars().into_iter().enumerate() {
            if char == '#' {
                cubes.insert((x as i32, y as i32, 0), true);
            }
        }
    }
    print_cubes(&cubes);

    for cycle in 0..6 {
        cubes = run_cycle(&mut cubes);
        println!("After cycle {}", cycle);
        // print_cubes(&cubes);
    }

    let mut enabled_count = 0;
    for (_pos, enabled) in cubes {
        if enabled == true {
            enabled_count += 1;
        }
    }
    println!("Part 1: {}", enabled_count);
}

fn count_neighbours_4d(
    cubes: &HashMap<(i32, i32, i32, i32), bool>,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
) -> i32 {
    let mut fx = x - 1;
    let mut fy = y - 1;
    let mut fz = z - 1;
    let mut fw = w - 1;

    let mut count = 0;

    while fw <= w + 1 {
        while fz <= z + 1 {
            while fy <= y + 1 {
                while fx <= x + 1 {
                    if fx != x || fy != y || fz != z || fw != w {
                        if cubes.contains_key(&(fx, fy, fz, fw))
                            && cubes.get(&(fx, fy, fz, fw)).unwrap() == &true
                        {
                            count += 1;
                        }
                    }
                    fx += 1;
                }
                fy += 1;
                fx = x - 1;
            }
            fz += 1;
            fy = y - 1;
            fx = x - 1;
        }
        fw += 1;
        fz = z - 1;
        fy = y - 1;
        fx = x - 1;
    }

    return count;
}

fn run_cycle_4d(
    cubes: &HashMap<(i32, i32, i32, i32), bool>,
) -> HashMap<(i32, i32, i32, i32), bool> {
    let mut new_cubes: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();

    for w in -6..=6 {
        for z in -6..=6 {
            for y in -6..=14 {
                for x in -6..=14 {
                    let neighbour_count = count_neighbours_4d(cubes, x, y, z, w);
                    if cubes.contains_key(&(x, y, z, w))
                        && cubes.get(&(x, y, z, w)).unwrap() == &true
                    {
                        if neighbour_count == 2 || neighbour_count == 3 {
                            // Cube is on and it stays on
                            let c = new_cubes.entry((x, y, z, w)).or_insert(true);
                            *c = true;
                        }
                    } else {
                        // Cube is off, see if it needs to be on
                        if neighbour_count == 3 {
                            let c = new_cubes.entry((x, y, z, w)).or_insert(true);
                            *c = true;
                        }
                    }
                }
            }
        }
    }
    return new_cubes;
}

fn part2(input: &String) {
    let mut cubes: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();
    for (y, line) in input.lines().into_iter().enumerate() {
        for (x, char) in line.chars().into_iter().enumerate() {
            if char == '#' {
                cubes.insert((x as i32, y as i32, 0, 0), true);
            }
        }
    }

    for cycle in 0..6 {
        cubes = run_cycle_4d(&mut cubes);
        println!("After cycle {}", cycle);
    }

    let mut enabled_count = 0;
    for (_pos, enabled) in cubes {
        if enabled == true {
            enabled_count += 1;
        }
    }
    println!("Part 2: {}", enabled_count);
}
