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

    let mut pointer = 50;
    for line in s.lines() {
        let (direction, amount) = line.split_at(1);

        let clicks = amount.parse::<i32>().unwrap();

        if direction == "R" {
            pointer = (pointer + clicks) % 100;
        } else {
            pointer = (pointer - clicks) % 100;
            if pointer < 0 {
                pointer = 100 + pointer;
            }
        }
        if pointer == 0 {
            solution += 1;
        }
    }

    solution
}

fn part2() -> i32 {
    let mut solution = 0;
    let s = get_puzzle_input();

    let mut pointer: i32 = 50;

    for line in s.lines() {
        let (direction, amount) = line.split_at(1);
        let clicks = amount.parse::<i32>().unwrap();
        let old = pointer;

        let first_k = if direction == "R" {
            let r = (100 - (old % 100)) % 100;
            if r == 0 { 100 } else { r }
        } else {
            let r = old % 100;
            if r == 0 { 100 } else { r }
        };

        let crosses = if clicks < first_k {
            0
        } else {
            1 + (clicks - first_k) / 100
        };

        solution += crosses;

        pointer = if direction == "R" {
            (old + clicks) % 100
        } else {
            let mut p = old - clicks;
            p %= 100;
            if p < 0 { p + 100 } else { p }
        };
    }

    solution
}


