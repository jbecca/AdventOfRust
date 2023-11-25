pub mod years;

use years::year_2015::{day1, day2, day3, day4, day5};

pub fn main() {
    println!("Day 1\n=====");
    let first_answer = day1::part1();
    let second_answer = day1::part2();
    println!(
        "part 1 answer: {}\npart 2 answer: {}\n",
        first_answer, second_answer
    );

    let _ = day2::both_parts();

    let _ = day3::part1();
    let _ = day3::part2();

    let _ = day4::part1();
    let _ = day4::part2();

    let _ = day5::part1();
}

