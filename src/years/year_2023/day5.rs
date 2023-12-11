use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn part1() -> Result<u32, Box<dyn Error>> {
    let seeds =
        read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.seeds")?
            .trim()
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
    let seeds_to_soil = read_to_string(
        "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.seeds_to_soil",
    )?
    .lines()
    .map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>();
    let soil_to_fert = read_to_string(
        "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.soil_to_fert",
    )?
    .lines()
    .map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>();
    let fert_to_water = read_to_string(
        "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.fert_to_water",
    )?
    .lines()
    .map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>();
    let water_to_light = read_to_string(
        "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.water_to_light",
    )?
    .lines()
    .map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>();
    let light_to_temp = read_to_string(
        "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.light_to_temp",
    )?
    .lines()
    .map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>();
    let temp_to_humidity = read_to_string(
        "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.temp_to_humidity",
    )?
    .lines()
    .map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>();
    let humidity_to_loc = read_to_string(
        "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day5.humidity_to_loc",
    )?
    .lines()
    .map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>();
    Ok(1)
}

pub fn run_both_parts() -> Result<(), Box<dyn Error>> {
    let part1_answer = part1()?;
    Ok(())
}
