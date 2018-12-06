#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod puzzle;
use puzzle::Puzzle;

fn main() {
    let solvers: Vec<&Puzzle> = vec![
        &puzzle::FrequencyCalibration,
        &puzzle::FrequencyDuplication,
        &puzzle::ChecksumCalculation,
        &puzzle::CommonInventoryChars,
        &puzzle::OverlappingFabricClaims,
    ];

    let args: Vec<String> = env::args().collect();
    let puzzle_index: usize = args[1].parse().expect("first argument must be a number");
    let input_path: &str = &args[2];

    let mut f = File::open(input_path).expect("could not open input file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("could not read file");

    println!("puzzle {} with input from {}", puzzle_index, input_path);
    println!("result: {}", solvers[puzzle_index].solve(&contents));
}
