use crate::aoc_utils::read_input;
use regex::Regex;

const DEBUG: bool = false;

pub fn run(input_filename: &str) {
    let input = read_input(input_filename);
    let passports = input_to_passports(&input);

    part1(&passports);
    part2(&passports);
}

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

// Add this to the struct so we can `println!("{:?}", passports)`
impl std::fmt::Debug for Passport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Passport")
            .field("byr", &self.byr)
            .field("iyr", &self.iyr)
            .field("eyr", &self.eyr)
            .field("hgt", &self.hgt)
            .field("hcl", &self.hcl)
            .field("ecl", &self.ecl)
            .field("pid", &self.pid)
            .field("cid", &self.cid)
            .finish()
    }
}

fn extract_from_passport(passport_string: &str, prop_name: &str) -> Option<String> {
    let regex_string = format!("{}:([^\\s]+)", prop_name);
    let regex = Regex::new(&regex_string).unwrap();
    let capture_result = regex.captures(passport_string);
    if capture_result.is_none() {
        return None;
    }
    return Some(
        capture_result
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .trim()
            .to_string(),
    );
}

fn input_to_passports(input: &String) -> Vec<Passport> {
    // Merge passports into single strings
    let mut passports: Vec<String> = vec![];
    let mut index: usize = 0;
    passports.push(String::from(""));
    for line in input.lines() {
        if line.trim() != "" {
            if passports[index] != "" {
                passports[index].push_str(" ");
            }
            passports[index].push_str(line);
        } else {
            passports.push(String::from(""));
            index += 1;
        }
    }

    // Convert the strings into structs
    return passports
        .iter()
        .map(|passport_string| {
            let passport_str = passport_string.as_str();
            return Passport {
                byr: extract_from_passport(passport_str, "byr"),
                iyr: extract_from_passport(passport_str, "iyr"),
                eyr: extract_from_passport(passport_str, "eyr"),
                hgt: extract_from_passport(passport_str, "hgt"),
                hcl: extract_from_passport(passport_str, "hcl"),
                ecl: extract_from_passport(passport_str, "ecl"),
                pid: extract_from_passport(passport_str, "pid"),
                cid: extract_from_passport(passport_str, "cid"),
            };
        })
        .collect();
}

fn is_valid_passport1(passport: &Passport) -> bool {
    return !passport.byr.is_none()
        && !passport.iyr.is_none()
        && !passport.eyr.is_none()
        && !passport.hgt.is_none()
        && !passport.hcl.is_none()
        && !passport.ecl.is_none()
        && !passport.pid.is_none();
}

fn part1(passports: &Vec<Passport>) {
    let mut ok_count = 0;
    passports.iter().for_each(|passport| {
        if is_valid_passport1(passport) {
            ok_count += 1;
        }
    });
    {
        ok_count += 1;
    }
    println!("Part 1: {}", ok_count);
}

fn is_valid_passport2(passport: &Passport) -> bool {
    let byr = passport
        .byr
        .as_ref()
        .unwrap_or(&"0".to_string())
        .parse::<i32>()
        .unwrap_or(0);
    if byr < 1920 || byr > 2002 {
        if DEBUG {
            println!("Failed on byr {}", byr);
        }
        return false;
    }

    let iyr = passport
        .iyr
        .as_ref()
        .unwrap_or(&"0".to_string())
        .parse::<i32>()
        .unwrap_or(0);
    if iyr < 2010 || iyr > 2020 {
        if DEBUG {
            println!("Failed on iyr {}", iyr);
        }
        return false;
    }

    let eyr = passport
        .eyr
        .as_ref()
        .unwrap_or(&"0".to_string())
        .parse::<i32>()
        .unwrap_or(0);
    if eyr < 2020 || eyr > 2030 {
        if DEBUG {
            println!("Failed on eyr {}", eyr);
        }
        return false;
    }

    let hgt = passport.hgt.as_ref().unwrap();
    if hgt.contains("in") {
        let hgt_in = hgt.replace("in", "").parse::<i32>().unwrap_or(0);
        if hgt_in < 59 || hgt_in > 76 {
            if DEBUG {
                println!("Failed on hgt (in) {}", hgt_in);
            }
            return false;
        }
    } else if hgt.contains("cm") {
        let hgt_cm = hgt.replace("cm", "").parse::<i32>().unwrap_or(0);
        if hgt_cm < 150 || hgt_cm > 193 {
            if DEBUG {
                println!("Failed on hgt (cm) {}", hgt_cm);
            }
            return false;
        }
    } else {
        return false;
    }

    let hcl = passport.hcl.as_ref().unwrap();
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if !re.is_match(hcl.trim()) {
        if DEBUG {
            println!("Failed on hcl {}", hcl);
        }
        return false;
    }

    let ecl = passport.ecl.as_ref().unwrap();
    let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    if !re.is_match(ecl.trim()) {
        if DEBUG {
            println!("Failed on ecl {}", ecl);
        }
        return false;
    }

    let pid = passport.pid.as_ref().unwrap();
    let re = Regex::new(r"^\d{9}$").unwrap();
    if !re.is_match(pid.trim()) {
        if DEBUG {
            println!("Failed on pid {}", pid);
        }
        return false;
    }

    return true;
}

fn part2(passports: &Vec<Passport>) {
    let mut ok_count = 0;
    passports.iter().for_each(|passport| {
        if is_valid_passport1(passport) && is_valid_passport2(passport) {
            ok_count += 1;
        }
    });

    println!("Part 2: {}", ok_count);
}
