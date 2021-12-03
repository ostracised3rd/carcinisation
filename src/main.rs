#![allow(unused)]

mod helpers;
mod wander;
mod aoc2020;
mod aoc2021;

use crate::helpers::load_data;

fn main() {
    // aoc2020::d1::run();
    // aoc2020::d2::run();
    // aoc2020::d3::run();
    // aoc2020::d4::run();
    // aoc2020::d5::run();
    // aoc2020::d6::run();
    // aoc2020::d7::run();
    // aoc2020::d7::p2_run();
    // aoc2020::d8::run();
    // aoc2020::d9::run();

    // aoc2020::d17::run();

    // aoc2021::d01::run();
    // aoc2021::d02::run();
    println!("{}", load_data("data/aoc2021/d01.txt"));
}