use crate::aoc_utils::read_input;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

struct PlaneSeat {
    row: u32,
    col: u32,
}

fn get_seat_from_binspace(binspace: &str) -> PlaneSeat {
    assert_eq!(binspace.len(), 10, "Invalid binspace");
    let mut seat = PlaneSeat { row: 0, col: 0 };
    let mut upper_part = 64;
    for i in 0..7 {
        if binspace.chars().nth(i).unwrap() == 'B' {
            seat.row += upper_part
        }
        upper_part /= 2;
    }

    upper_part = 4;
    for i in 7..10 {
        if binspace.chars().nth(i).unwrap() == 'R' {
            seat.col += upper_part
        }
        upper_part /= 2;
    }
    return seat;
}

fn planeseat_to_seatid(seat: PlaneSeat) -> u32 {
    return seat.row * 8 + seat.col;
}

fn part1(input: &String) {
    // assert_eq!(
    //     planeseat_to_seatid(get_seat_from_binspace("BFFFBBFRRR")),
    //     567,
    //     "Test 1 failed"
    // );
    // assert_eq!(
    //     planeseat_to_seatid(get_seat_from_binspace("FFFBBBFRRR")),
    //     119,
    //     "Test 2 failed"
    // );
    // assert_eq!(
    //     planeseat_to_seatid(get_seat_from_binspace("BBFFBBFRLL")),
    //     820,
    //     "Test 3 failed"
    // );

    let mut highest = 0;
    for line in input.lines() {
        let seat_id = planeseat_to_seatid(get_seat_from_binspace(line));
        if seat_id > highest {
            highest = seat_id;
        }
    }
    println!("Part 1: {}", highest);
}

fn part2(input: &String) {
    let mut seat_id_list: Vec<u32> = vec![];
    for line in input.lines() {
        seat_id_list.push(planeseat_to_seatid(get_seat_from_binspace(line)));
    }
    seat_id_list.sort();
    let mut seat_counter = seat_id_list[0];

    for seat_id in seat_id_list {
        if seat_id != seat_counter {
            break;
        }
        seat_counter += 1;
    }
    println!("Part 2: {}", seat_counter)
}
