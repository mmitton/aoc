use helper::NewRunner;
use std::collections::BTreeMap;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

pub fn register(runners: &mut BTreeMap<(usize, usize), (u8, NewRunner)>) {
    runners.insert((2020, 1), (2, || Box::new(day_01::Day01::new())));
    runners.insert((2020, 2), (2, || Box::new(day_02::Day02::new())));
    runners.insert((2020, 3), (2, || Box::new(day_03::Day03::new())));
    runners.insert((2020, 4), (2, || Box::new(day_04::Day04::new())));
    runners.insert((2020, 5), (2, || Box::new(day_05::Day05::new())));
    runners.insert((2020, 6), (2, || Box::new(day_06::Day06::new())));
    runners.insert((2020, 7), (2, || Box::new(day_07::Day07::new())));
    runners.insert((2020, 8), (2, || Box::new(day_08::Day08::new())));
    runners.insert((2020, 9), (2, || Box::new(day_09::Day09::new())));
    runners.insert((2020, 10), (2, || Box::new(day_10::Day10::new())));
    runners.insert((2020, 11), (2, || Box::new(day_11::Day11::new())));
    runners.insert((2020, 12), (2, || Box::new(day_12::Day12::new())));
    runners.insert((2020, 13), (2, || Box::new(day_13::Day13::new())));
    runners.insert((2020, 14), (2, || Box::new(day_14::Day14::new())));
    runners.insert((2020, 15), (2, || Box::new(day_15::Day15::new())));
    runners.insert((2020, 16), (2, || Box::new(day_16::Day16::new())));
    runners.insert((2020, 17), (2, || Box::new(day_17::Day17::new())));
    runners.insert((2020, 18), (2, || Box::new(day_18::Day18::new())));
    runners.insert((2020, 19), (2, || Box::new(day_19::Day19::new())));
    runners.insert((2020, 20), (2, || Box::new(day_20::Day20::new())));
    runners.insert((2020, 21), (2, || Box::new(day_21::Day21::new())));
    runners.insert((2020, 22), (2, || Box::new(day_22::Day22::new())));
    runners.insert((2020, 23), (2, || Box::new(day_23::Day23::new())));
    runners.insert((2020, 24), (2, || Box::new(day_24::Day24::new())));
    runners.insert((2020, 25), (1, || Box::new(day_25::Day25::new())));
}