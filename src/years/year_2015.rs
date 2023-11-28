pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub(crate) fn run_day(day: &u16) {
    match day {
        1 => {
            day1::run_both_parts();
        },
        2 => {
            let _ = day2::both_parts();
        },
        3 => {
            let _ = day3::part1();
            let _ = day3::part2();
        },
        4 => {
            day4::part1();
            day4::part2();
        },
        5 => {
            let _ = day5::part1();
        },
        _ => println!("Invalid day"),
    }
}
