use std::{fs::File, io::Read, path::Path};

pub fn solve() -> (u32, i32) {
    (part1(), part2())
}

fn get_puzzle_input() -> String {
    let path = Path::new("input/day3.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("konnte {} nicht öffnen: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

fn part1() -> u32 {
    let mut solution = 0;
    let s = get_puzzle_input();

    for line in s.lines() {
        let nums: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut first = 0;
        let mut first_idx = 0;
        for (i, x) in nums.iter().take(nums.len() - 1).enumerate() {
            if first < *x {
                first = *x;
                first_idx = i;
            }
        }

        let mut scnd = 0;
        for x in nums.iter().skip(first_idx + 1) {
            if scnd < *x {
                scnd = *x;
            }
        }

        solution += 10 * first + scnd;

        println!("{line}    {first}{scnd}");
    }

    solution
}

fn part2() -> i32 {
    let mut solution = 0;
    let s = get_puzzle_input();

    for line in s.lines() {}

    solution
}
