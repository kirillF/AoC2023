use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let cards_data: HashMap<i32, i32> = data.lines()
        .enumerate()
        .map(|(index, line)| (index as i32, count_matching_cards(line)))
        .collect();

    let mut stack = (0..cards_data.len() as i32).collect::<Vec<i32>>();
    let mut count = cards_data.len() as i32;

    while let Some(card) = stack.pop() {
        let matches = cards_data[&card];
        if matches == 0 {
            continue;
        }
        count += matches;
        stack.extend((card + 1)..=(card + matches));
    }

    println!("{}", count);
}

fn count_matching_cards(line: &str) -> i32 {
    let str_numbers = line.split(":").collect::<Vec<&str>>()[1]
        .split('|').collect::<Vec<&str>>();
    let winning = str_numbers[0].split_whitespace()
        .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let on_hand = str_numbers[1].split_whitespace()
        .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    on_hand.iter()
        .filter(|&number| winning.contains(number))
        .count() as i32
}