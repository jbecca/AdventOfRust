pub fn part1() {
    let mut found = false;
    let mut num = 1;
    while !found {
        let t = "yzbqklnj".to_string() + &num.to_string();
        let result = md5::compute(t.as_bytes());
        let test = format!("{:x}", result);
        if &test[0..5] == "00000" {
            println!("Part 1: {}", num);
            found = true;
        }
        num += 1;
        // if num % 100000 == 0 {
        //     println!("number of iters: {}", num)
        // }
    }
}
pub fn part2() {
    let mut found = false;
    let mut num = 1;

    while !found {
        let t = "yzbqklnj".to_string() + &num.to_string();
        let result = md5::compute(t.as_bytes());
        let test = format!("{:x}", result);
        if &test[0..6] == "000000" {
            println!("Part 2: {}\n", num);
            found = true;
        }
        num += 1;
        // if num % 100000 == 0 {
        //     println!("number of iters: {}", num)
        // }
    }
}

pub fn main() {
    part1();
    part2();
}
