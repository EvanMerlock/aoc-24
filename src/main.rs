use clap::Parser;


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Options {
    day: u32,
    problem: u8,
    in_file: String,
}

fn main() {
    let opts = Options::parse();

    match (opts.day, opts.problem) {
        (1, 1) => (),
        _ => panic!("invalid day/problem combo")
    }
}
