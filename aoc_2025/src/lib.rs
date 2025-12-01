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

pub fn register(runners: &mut BTreeMap<(usize, usize), (u8, NewRunner)>) {
    runners.insert((2025, 1), (2, || Box::new(day_01::Day01::new())));
    runners.insert((2025, 2), (2, || Box::new(day_02::Day02::new())));
    runners.insert((2025, 3), (2, || Box::new(day_03::Day03::new())));
    runners.insert((2025, 4), (2, || Box::new(day_04::Day04::new())));
    runners.insert((2025, 5), (2, || Box::new(day_05::Day05::new())));
    runners.insert((2025, 6), (2, || Box::new(day_06::Day06::new())));
    runners.insert((2025, 7), (2, || Box::new(day_07::Day07::new())));
    runners.insert((2025, 8), (2, || Box::new(day_08::Day08::new())));
    runners.insert((2025, 9), (2, || Box::new(day_09::Day09::new())));
    runners.insert((2025, 10), (2, || Box::new(day_10::Day10::new())));
    runners.insert((2025, 11), (2, || Box::new(day_11::Day11::new())));
    runners.insert((2025, 12), (1, || Box::new(day_12::Day12::new())));
}
