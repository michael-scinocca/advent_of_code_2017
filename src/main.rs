use std::env;

use crate::days::run_day_part;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: i32 = (args[1]).parse().unwrap();
    let part: i32 = (args[2]).parse().unwrap();

    run_day_part(day, part);
}
