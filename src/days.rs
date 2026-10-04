mod day1;
mod day2;

pub fn run_day_part(day: i32, part: i32) {
    match (day, part) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        _ => (),
    }
}
