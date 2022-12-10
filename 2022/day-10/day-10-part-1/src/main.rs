#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut values: BTreeMap<usize, isize> = BTreeMap::new();
    let mut cycle = 0;
    values.insert(1, 1);

    for line in &lines {
        if line.is_empty() {
            continue;
        }

        cycle += 1;
        let inst: Vec<&str> = line.split(' ').collect();
        let x = *values.get(&cycle).unwrap();
        match inst[0] {
            "noop" => {}
            "addx" => {
                values.insert(cycle + 1, x);
                cycle += 1;
                let rhs: isize = inst[1].parse().unwrap();
                values.insert(cycle + 1, x + rhs);
            }
            _ => unreachable!(),
        }

        if !values.contains_key(&(cycle + 1)) {
            values.insert(cycle + 1, x);
        }
    }

    let mut ans = 0;
    for (cycle, value) in values.iter().skip(19).step_by(40) {
        ans += *cycle as isize * *value;
    }

    println!("ans: {ans}");
}
