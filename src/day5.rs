use std::{fs::File, io::Read, ops::RangeInclusive, path::Path, usize};

pub fn solve() -> (i32, usize) {
    (part1(), part2())
}

fn get_puzzle_input() -> String {
    let path = Path::new("input/day5.txt");
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
    let s: Vec<&str> = s.split("\n\n").collect();

    let ranges: Vec<RangeInclusive<usize>> = s[0]
        .lines()
        .map(|x| {
            let mut split = x.split('-');
            split.next().unwrap().parse::<usize>().unwrap()
                ..=split.next().unwrap().parse::<usize>().unwrap()
        })
        .collect();

    let ingridients: Vec<usize> = s[1].lines().map(|x| x.parse::<usize>().unwrap()).collect();

    'outer: for ingridient in ingridients {
        for range in &ranges {
            if range.contains(&ingridient) {
                solution += 1;
                continue 'outer;
            }
        }
    }

    solution
}

fn merge_many(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<usize>> = Vec::new();

    for r in ranges {
        if let Some(last) = merged.last_mut() {
            // if overlap or adjacent
            if *r.start() <= *last.end() + 1 {
                *last = *last.start()..=(*last.end()).max(*r.end());
                continue;
            }
        }
        merged.push(r);
    }

    merged
}

fn part2() -> usize {
    let s = get_puzzle_input();
    let s: Vec<&str> = s.split("\n\n").collect();

    let ranges: Vec<RangeInclusive<usize>> = s[0]
        .lines()
        .map(|x| {
            let mut split = x.split('-');
            split.next().unwrap().parse::<usize>().unwrap()
                ..=split.next().unwrap().parse::<usize>().unwrap()
        })
        .collect();

    let ranges = merge_many(ranges);
    let sum: usize = ranges.iter().map(|x| x.clone().count()).sum();

    sum
}
