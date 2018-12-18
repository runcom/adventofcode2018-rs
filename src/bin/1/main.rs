use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    sol1();
    sol2();
}

fn sol1() {
    let f = File::open("inputs/1.txt").unwrap();
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
            let op = l.chars().next().unwrap();
            let num = &l[1..l.len()];
            let n: i32 = num.parse().unwrap();
            solution = match op {
                '+' => solution + n,
                '-' => solution - n,
                _ => solution + 0,
            };
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
