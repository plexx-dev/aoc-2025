mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn main() {
    let day = get_day_from_args();

    let solvers: [fn() -> Box<dyn std::fmt::Debug>; 12] = [
        || Box::new(day1::solve()),
        || Box::new(day2::solve()),
        || Box::new(day3::solve()),
        || Box::new(day4::solve()),
        || Box::new(day5::solve()),
        || Box::new(day6::solve()),
        || Box::new(day7::solve()),
        || Box::new(day8::solve()),
        || Box::new(day9::solve()),
        || Box::new(day10::solve()),
        || Box::new(day11::solve()),
        || Box::new(day12::solve()),
    ];

    println!("Day {}: {:?}", day, solvers[day - 1]());
}

fn get_day_from_args() -> usize {
    std::env::args()
        .nth(1)
        .expect("Missing day argument")
        .parse::<usize>()
        .expect("Not a number")
        .clamp(1, 12)
}
