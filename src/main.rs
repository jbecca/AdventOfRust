pub mod years;

use clap::{arg, command, value_parser};

pub fn main() {
    let matches = command!()
        .arg(
            arg!([year] "Year of Advent of Code")
                .required(true)
                .value_parser(value_parser!(u16).range(2015..=2016))
        )
        .arg(
            arg!([day] "Day of code to run")
                .required(true)
                .value_parser(value_parser!(u16).range(1..=25)),
        )
        .get_matches();

    
    if let Some(year) = matches.get_one::<u16>("year") {
        println!("Year: {year}")
    }

    if let Some(day) = matches.get_one::<u16>("day") {
        println!("Day:  {day}")
    }

    let year = matches.get_one::<u16>("year").unwrap();
    let day = matches.get_one::<u16>("day").unwrap();

    println!("====================");
    match year {
        2015 => years::year_2015::run_day(&day),
        _ => println!("Invalid year")
    }
}