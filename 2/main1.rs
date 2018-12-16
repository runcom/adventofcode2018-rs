use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut twos = 0;
    let mut threes = 0;
    let f = File::open("input.txt").unwrap();
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
            if *count == 2  && !twoalready {
                twos += 1;
                twoalready = true;
            }
            if *count == 3 && !threealready {
                threes += 1;
                threealready = true;
            }
        }
    }
    println!("{}", twos * threes);
}
