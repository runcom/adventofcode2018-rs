use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut solution: i32 = 0;
    let mut dup = HashMap::new();
    let mut dupfound = false;
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut lines: Vec<String> = Vec::new();
    for line in file.lines() {
        lines.push(line.unwrap());
    }
    while !dupfound {
        for l in &lines{
            let op = l.chars().next().unwrap();
            let num = &l[1..l.len()];
            let n: i32 = num.parse().unwrap();
            match op {
                '+' => solution = solution + n,
                '-' => solution = solution- n,
                _ => panic!("shouldn't happen"),
            };
            if !dupfound {
                if dup.contains_key(&solution) {
                    println!("{}", solution);
                    dupfound = true;
                } else {
                    dup.insert(solution, true);
                }
            }
        }
    }
}
