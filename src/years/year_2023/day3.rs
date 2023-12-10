use std::error::Error;
use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}
fn part1() -> Result<u32, Box<dyn Error>>{
    let raw_input = read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day3.input")?;
//     let raw_input = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";
    //println!("input : {raw_input}");
    let mut symbols: Vec<Point> = Vec::new();
    let mut current_part_num: Vec<char> = Vec::new();
    let mut current_part_point: Vec<Point> = Vec::new();
    let mut part_list: Vec<u32> = Vec::new();
    let mut part_location: Vec<Vec<Point>> = Vec::new();
    let mut confirmed_parts: Vec<u32> = Vec::new();
    let mut all_lines = Vec::new();
    for line in raw_input.lines() {
        let line_of_chars = line.chars().collect::<Vec<char>>();
        all_lines.push(line_of_chars);
    }

    // parse symbols, symbol locations, parts, and part locations from input
    for i in 0..all_lines.len() {
        for j in 0..all_lines[i].len() {
            // check each character for a symbol
            if "!@#$%^&*()?><+=/-".contains(all_lines[i][j]) {
                println!("i: {i} j: {j} character: {}", all_lines[i][j]);
                // if symbol found, look above for parts if line above exists
                symbols.push(Point{x:i, y:j});
            }

            if all_lines[i][j].is_digit(10) {
                current_part_num.push(all_lines[i][j]);
                current_part_point.push(Point{x:i, y:j});
            }

            if ".!@#$%^&*()?><+=/-".contains(all_lines[i][j]) && current_part_num.len() > 0 {
                let part_number: u32 = current_part_num.iter().collect::<String>().parse::<u32>().unwrap();
                current_part_num.clear();
                part_list.push(part_number);
                part_location.push(current_part_point.clone());
                current_part_point.clear();
            }
        }
    }

    // now loop over parts and see if a symbol is near them
    // can cancel the check once far enough away since we added parts
    // and symbols to vectors from top left to bottom right in 2d space
    for (index, val) in part_list.iter().enumerate() {
        let left_lim = match part_location[index][0].y {
            0 => 0,
            z => z - 1
        };
        let upper_lim = match part_location[index][0].x {
            0 => 0,
            z => z - 1
        };
        let lower_lim = part_location[index][0].x + 1;
        let right_lim = part_location[index][part_location[index].len() - 1].y + 1;

        let mut is_part = false;
        for (symb_index, symb_point) in symbols.iter().enumerate() {
            let Point {x, y} = symb_point;
            if x >= &upper_lim && x <= &lower_lim && y <= &right_lim && y >= &left_lim { is_part = true};
        }

        if is_part {
            confirmed_parts.push(*val);
        }
    }
    let answer: u32 = confirmed_parts.iter().sum();
    println!("{:?}", confirmed_parts);
    println!("{:?}", answer);
    Ok(answer)
}

fn part2() -> Result<u32, Box<dyn Error>>{
//    let raw_input = read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day3.input")?;
     let raw_input = "467..114..
 ...*......
 ..35..633.
 ......#...
 617*......
 .....+.58.
 ..592.....
 ......755.
 ...$.*....
 .664.598..";
    //println!("input : {raw_input}");
    let mut symbols: Vec<Point> = Vec::new();
    let mut current_part_num: Vec<char> = Vec::new();
    let mut current_part_point: Vec<Point> = Vec::new();
    let mut part_list: Vec<u32> = Vec::new();
    let mut part_location: Vec<Vec<Point>> = Vec::new();
    let mut confirmed_parts: Vec<u32> = Vec::new();
    let mut all_lines = Vec::new();
    for line in raw_input.lines() {
        let line_of_chars = line.chars().collect::<Vec<char>>();
        all_lines.push(line_of_chars);
    }

    // parse symbols, symbol locations, parts, and part locations from input
    for i in 0..all_lines.len() {
        for j in 0..all_lines[i].len() {
            // check each character for a symbol
            if "!@#$%^&*()?><+=/-".contains(all_lines[i][j]) {
                println!("i: {i} j: {j} character: {}", all_lines[i][j]);
                // if symbol found, look above for parts if line above exists
                symbols.push(Point{x:i, y:j});
            }

            if all_lines[i][j].is_digit(10) {
                current_part_num.push(all_lines[i][j]);
                current_part_point.push(Point{x:i, y:j});
            }

            if ".!@#$%^&*()?><+=/-".contains(all_lines[i][j]) && current_part_num.len() > 0 {
                let part_number: u32 = current_part_num.iter().collect::<String>().parse::<u32>().unwrap();
                current_part_num.clear();
                part_list.push(part_number);
                part_location.push(current_part_point.clone());
                current_part_point.clear();
            }
        }
    }

    // now loop over parts and see if a symbol is near them
    // can cancel the check once far enough away since we added parts
    // and symbols to vectors from top left to bottom right in 2d space
    for (index, val) in part_list.iter().enumerate() {
        let left_lim = match part_location[index][0].y {
            0 => 0,
            z => z - 1
        };
        let upper_lim = match part_location[index][0].x {
            0 => 0,
            z => z - 1
        };
        let lower_lim = part_location[index][0].x + 1;
        let right_lim = part_location[index][part_location[index].len() - 1].y + 1;

        let mut is_part = false;
        for (symb_index, symb_point) in symbols.iter().enumerate() {
            let Point {x, y} = symb_point;
            if x >= &upper_lim && x <= &lower_lim && y <= &right_lim && y >= &left_lim { is_part = true};
        }

        if is_part {
            confirmed_parts.push(*val);
        }
    }
    let answer: u32 = confirmed_parts.iter().sum();
    println!("{:?}", confirmed_parts);
    println!("{:?}", answer);
    Ok(answer)
}

pub fn run_both_parts() -> Result<(), Box<dyn Error>> {
    let part1_answer = part1()?;
    let part2_answer = part2()?;
    Ok(())
}

