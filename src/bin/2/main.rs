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
    let mut lines1: Vec<String> = Vec::new();
    let mut lines2: Vec<String> = Vec::new();
    for line in file.lines() {
        lines1.push(line.unwrap());
    }
    for line in &lines1 {
        lines2.push(line.to_string());
    }
    'outer: for l1 in &lines1 {
        for l2 in &lines2 {
            let mut diff = 0;
            let mut i = 0;
            for x in 0..l2.len() {
                if &l1.as_bytes()[x] != &l2.as_bytes()[x] {
                    diff += 1;
                    i = x;
                }
            }
            if diff == 1 {
                println!("solution 2: {}{}", &l1[0..i], &l1[i+1..l1.len()]);
                break 'outer;
            }
        }
    }
}
