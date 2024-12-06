use std::{fs::File, io::Read, iter};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    input_file: String,
}

fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data.trim().to_owned()
}

fn calculate_total_distance(left: &[u32], right: &[u32]) -> u32 {
    let mut total_distance: u32 = 0;

    for (left_location_id, right_location_id) in iter::zip(left, right) {
        total_distance += ((*right_location_id as i32) - (*left_location_id as i32)).abs() as u32;
    }

    total_distance
}

fn read_input_file(path: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in read_file_to_string(path).split("\n") {
        let mut split = line.split("   ");
        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
    }

    (left, right)
}

fn main() {
    let cli = Cli::parse();

    let (mut left, mut right) = read_input_file(&cli.input_file);

    left.sort();
    right.sort();

    let total_distance = calculate_total_distance(&left, &right);
    println!("Total distance: {total_distance}");
}
