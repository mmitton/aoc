use crate::NewRunner;
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

pub fn register(runners: &mut BTreeMap<(usize, usize), NewRunner>) {
    runners.insert((2023, 1), |part| Box::new(day_01::Day01::new(part)));
    runners.insert((2023, 2), |part| Box::new(day_02::Day02::new(part)));
    runners.insert((2023, 3), |part| Box::new(day_03::Day03::new(part)));
    runners.insert((2023, 4), |part| Box::new(day_04::Day04::new(part)));
    runners.insert((2023, 5), |part| Box::new(day_05::Day05::new(part)));
    runners.insert((2023, 6), |part| Box::new(day_06::Day06::new(part)));
    runners.insert((2023, 7), |part| Box::new(day_07::Day07::new(part)));
    runners.insert((2023, 8), |part| Box::new(day_08::Day08::new(part)));
    runners.insert((2023, 9), |part| Box::new(day_09::Day09::new(part)));
    runners.insert((2023, 10), |part| Box::new(day_10::Day10::new(part)));
    runners.insert((2023, 11), |part| Box::new(day_11::Day11::new(part)));
    runners.insert((2023, 12), |part| Box::new(day_12::Day12::new(part)));
    runners.insert((2023, 13), |part| Box::new(day_13::Day13::new(part)));
    runners.insert((2023, 14), |part| Box::new(day_14::Day14::new(part)));
    runners.insert((2023, 15), |part| Box::new(day_15::Day15::new(part)));
    runners.insert((2023, 16), |part| Box::new(day_16::Day16::new(part)));
    runners.insert((2023, 17), |part| Box::new(day_17::Day17::new(part)));
    runners.insert((2023, 18), |part| Box::new(day_18::Day18::new(part)));
    runners.insert((2023, 19), |part| Box::new(day_19::Day19::new(part)));
    runners.insert((2023, 20), |part| Box::new(day_20::Day20::new(part)));
    runners.insert((2023, 21), |part| Box::new(day_21::Day21::new(part)));
    runners.insert((2023, 22), |part| Box::new(day_22::Day22::new(part)));
    runners.insert((2023, 23), |part| Box::new(day_23::Day23::new(part)));
    runners.insert((2023, 24), |part| Box::new(day_24::Day24::new(part)));
    runners.insert((2023, 25), |part| Box::new(day_25::Day25::new(part)));
}
