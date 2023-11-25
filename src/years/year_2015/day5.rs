use std::{error::Error};

pub fn part2() -> Result<(), Box<dyn Error>> {
    let input_path = "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/05.input";
    let input_string = std::fs::read_to_string(input_path)?;
    let mut total_nice = 0;
    for line in input_string.lines() {
        line.char_indices()
    }

}

pub fn part1() -> Result<(), Box<dyn Error>>{
    let input_path = "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/05.input";
    let input_string = std::fs::read_to_string(input_path)?;
    let mut total_nice = 0;
    for line in input_string.lines() {
        let vowel_count = line.chars().filter(|x| "aeiou".contains(*x)).collect::<String>().len();
        if vowel_count < 3 {continue};

        let mut double_let = false;

        let vec_of_chars = line.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < vec_of_chars.len() - 1 {
            let a = vec_of_chars[i];
            let b = vec_of_chars[i+1];
            if a == b {
                double_let = true;
            }
            i += 1;
        }
        if !double_let {continue};

        let last_check: bool;

        if line.contains("ab") {
            last_check = false
        } else if line.contains("cd") {
            last_check = false
        } else if line.contains("pq") {
            last_check = false
        } else if line.contains("xy") {
            last_check = false
        } else {
            last_check = true
        }
        if !last_check {continue};
        println!("{}, {}, {}, {}", line,vowel_count, double_let, last_check);
        total_nice += 1;
    }
    println!("Total nice: {}", total_nice);
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>>{
    part1()?;
    Ok(())
}
