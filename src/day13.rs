use crate::aoc_utils::read_input;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Instant;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);

    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let arrival: u64 = input.lines().nth(0).unwrap().parse().unwrap_or(0);
    let lines: Vec<&str> = input.lines().nth(1).unwrap().split(",").collect();

    let mut closest_line = "";
    let mut diff_minutes: u64 = 9999999;
    for line in lines {
        if line != "x" {
            let line_num_ori: u64 = line.parse().unwrap();
            let mut line_num: u64 = line_num_ori;
            while line_num < arrival {
                line_num += line_num_ori;
            }
            let diff = line_num - arrival;
            if diff < diff_minutes {
                diff_minutes = diff;
                closest_line = line;
            }
        }
    }
    println!(
        "Part 1: {}",
        closest_line.parse::<u64>().unwrap() * diff_minutes
    )
}

fn part2(input: &String) {
    let lines: Vec<&str> = input.lines().nth(1).unwrap().split(",").collect();
    let mut index_offset = 0;
    let mut res = 0;
    let mut step = 1;
    for line in lines {
        if line != "x" {
            let line_num = line.parse::<u64>().unwrap();
            loop {
                if (res + index_offset) % line_num == 0 {
                    step *= line_num;
                    break;
                }
                res += step;
            }
            println!("Line {} done, {}", line_num, index_offset);
        }
        index_offset += 1;
    }

    println!("Part 2: {}", res);
}

//
// fn calc_timestamps(
//     from: u64,
//     to: u64,
//     offsets: HashMap<u64, u64>,
//     step_size: u64,
//     part2_mutex: Arc<Mutex<u64>>,
// ) {
//     let mut timestamp: u64 = from;
//     loop {
//         let mut ok_count = 0;
//         for (line, index_offset) in offsets.iter() {
//             let num = (timestamp + index_offset) / line; // number is floored we need ceil
//             if line * num == timestamp + index_offset {
//                 ok_count += 1;
//             } else {
//                 break;
//             }
//         }
//         if ok_count == offsets.len() {
//             // We found a result! Set it in the mutex
//             let mut result = part2_mutex.lock().unwrap();
//             // We're looking for the smallest result
//             if *result == 0 || timestamp < *result {
//                 println!("Result! {} {}", timestamp, result);
//                 *result = timestamp
//             }
//             break;
//         }
//         timestamp += step_size;
//         if timestamp > to {
//             break;
//         }
//     }
// }
//
// fn part2(input: &String) {
//     let start = Instant::now();
//     let mut offsets: HashMap<u64, u64> = HashMap::new();
//     let mut step_num = 0;
//     let lines: Vec<&str> = input.lines().nth(1).unwrap().split(",").collect();
//     let mut index_offset = 0;
//     for line in lines {
//         if line != "x" {
//             let line_num = line.parse::<u64>().unwrap();
//             offsets.insert(line_num, index_offset);
//             if step_num == 0 {
//                 step_num = line_num;
//             }
//         }
//         index_offset += 1;
//     }
//
//     let part2: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
//     let mut thread_offsets = 0;
//     let thread_count = 30;
//     let thread_size = step_num * 100000000;
//     loop {
//         println!(
//             "Calculating from {} ({:.2?})",
//             thread_size * thread_offsets,
//             start.elapsed()
//         );
//
//         let mut handles = vec![];
//         for i in 0..thread_count {
//             let off = offsets.clone();
//             let part2_mut = Arc::clone(&part2);
//             let handle = thread::spawn(move || {
//                 let from = thread_size * (i + thread_offsets);
//                 let to = from + thread_size - 1;
//                 calc_timestamps(from, to, off, step_num, part2_mut);
//             });
//             handles.push(handle);
//         }
//         // Wait on all threads to finish
//         for handle in handles {
//             handle.join().unwrap();
//         }
//         if *part2.lock().unwrap() > 0 {
//             break;
//         }
//         thread_offsets += thread_count;
//     }
//     println!(
//         "Part 2: {} ({:.2?})",
//         *part2.lock().unwrap(),
//         start.elapsed()
//     );
// }
