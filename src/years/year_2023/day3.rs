use std::error::Error;
use std::fs::read_to_string;

fn part1() -> Result<u32, Box<dyn Error>>{
    let raw_input = read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day3.input")?;
    //println!("input : {raw_input}");
    let mut all_lines = Vec::new();
    for line in raw_input.lines() {
        let line_of_chars = line.chars().collect::<Vec<char>>();
        all_lines.push(line_of_chars);
    }
    println!("number of lines: {}", all_lines.len());

    println!("hopefully line 1: {:?}", all_lines[0]);
    Ok(1)
}

pub fn run_both_parts() -> Result<(), Box<dyn Error>> {
    let part1_answer = part1()?;
    Ok(())
}

