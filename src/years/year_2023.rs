use std::error::Error;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub(crate) fn run_day(day: &u16) -> Result<(), Box<dyn Error>> {
    match day {
        1 => Ok(day1::run_both_parts()?),
        2 => Ok(day2::run_both_parts()?),
        3 => Ok(day3::run_both_parts()?),
        4 => Ok(day4::run_both_parts()?),
        _ => Ok(println!("Invalid day")),
    }
}
