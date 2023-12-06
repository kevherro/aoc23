use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let lines = get_lines("src/input.txt")?;

    part_one(&lines);

    Ok(())
}

fn part_one(cards: &[String]) {
    let mut score: usize = 0;

    for card in cards {
        let matches = n_matches(card.as_str());
        if matches > 0 {
            score += 2_i32.pow(matches - 1) as usize;
        }
    }

    println!("{}", score)
}

fn n_matches(card: &str) -> u32 {
    let parts: Vec<&str> = card.split('|').collect();

    let winning_numbers: HashSet<i32> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let my_numbers: HashSet<i32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    (&winning_numbers & &my_numbers).len() as u32
}

fn get_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(Path::new(path))?;
    io::BufReader::new(file).lines().collect()
}
