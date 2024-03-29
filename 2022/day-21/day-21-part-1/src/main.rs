const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Monkey {
    Num(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut monkeys: BTreeMap<String, Monkey> = BTreeMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let name = line[0..4].to_string();
        let op = &line[6..];
        if let Some((a, b)) = op.split_once(" + ") {
            monkeys.insert(name, Monkey::Add(a.to_string(), b.to_string()));
        } else if let Some((a, b)) = op.split_once(" - ") {
            monkeys.insert(name, Monkey::Sub(a.to_string(), b.to_string()));
        } else if let Some((a, b)) = op.split_once(" * ") {
            monkeys.insert(name, Monkey::Mul(a.to_string(), b.to_string()));
        } else if let Some((a, b)) = op.split_once(" / ") {
            monkeys.insert(name, Monkey::Div(a.to_string(), b.to_string()));
        } else {
            monkeys.insert(name, Monkey::Num(op.parse().unwrap()));
        }
    }

    let mut values: BTreeMap<String, isize> = BTreeMap::new();

    while values.len() != monkeys.len() {
        let mut new_values = values.clone();
        for (name, op) in monkeys.iter() {
            if values.contains_key(name) {
                continue;
            }
            match op {
                Monkey::Num(n) => {
                    new_values.insert(name.to_owned(), *n);
                }
                Monkey::Add(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        new_values.insert(name.to_owned(), *a + *b);
                    }
                }
                Monkey::Sub(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        new_values.insert(name.to_owned(), *a - *b);
                    }
                }
                Monkey::Mul(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        new_values.insert(name.to_owned(), *a * *b);
                    }
                }
                Monkey::Div(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        new_values.insert(name.to_owned(), *a / *b);
                    }
                }
            }
        }

        values = new_values;
    }

    println!("ans: {}", values.get("root").unwrap());
}
