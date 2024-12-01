use std::{fs::File, io::{self, BufReader}};

use clap::Parser;

mod aoc;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Options {
    day: u32,
    problem: u8,
    in_file: String,
}

fn main() -> Result<(), io::Error> {
    let opts = Options::parse();

    let in_file = File::open(opts.in_file)?;
    let input_data = BufReader::new(in_file);

    match (opts.day, opts.problem) {
        (1, 1) => aoc::day1::question1(input_data)?,
        (1, 2) => aoc::day1::question2(input_data)?,
        _ => panic!("invalid day/problem combo")
    };

    Ok(())
}
