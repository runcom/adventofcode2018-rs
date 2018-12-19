use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    sol1();
    sol2();
}

fn sol1() {
    let f = File::open("inputs/1.txt").unwrap();
    let file = BufReader::new(&f);
    let mut solution: i32 = 0;
    for line in file.lines() {
        let l: i32 = line.unwrap().parse().unwrap();
        solution += l;
    }
    println!("solution 1: {}", solution);
}

fn sol2() {
    let mut solution: i32 = 0;
    let mut dup = HashMap::new();
    let mut dupfound = false;
    let f = File::open("inputs/1.txt").unwrap();
    let file = BufReader::new(&f);
    let mut lines: Vec<String> = Vec::new();
    for line in file.lines() {
        lines.push(line.unwrap());
    }
    while !dupfound {
        for l in &lines {
            let delta: i32 = l.parse().unwrap();
            solution += delta;
            if !dupfound {
                if dup.contains_key(&solution) {
                    println!("solution 2: {}", solution);
                    dupfound = true;
                } else {
                    dup.insert(solution, true);
                }
            }
        }
    }
}
