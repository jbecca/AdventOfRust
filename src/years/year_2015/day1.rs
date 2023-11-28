use std::fs::read_to_string;

fn read_input() -> String {
    read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/01.input").unwrap()
}
pub fn part1() -> i32 {
    let input = read_input();
    let floor = input.chars().fold(0, |acc, x| match x {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc + 0,
    });
    floor
}

pub fn part2() -> usize {
    let mut floor2 = 0;
    let mut index = 0;
    let input = read_input();
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor2 += 1,
            ')' => floor2 -= 1,
            _ => floor2 += 0,
        };
        if floor2 < 0 {
            index = i + 1;
            break;
        }
    }
    index
}

pub fn run_both_parts() -> () {
    let part1 = part1();
    let part2 = part2();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
