use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let lines = get_lines("src/input.txt")?;

    part_one(&lines);

    part_two(&lines);

    Ok(())
}

fn part_one(cards: &[String]) {
    let mut score: usize = 0;

    for i in 0..cards.len() {
        let matches = n_matches(&cards[i]);
        if matches > 0 {
            score += 2_i32.pow(matches - 1) as usize;
        }
    }

    println!("{}", score)
}

fn part_two(cards: &[String]) {
    let mut copies = vec![1; cards.len()];

    for i in 0..cards.len() {
        let matches = n_matches(&cards[i]);
        for j in i + 1..std::cmp::min(i + 1 + matches as usize, cards.len()) {
            copies[j] += copies[i];
        }
    }

    let n_cards: usize = copies.iter().sum();

    println!("{:?}", n_cards);
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
