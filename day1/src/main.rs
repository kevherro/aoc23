use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let lines = get_lines("src/input.txt")?;

    let part_one = calculate_part_one(&lines);
    println!("{}", part_one);

    let part_two = calculate_part_two(&lines);
    println!("{}", part_two);

    Ok(())
}

fn calculate_part_one(lines: &[String]) -> i32 {
    lines.iter().map(|line| local_sum(line)).sum()
}

fn calculate_part_two(lines: &[String]) -> i32 {
    let map = create_map();
    let rev_map = create_rev_map();

    lines.iter().fold(0, |sum, line| {
        let mut calibrated_digit = String::new();
        if let Some(value) = find_first_value(&map, line) {
            calibrated_digit.push(value);
        }
        let reversed_line: String = line.chars().rev().collect();
        if let Some(value) = find_first_value(&rev_map, &reversed_line) {
            calibrated_digit.push(value);
        }
        sum + calibrated_digit.parse::<i32>().unwrap_or(0)
    })
}

fn find_first_value(map: &HashMap<&str, char>, text: &str) -> Option<char> {
    map.iter()
        .filter_map(|(key, &value)| text.find(key).map(|index| (value, index)))
        .min_by_key(|&(_, index)| index)
        .map(|(value, _)| value)
}

fn get_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(Path::new(path))?;
    io::BufReader::new(file).lines().collect()
}

fn local_sum(line: &str) -> i32 {
    let first_digit = line.chars().find(|ch| ch.is_digit(10)).unwrap_or('0');
    let last_digit = line.chars().rev().find(|ch| ch.is_digit(10)).unwrap_or('0');
    format!("{}{}", first_digit, last_digit)
        .parse()
        .unwrap_or(0)
}

fn create_map() -> HashMap<&'static str, char> {
    [
        ("one", '1'),
        ("1", '1'),
        ("two", '2'),
        ("2", '2'),
        ("three", '3'),
        ("3", '3'),
        ("four", '4'),
        ("4", '4'),
        ("five", '5'),
        ("5", '5'),
        ("six", '6'),
        ("6", '6'),
        ("seven", '7'),
        ("7", '7'),
        ("eight", '8'),
        ("8", '8'),
        ("nine", '9'),
        ("9", '9'),
    ]
    .iter()
    .cloned()
    .collect()
}

fn create_rev_map() -> HashMap<&'static str, char> {
    [
        ("eno", '1'),
        ("1", '1'),
        ("owt", '2'),
        ("2", '2'),
        ("eerht", '3'),
        ("3", '3'),
        ("ruof", '4'),
        ("4", '4'),
        ("evif", '5'),
        ("5", '5'),
        ("xis", '6'),
        ("6", '6'),
        ("neves", '7'),
        ("7", '7'),
        ("thgie", '8'),
        ("8", '8'),
        ("enin", '9'),
        ("9", '9'),
    ]
    .iter()
    .cloned()
    .collect()
}
