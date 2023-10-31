use std::fs::read_to_string;
fn part1(input: &String) -> i32 {
    let floor = input.chars().fold(0, |acc, x| match x { '(' => acc + 1, ')' => acc - 1 , _ => acc + 0});
    floor
}
fn main() {
    let input = read_to_string("/Users/jeffreybecca/projects/AoC/AdventOfRust/input/01.input").unwrap();
    let floor = part1(&input);
    let mut floor2 = 0;
    let mut index = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor2 += 1,
            ')' => floor2 -= 1,
             _  => floor2 += 0
        };
        if floor2 < 0 {
            index = i + 1;
            break
        }
    }
    println!("{}, {}", floor, index)
}
