use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

pub fn part1() -> Result<(), Box<dyn Error>> {
    let path = "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/03.input";
    let input = read_to_string(path)?;
    let mut hmap = HashMap::new();
    let e = hmap.entry((0, 0)).or_insert(0);
    *e += 1;

    let mut x = 0;
    let mut y = 0;
    for c in input.chars() {
        match c {
            '<' => x += -1,
            '>' => x += 1,
            '^' => y += 1,
            'v' => y += -1,
            _ => (),
        }
        let house = hmap.entry((x, y)).or_insert(0);
        *house += 1
    }

    println!("Part 1: {}", hmap.keys().len());

    Ok(())
}
pub fn part2() -> Result<(), Box<dyn Error>> {
    let path = "/Users/jeffreybecca/projects/AoC/advent_of_rust/input/03.input";
    let input = read_to_string(path)?;
    let mut hmap = HashMap::new();
    let e = hmap.entry((0, 0)).or_insert(0);
    *e += 1; // santa first present
    *e += 1; // robo santa first present

    let mut x = 0;
    let mut y = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut turn = 0;
    for c in input.chars() {
        match turn % 2 {
            0 => match c {
                '<' => x += -1,
                '>' => x += 1,
                '^' => y += 1,
                'v' => y += -1,
                _ => (),
            },
            _ => match c {
                '<' => x2 += -1,
                '>' => x2 += 1,
                '^' => y2 += 1,
                'v' => y2 += -1,
                _ => (),
            },
        };
        match turn % 2 {
            0 => {
                let house = hmap.entry((x, y)).or_insert(0);
                *house += 1
            }
            _ => {
                let house = hmap.entry((x2, y2)).or_insert(0);
                *house += 1
            }
        };
        turn += 1;
    }

    println!("Part 2: {}\n", hmap.keys().len());

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
