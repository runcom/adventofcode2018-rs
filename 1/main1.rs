use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut solution: i32 = 0;
    for line in file.lines() {
        let l = line.unwrap();
        let op = l.chars().next().unwrap();
        let num = &l[1..l.len()];
        let n: i32 = num.parse().unwrap();
        solution = match op {
            '+' => solution + n,
            '-' => solution - n,
            _ => panic!("shouldn't happen"),
        };
    }
    println!("{}", solution);
}
