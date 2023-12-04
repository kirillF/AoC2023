use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let sum: i32 = data.lines()
        .map(|line| {
            let str_numbers = line.split(":").collect::<Vec<&str>>()[1]
                .split('|').collect::<Vec<&str>>();
            let winning = str_numbers[0].split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let on_hand = str_numbers[1].split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            let winngin_map: HashMap<i32, bool> = winning.iter()
                .map(|&number| (number, true)).collect();

            let count = on_hand.iter()
                .filter(|&number| winngin_map.contains_key(number))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });

            count
        })
        .sum();

    println!("{}", sum);
}