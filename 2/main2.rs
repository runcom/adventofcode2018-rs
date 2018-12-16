use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let f = File::open("input.txt").unwrap();
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
    println!("{}", twos * threes);
}
