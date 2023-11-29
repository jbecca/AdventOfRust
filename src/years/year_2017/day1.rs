use std::{fs::read_to_string, error::Error};
use itertools::Itertools;

fn part1() -> Result<u32, Box<dyn Error>>{
    let raw_input = read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2017/day1.input")?;
    let input = raw_input.trim();

    let mut answer = 0;
    for (a, b) in input.chars().tuple_windows() {
        if a == b {
            answer += a.to_digit(10).unwrap();
        }
    }

    if input.chars().next() == input.chars().rev().next() {
        answer += 1
    };

    Ok(answer)

}

fn part2() {

}

pub fn run_both_parts() -> Result<(), Box<dyn Error>>{
    let part1_answer = part1()?;
    println!("Part 1: {part1_answer}");
    Ok(())
}
