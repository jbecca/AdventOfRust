pub mod years;

use years::year_2015::day1;
//use years::year_2015::day2;

pub fn main() {
    println!("Day 1\n");
    let first_answer = day1::part1();
    let second_answer = day1::part2();
    println!("part 1 answer: {}\npart 2 answer: {}\n", first_answer, second_answer);

    //
 //   let first_answer2 = day2::both_parts();
 //   Ok(())

}


