use std::{fs::File, io::Read, path::Path};

pub fn solve() -> (u32, u128) {
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
    }

    solution
}

fn part2() -> u128 {
    let mut solution: u128 = 0;
    let s = get_puzzle_input();

    for line in s.lines() {
        let nums: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

        println!("\n\n\n{line}");
        let mut idx: Vec<usize> = vec![0];
        for pos in (1..=12).rev() {
            let n = if pos == 12 { 0 } else { *idx.last().unwrap() };

            let mut biggest = 0;
            let mut biggest_idx = 0;
            for (i, x) in nums.iter().enumerate().take(nums.len() - pos + 1).skip(n) {
                if biggest < *x {
                    biggest = *x;
                    biggest_idx = i;
                }
            }

            //println!(
            //    "{biggest} {:?}",
            //    nums.iter()
            //        .take(nums.len() - pos + 1)
            //        .skip(n)
            //        .collect::<Vec<&u32>>()
            //);

            idx.push(biggest_idx + 1);

            solution += 10_u128.pow(pos as u32 - 1) * biggest as u128;
        }
    }

    solution
}
