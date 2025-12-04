use std::{fs::File, io::Read, path::Path};

pub fn solve() -> (i32, i32) {
    (part1(), part2())
}

fn get_puzzle_input() -> String {
    let path = Path::new("input/day4.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("konnte {} nicht öffnen: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

fn check_space_around(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let mut amount = 0;
    for i in -1isize..=1 {
        if y == 0 && i == -1 || y == grid.len() - 1 && i == 1 {
            continue;
        }
        if x > 0 && grid[(y as isize + i) as usize][x - 1] {
            amount += 1;
        }
        if x < grid[y].len() - 1 && grid[(y as isize + i) as usize][x + 1] {
            amount += 1;
        }
        if i != 0 && grid[(y as isize + i) as usize][x] {
            amount += 1;
        }
    }

    if amount < 4 { true } else { false }
}

fn part1() -> i32 {
    let mut solution = 0;
    let s = get_puzzle_input();

    let grid: Vec<Vec<bool>> = s
        .lines()
        .map(|x| {
            x.chars()
                .map(|char| if char == '.' { false } else { true })
                .collect()
        })
        .collect();

    for (y, row) in grid.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            if *field && check_space_around(&grid, x, y) {
                solution += 1;
            }
        }
    }

    solution
}

fn part2() -> i32 {
    let mut solution = 0;
    let s = get_puzzle_input();

    let mut grid: Vec<Vec<bool>> = s
        .lines()
        .map(|x| {
            x.chars()
                .map(|char| if char == '.' { false } else { true })
                .collect()
        })
        .collect();

    let mut last_solution = -1;
    while last_solution != solution {
        last_solution = solution;
        let last_grid = grid.clone();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if last_grid[y][x] && check_space_around(&last_grid, x, y) {
                    grid[y][x] = false;
                    solution += 1;
                }
            }
        }
    }

    solution
}
