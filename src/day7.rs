use std::{fs::File, io::Read, path::Path};

pub fn solve() -> (i32, i32) {
    (part1(), part2())
}

fn get_puzzle_input() -> String {
    let path = Path::new("input/day1.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("konnte {} nicht öffnen: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

fn part1() -> i32 {
    let mut solution = 0;
    let s = get_puzzle_input();

    for line in s.lines() {
        solution += 1;
        line.contains(" ");
    }

    solution
}

fn part2() -> i32 {
    let mut solution = 0;
    let s = get_puzzle_input();

    for line in s.lines() {
        solution += 1;
        line.contains(" ");
    }

    solution
}
