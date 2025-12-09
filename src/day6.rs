use std::{fs::File, io::Read, path::Path};

pub fn solve() -> (u64, u64) {
    (part1(), part2())
}

fn get_puzzle_input() -> String {
    let path = Path::new("input/day6.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("konnte {} nicht öffnen: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

pub fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i].clone()).collect())
        .collect()
}

fn part1() -> u64 {
    let s = get_puzzle_input();

    let ops = s.split('\n').last().unwrap();
    let nums: Vec<Vec<u64>> = s
        .strip_suffix(&("\n".to_string() + ops))
        .unwrap()
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut ops = ops.split_whitespace().collect::<Vec<&str>>().into_iter();
    let nums = transpose(nums);

    nums.iter()
        .map(|x| {
            if ops.next().unwrap() == "+" {
                x.iter().sum::<u64>()
            } else {
                x.iter().product::<u64>()
            }
        })
        .sum::<u64>()
}

fn part2() -> u64 {
    let s = get_puzzle_input();

    let ops: &str = s.split('\n').last().unwrap();
    let mut start_indexes: Vec<usize> = vec![];
    for (pos, char) in ops.char_indices() {
        if matches!(char, '*' | '+') {
            start_indexes.push(pos);
        }
    }
    start_indexes.push(ops.len() + 1);

    let nums: Vec<Vec<String>> = s
        .strip_suffix(&("\n".to_string() + ops))
        .unwrap()
        .lines()
        .map(|line| {
            let mut strings: Vec<String> = vec![];
            let mut cur_start_index = 0;
            let iter = start_indexes.iter().skip(1);

            for next_start_index in iter {
                let gg: String = line
                    .chars()
                    .skip(cur_start_index)
                    .take((*next_start_index - 1) - cur_start_index)
                    .collect();

                cur_start_index = *next_start_index;
                strings.push(gg);
            }

            strings
        })
        .collect();

    let nums = transpose(nums);
    let nums: Vec<Vec<u64>> = nums.iter().map(|x| {
        let mut temp_vec: Vec<u64> = vec![];
        let num_len = x[0].len();
        for pos in 0..num_len {
            
            let mut num_parts: Vec<u64> = vec![];
            for part in 0..x.len() {
                let num_part = x[part].chars().nth(pos).unwrap();
                if num_part != ' ' {
                    num_parts.push(num_part.to_digit(10).unwrap() as u64);
                }
            }
            
            let mut temp_num = 0;
            for (pos , num_part) in num_parts.iter().rev().enumerate() {
                temp_num += 10_u64.pow((pos).try_into().unwrap()) * num_part;
            }

            temp_vec.push(temp_num);
        }
        temp_vec
    }).collect();

    let mut ops = ops.split_whitespace().collect::<Vec<&str>>().into_iter();
    nums.iter()
        .map(|x| {
            if ops.next().unwrap() == "+" {
                x.iter().sum::<u64>()
            } else {
                x.iter().product::<u64>()
            }
        })
        .sum::<u64>()
    
}
