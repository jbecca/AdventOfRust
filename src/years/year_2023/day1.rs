use std::{error::Error, fs::read_to_string};

fn part1() -> Result<u32, Box<dyn Error>> {
    let raw_input =
        read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day1.input")?;

    let mut answer = 0;
    for line in raw_input.lines() {
        let temp = line.chars().filter(|x| x.is_digit(10)).collect::<String>();
        let first = temp.chars().next().unwrap();
        let last = temp.chars().rev().next().unwrap();
        let num = format!("{first}{last}");
        let parsed_num = num.parse::<u32>()?;
        answer += parsed_num;
    }

    Ok(answer)
}
fn part2() -> Result<u32, Box<dyn Error>> {
    let rawer_input =
        read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day1.input")?;
    let raw_input = rawer_input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    let mut answer = 0;
    for line in raw_input.lines() {
        let temp = line.chars().filter(|x| x.is_digit(10)).collect::<String>();
        let first = temp.chars().next().unwrap();
        let last = temp.chars().rev().next().unwrap();
        let num = format!("{first}{last}");
        let parsed_num = num.parse::<u32>()?;
        answer += parsed_num;
    }

    Ok(answer)
}

pub fn run_both_parts() -> Result<(), Box<dyn Error>> {
    let part1_answer = part1()?;
    println!("Part 1: {part1_answer}");
    let part2_answer = part2()?;
    println!("Part 2: {part2_answer}");
    Ok(())
}
