use std::{fs::File, io::{self, BufRead, BufReader}};


pub fn question1(fin: BufReader<File>) -> Result<(), io::Error> {

    // step 1: parse input into 2 maps, one for left and one for right

    let mut map_left = Vec::new();
    let mut map_right = Vec::new();

    for (index, line) in fin.lines().enumerate() {
        let line = line?;

        let lr = line.split_whitespace().collect::<Vec<&str>>();

        let left = lr[0];
        let right = lr[1];

        let left = left.parse::<usize>().expect("failed to convert to usize");
        let right = right.parse::<usize>().expect("failed to convert to usize");

        map_left.push((left, index));
        map_right.push((right, index));
    }

    // step 2: sort left and right key lists so that we have the ordered values

    let (mut sorted_left_keys, _left_indexes): (Vec<_>, Vec<_>) = map_left.iter().cloned().unzip();
    let (mut sorted_right_keys, _right_indexes): (Vec<_>, Vec<_>)  = map_right.iter().cloned().unzip();

    sorted_left_keys.sort();
    sorted_right_keys.sort();


    // step 3: compute distance

    let output: usize = sorted_left_keys
        .iter()
        .zip(sorted_right_keys.iter())
        .map(|(x, y)| {
            x.abs_diff(*y)
        }).sum();

    println!("output = {}", output);

    Ok(())
}