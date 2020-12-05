# Advent of code 2020
This year i'm doing advent of code in Rust, it's a new language for me. I've started learning using the Rust "book": https://doc.rust-lang.org/book/

## Stuff i've learned
- `cargo new projectname` makes a new project
- `Cargo.toml` is the `package.json` of rust 
- IntelliJ has a free plugin for rust, that will install upon opening the project directory (https://www.jetbrains.com/rust/)
- You can enable rustfmt (search in settings) on save to format your code automatically

## Running
If you want to run this for yourself make sure to make a directory called `input` in the root of the project.
Set an environment variable called `ADVENT_SESSION` with the session cookie of adventofcode.com
If you run your day solution for the first time `main.rs` will download the input file to `input/day##.txt` automatically 

## Debugging
- Use triple shift `run cargo command` -> `cargo run 1` to run day 1, or if no day is supplied it will ask for a day
 
