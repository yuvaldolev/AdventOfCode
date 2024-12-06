use std::{collections::HashMap, fs::File, io::Read};

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

fn calculate_similarity_score(left: &[u32], right: &[u32]) -> u32 {
    let mut similarity_score = 0;
    let mut location_id_scores: HashMap<u32, u32> = HashMap::new();

    for location_id in left.iter() {
        similarity_score += *location_id_scores.entry(*location_id).or_insert_with(|| {
            (right.iter().filter(|&&item| *location_id == item).count() as u32) * (*location_id)
        });
    }

    similarity_score
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

    let similarity_score = calculate_similarity_score(&left, &right);
    println!("Similarity score: {similarity_score}");
}
