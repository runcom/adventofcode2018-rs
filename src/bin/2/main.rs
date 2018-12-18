use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    sol1();
    sol2();
}

fn sol1() {
    let mut twos = 0;
    let mut threes = 0;
    let f = File::open("inputs/2.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let mut map = HashMap::new();
        let l = line.unwrap();
        let mut twoalready = false;
        let mut threealready = false;
        for c in l.chars() {
            let count = map.entry(c).or_insert(0);
            if *count != 3 {
                *count += 1;
            }
        }
        for (_, count) in &map {
            if *count == 2 && !twoalready {
                twos += 1;
                twoalready = true;
            }
            if *count == 3 && !threealready {
                threes += 1;
                threealready = true;
            }
        }
    }
    println!("solution 1: {}", twos * threes);
}

fn sol2() {
    let f = File::open("inputs/2.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let mut map = HashMap::new();
        let l = line.unwrap();
        for c in l.chars() {
            // TODO

            let count = map.entry(c).or_insert(0);
            if *count != 3 {
                *count += 1;
            }
        }
    }
    println!("solution 2: {}", "");
}
