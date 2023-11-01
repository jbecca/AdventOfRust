use std::fs::read_to_string;

pub fn main() -> std::io::Result<()> {
    let path = "/Users/jeffreybecca/projects/AoC/AdventOfRust/input/02.input";
    let mut total_square_feet: i32 = 0;
    let mut total_ribbon: i32 = 0;
    for line in read_to_string(path)?.lines() {
        let a: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        if let [l,w,h] = a[..] {
            let surface_area: i32 = [l*w, w*h, h*l].iter().sum();
            let smallest_perim: i32 = * [2 * (l+w), 2 * (w+h), 2 * (h+l)].iter().min().unwrap();
            let volume = l * w * h;
            total_ribbon += smallest_perim + volume;
            total_square_feet = total_square_feet + (2 * surface_area + [l*w, w*h, h*l].iter().min().unwrap());
        }
    }
    println!("{} {}", total_square_feet, total_ribbon);
    Ok(())
}
