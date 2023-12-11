use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn part1() -> Result<u32, Box<dyn Error>> {
    let raw_input =
        read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day4.input")?;
    let mut answer: i32 = 0;
    for line in raw_input.lines() {
        let mut whole_line = line.split(":");
        let game_id = whole_line.next().unwrap();
        let mut two_lists = whole_line.next().unwrap().split(" | ");
        let winners = two_lists.next().unwrap();
        let my_numbers = two_lists.next().unwrap();
        println!("id - {game_id}");
        println!("winners - {winners}");
        println!("my numbers - {my_numbers}");

        let mut vec_of_winners = Vec::new();
        for winner in winners.split(" ") {
            if !winner.is_empty() {
                vec_of_winners.push(winner.parse::<u32>());
            }
        }
        let mut matches = 0;
        for number in my_numbers.split(" ") {
            if !number.is_empty() {
                if vec_of_winners.contains(&number.parse::<u32>()) {
                    matches += 1;
                }
            }
        }
        println!("matches - {matches}");

        if matches > 0 {
            let base: i32 = 2;
            answer += (base.pow(matches - 1));
        }
    }
    println!("answer {answer}");
    Ok(1)
}

fn part2() -> Result<u32, Box<dyn Error>> {
    let raw_input =
        read_to_string("/Users/jeffreybecca/projects/AoC/advent_of_rust/input/2023/day4.input")?;
    let mut answer: HashMap<u32, u32> = HashMap::new();

    let re_id = Regex::new(r"Card[\s]+(\d+)")?;

    for line in raw_input.lines() {
        let mut whole_line = line.split(":");

        let Some(game_id) = re_id.captures(whole_line.next().unwrap()) else {
            panic!("did not find game id")
        };
        let id_value = &game_id[1].parse::<u32>().unwrap();
        answer.entry(*id_value).or_insert(1);

        let mut two_lists = whole_line.next().unwrap().split(" | ");
        let winners = two_lists.next().unwrap();
        let my_numbers = two_lists.next().unwrap();

        let mut vec_of_winners = Vec::new();
        for winner in winners.split(" ") {
            if !winner.is_empty() {
                vec_of_winners.push(winner.parse::<u32>());
            }
        }
        let mut matches = 0;
        for number in my_numbers.split(" ") {
            if !number.is_empty() {
                if vec_of_winners.contains(&number.parse::<u32>()) {
                    matches += 1;
                }
            }
        }
        println!("matches - {matches}");

        if matches > 0 {
            println!("id {:?}, matches {matches}", id_value);
            let number_of_this_card = answer.get(id_value).unwrap().clone();
            for next_card in id_value + 1..=id_value + matches {
                //println!("next card: {next_card}");
                let entry = answer.entry(next_card).or_insert(1);
                *entry += 1 * number_of_this_card;
                println!("count of next card {next_card} {entry}");
            }
        }
    }
    let mut cards = 0;
    for value in answer.values() {
        cards += value;
    }

    println!("cards: {cards}");

    Ok(1)
}
pub fn run_both_parts() -> Result<(), Box<dyn Error>> {
    //let part1_answer = part1()?;
    let part2_answer = part2()?;
    Ok(())
}
