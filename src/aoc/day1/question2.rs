use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader}};

pub fn question2(fin: BufReader<File>) -> Result<(), io::Error> {

    // step 1: parse input into 2 maps, one for left and one for right

    let mut left_entries = Vec::new();
    let mut map_right = HashMap::new();

    for line in fin.lines() {
        let line = line?;

        let lr = line.split_whitespace().collect::<Vec<&str>>();

        let left = lr[0];
        let right = lr[1];

        let left = left.parse::<usize>().expect("failed to convert to usize");
        let right = right.parse::<usize>().expect("failed to convert to usize");

        left_entries.push(left);
        map_right.entry(right).and_modify(|y: &mut usize| *y += 1).or_insert(1);
    }


    // step 2: compute similarity1

    let output: usize = left_entries
        .iter()
        .map(|x| {
            x * map_right.get(x).unwrap_or(&0)
        }).sum();

    println!("output = {}", output);

    Ok(())
}