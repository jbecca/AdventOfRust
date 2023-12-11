use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
struct CubeColors {
    red: u32,
    green: u32,
    blue: u32,
}

fn part1() -> Result<(), Box<dyn Error>> {
    let raw_input =
        read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day2.input")?;
    let given_cubes = CubeColors {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut answer: u32 = 0;

    for line in raw_input.lines() {
        println!("line: {line}");
        let mut temp = line.split(":");
        let id = temp.next().unwrap();
        println!("id: {id}");
        let re_id = Regex::new(r"Game (\d+)")?;
        let re_blue = Regex::new(r"(\d+) blue")?;
        let re_green = Regex::new(r"(\d+) green")?;
        let re_red = Regex::new(r"(\d+) red")?;
        let Some(game_id) = re_id.captures(id) else {
            panic!("did not find game id")
        };
        let id_value = &game_id[1].parse::<u32>().unwrap();
        println!("parsed game id: {:?}", id_value);
        let pulls = temp.next().unwrap();

        let mut possible = true;
        for pull in pulls.split(";") {
            let mut current_pull_values = CubeColors {
                red: 0,
                blue: 0,
                green: 0,
            };
            if let Some(blue_count) = re_blue.captures(pull) {
                current_pull_values.blue = blue_count[1].parse::<u32>().unwrap();
            };

            if let Some(red_count) = re_red.captures(pull) {
                current_pull_values.red = red_count[1].parse::<u32>().unwrap().clone();
            };

            if let Some(green_count) = re_green.captures(pull) {
                current_pull_values.green = green_count[1].parse::<u32>().unwrap().clone();
            };
            //println!("pulled: {pull}");
            println!("parsed color pulls - {:?}", current_pull_values);
            if current_pull_values.red > given_cubes.red {
                possible = false;
            };

            if current_pull_values.blue > given_cubes.blue {
                possible = false;
            };
            if current_pull_values.green > given_cubes.green {
                possible = false;
            };
        }
        if possible {
            answer += id_value
        };
        println!("Possible? {possible}");
    }
    println!("Answer: {answer}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let raw_input =
        read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day2.input")?;

    let mut answer: u32 = 0;

    for line in raw_input.lines() {
        let mut given_cubes = CubeColors {
            red: 0,
            green: 0,
            blue: 0,
        };
        println!("line: {line}");
        let mut temp = line.split(":");
        let id = temp.next().unwrap();
        println!("id: {id}");
        let re_id = Regex::new(r"Game (\d+)")?;
        let re_blue = Regex::new(r"(\d+) blue")?;
        let re_green = Regex::new(r"(\d+) green")?;
        let re_red = Regex::new(r"(\d+) red")?;
        let Some(game_id) = re_id.captures(id) else {
            panic!("did not find game id")
        };
        let id_value = &game_id[1].parse::<u32>().unwrap();
        println!("parsed game id: {:?}", id_value);
        let pulls = temp.next().unwrap();

        let mut possible = true;
        for pull in pulls.split(";") {
            let mut current_pull_values = CubeColors {
                red: 0,
                blue: 0,
                green: 0,
            };
            if let Some(blue_count) = re_blue.captures(pull) {
                current_pull_values.blue = blue_count[1].parse::<u32>().unwrap();
            };

            if let Some(red_count) = re_red.captures(pull) {
                current_pull_values.red = red_count[1].parse::<u32>().unwrap().clone();
            };

            if let Some(green_count) = re_green.captures(pull) {
                current_pull_values.green = green_count[1].parse::<u32>().unwrap().clone();
            };
            //println!("pulled: {pull}");
            println!("parsed color pulls - {:?}", current_pull_values);
            if current_pull_values.red > given_cubes.red {
                given_cubes.red = current_pull_values.red
            };

            if current_pull_values.blue > given_cubes.blue {
                given_cubes.blue = current_pull_values.blue
            };
            if current_pull_values.green > given_cubes.green {
                given_cubes.green = current_pull_values.green
            };
        }
        answer += given_cubes.red * given_cubes.green * given_cubes.blue;
    }
    println!("Part 2 Answer: {answer}");
    Ok(())
}
pub fn run_both_parts() -> Result<(), Box<dyn Error>> {
    let answer_1 = part1()?;
    let answer_2 = part2()?;
    Ok(())
}
