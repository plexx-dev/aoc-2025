use std::{fs::File, io::Read, ops::RangeInclusive, path::Path};

pub fn solve() -> (u128, u128) {
    (part1(), part2())
}

fn get_puzzle_input() -> String {
    let path = Path::new("input/day2.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("konnte {} nicht öffnen: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

fn part1() -> u128 {
    let mut solution = 0;
    let s = get_puzzle_input();

    let ranges: Vec<RangeInclusive<u128>> = s
        .split(',')
        .map(|x| x.split('-').map(|x| x.parse::<u128>().unwrap()).collect())
        .map(|x: Vec<u128>| x[0]..=x[1])
        .collect();

    for range in ranges {
        for x in range {
            let len = (x.checked_ilog10().unwrap_or(0) + 1) as usize;

            if len % 2 != 0 {
                continue;
            }

            let slice_lengths: usize = len / 2;
            let x_as_str = x.to_string();
            let x_slice = &x_as_str[0..slice_lengths];

            if x_as_str == x_slice.repeat(2) {
                solution += x;
            }
        }
    }

    solution
}

fn part2() -> u128 {
    let mut solution = 0;
    let s = get_puzzle_input();

    solution
}
