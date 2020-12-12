use crate::aoc_utils::read_input;

#[derive(Debug, PartialEq, Clone, Copy)]
enum SeatState {
    Ground,
    Empty,
    Occupied,
}

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    let mut input_vec: Vec<Vec<SeatState>> = vec![];
    for line in input.lines() {
        let mut row_vec: Vec<SeatState> = vec![];
        for char in line.chars() {
            match char {
                'L' => row_vec.push(SeatState::Empty),
                '#' => row_vec.push(SeatState::Occupied),
                _ => row_vec.push(SeatState::Ground),
            }
        }
        input_vec.push(row_vec);
    }

    part1(&input_vec);
    part2(&input_vec);
}

// fn print_seats(input: &Vec<Vec<SeatState>>) {
//     for row in input {
//         for seat in row {
//             match seat {
//                 SeatState::Occupied => {
//                     print!("#");
//                 }
//                 SeatState::Empty => {
//                     print!("L");
//                 }
//                 SeatState::Ground => {
//                     print!(".");
//                 }
//             }
//         }
//         println!("");
//     }
//     println!("");
// }

fn count_occupied(input: &Vec<Vec<SeatState>>) -> u32 {
    let mut count: u32 = 0;
    for row in input {
        for seat in row {
            if seat == &SeatState::Occupied {
                count += 1;
            }
        }
    }
    return count;
}

fn get_seat_visual(
    input: &Vec<Vec<SeatState>>,
    row_index: usize,
    seat_index: usize,
    x_step: i32,
    y_step: i32,
    look_max: u32, // Range of visibility, 0 for unlimited
) -> SeatState {
    let mut seat: i32 = seat_index as i32 + x_step;
    let mut row: i32 = row_index as i32 + y_step;
    let mut iterations: u32 = 0;
    loop {
        if row < 0
            || row >= input.len() as i32
            || seat < 0
            || seat >= input[row as usize].len() as i32
        {
            return SeatState::Ground;
        }
        if input[row as usize][seat as usize] != SeatState::Ground {
            return input[row as usize][seat as usize];
        }
        seat += x_step;
        row += y_step;
        iterations += 1;
        if look_max > 0 && iterations >= look_max {
            return SeatState::Ground;
        }
    }
}

fn count_neighbours_visual(
    input: &Vec<Vec<SeatState>>,
    row_index: usize,
    seat_index: usize,
    look_max: u32,
) -> u32 {
    let mut count: u32 = 0;
    count += (get_seat_visual(input, row_index, seat_index, 0, -1, look_max) == SeatState::Occupied)
        as u32;
    count += (get_seat_visual(input, row_index, seat_index, 1, -1, look_max) == SeatState::Occupied)
        as u32;
    count += (get_seat_visual(input, row_index, seat_index, 1, 0, look_max) == SeatState::Occupied)
        as u32;
    count += (get_seat_visual(input, row_index, seat_index, 1, 1, look_max) == SeatState::Occupied)
        as u32;
    count += (get_seat_visual(input, row_index, seat_index, 0, 1, look_max) == SeatState::Occupied)
        as u32;
    count += (get_seat_visual(input, row_index, seat_index, -1, 1, look_max) == SeatState::Occupied)
        as u32;
    count += (get_seat_visual(input, row_index, seat_index, -1, 0, look_max) == SeatState::Occupied)
        as u32;
    count += (get_seat_visual(input, row_index, seat_index, -1, -1, look_max)
        == SeatState::Occupied) as u32;
    return count;
}

fn run_round(input: &Vec<Vec<SeatState>>, round: u32) -> Vec<Vec<SeatState>> {
    let mut output_mut: Vec<Vec<SeatState>> = vec![];
    for (row_index, line) in input.iter().enumerate() {
        let mut row_vec: Vec<SeatState> = vec![];
        for (seat_index, seat) in line.iter().enumerate() {
            let neighbours_needed = if round == 1 { 4 } else { 5 };

            let neighbours: u32;
            if round == 1 {
                neighbours = count_neighbours_visual(input, row_index, seat_index, 1);
            } else {
                neighbours = count_neighbours_visual(input, row_index, seat_index, 0);
            };
            match seat {
                SeatState::Empty => {
                    if neighbours > 0 {
                        row_vec.push(SeatState::Empty);
                    } else {
                        row_vec.push(SeatState::Occupied);
                    }
                }
                SeatState::Occupied => {
                    if neighbours >= neighbours_needed {
                        row_vec.push(SeatState::Empty);
                    } else {
                        row_vec.push(SeatState::Occupied);
                    }
                }
                SeatState::Ground => {
                    row_vec.push(SeatState::Ground);
                }
            }
        }
        output_mut.push(row_vec);
    }
    return output_mut;
}

fn part1(input: &Vec<Vec<SeatState>>) {
    let mut output;
    output = run_round(&input, 1);
    let mut occupied: u32 = 0;
    let mut prev_occupied: u32 = 1;
    while occupied != prev_occupied {
        prev_occupied = occupied;
        output = run_round(&output, 1);
        occupied = count_occupied(&output);
    }
    println!("Part 1: {}", occupied)
}

fn part2(input: &Vec<Vec<SeatState>>) {
    let mut output;
    output = run_round(&input, 2);
    let mut occupied: u32 = 0;
    let mut prev_occupied: u32 = 1;
    while occupied != prev_occupied {
        prev_occupied = occupied;
        output = run_round(&output, 2);
        occupied = count_occupied(&output);
    }
    println!("Part 2: {}", occupied)
}
