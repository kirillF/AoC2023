use std::fs;
use std::collections::HashMap;

fn main() {
    let read_to_string = fs::read_to_string("input.txt").unwrap();
    let data = read_to_string;
    let blocks = data.split("\n\n").collect::<Vec<&str>>();
    let mut seeds = blocks[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    for block in blocks[1..].iter() {
        let intervals = process_blocks(block);
        let mut new_seeds = Vec::new();
        for seed in seeds.iter() {
            let mut in_interval = false;
            for interval in intervals.iter() {
                let (start1, _) = interval.0;
                let (start2, end2) = interval.1;
                if start2 <= seed && seed <= end2 {
                    new_seeds.push(start1 + seed - start2); 
                    in_interval = true;
                }
            }
            if !in_interval {
                new_seeds.push(*seed);
            }
        }
        seeds = new_seeds;
    }

    println!("{}", seeds.iter().min().unwrap());
}

fn process_blocks(block: &str) -> HashMap<(i64, i64), (i64, i64)> {
    let sts = block.split("\n").collect::<Vec<&str>>();
    let mut intervals = HashMap::new();
    for i in 1..sts.len() {
        let st = sts[i].split_whitespace().map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let start1 = st[0];
        let start2 = st[1];
        let length = st[2];
        intervals.insert((start1, start1 + length - 1), (start2, start2 + length - 1));
    }
    intervals
}