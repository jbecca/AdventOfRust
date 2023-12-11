use std::error::Error;

fn part1() -> Result<u32, Box<dyn Error>> {

    let times = [42, 89, 91, 89];
    let distances = [308, 1170, 1291, 1467];
    //let times = [7, 15, 30];
    //let distances = [9, 40, 200];

    let mut wins = Vec::new();

    for (index, time) in times.iter().enumerate() {
        let mut possible_wins = 0;
        for button_hold in 1..=time.clone() {
            let time_left = time - button_hold;
            let distance_covered = button_hold * time_left;
            if distance_covered > distances[index] {
                possible_wins += 1;
            }
        }
        wins.push(possible_wins);

    }

    let answer = wins.iter().fold(1, |acc, x| acc * x);
    println!("answer - {answer}");

    Ok(1)
}
fn part2() -> Result<u64, Box<dyn Error>> {

    let times: u64 = 42899189;
    let distances: u64 = 308117012911467;


    let mut possible_wins = 0;
    for button_hold in 1..=times {
        let time_left = times - button_hold;
        let distance_covered = button_hold * time_left;
        if distance_covered > distances {
            possible_wins += 1;
        }
    }

    println!("part 2 answer - {possible_wins}");

    Ok(1)
}

pub fn run_both_parts() -> Result<(), Box<dyn Error>> {
    let part1_answer = part1()?;
    let part2_answer = part2()?;
    Ok(())
}
