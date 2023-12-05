
use std::fs;
use std::collections::HashMap;

fn main() {
    let read_to_string = fs::read_to_string("input.txt").unwrap();
    let data = read_to_string;
    let blocks = data.split("\n\n").collect::<Vec<&str>>();
    let seeds = blocks[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let pairs = seeds.chunks(2);
    let mut new_seeds = Vec::new();
    for pair in pairs {
        new_seeds.push((pair[0], pair[0] + pair[1] - 1));
    }
    let mut prev_intervals = new_seeds.clone();
    for block in blocks[1..].iter() {
        prev_intervals.sort_by_key(|interval| interval.0);
        let mut new_intervals = Vec::new();
        let intervals = process_blocks(block);
        let mut keys = intervals.keys().collect::<Vec<&(i64, i64)>>();
        keys.sort_by(|a, b| a.0.cmp(&b.0));
        let mut i = 0;
        for interval in prev_intervals.iter() {
            let mut new_start = interval.0;
            let new_end = interval.1;
            while i < keys.len() {
                let (start1, end1) = keys[i];
                let (start2, end2) = intervals[keys[i]];
                if new_start < *start1 && new_end < *start1 {
                    new_intervals.push((new_start, new_end));
                    break;
                } 
                if new_start < *start1 && new_end >= *start1 {
                    new_intervals.push((new_start, *start1 - 1));
                    new_start = *start1;
                } 
                if new_start >= *start1 && new_start < *end1 && new_end < *end1 {
                    new_intervals.push((start2 + new_start - *start1, start2 + new_end - *start1));
                    new_start = new_end + 1;
                }
                if new_start >= *start1 && new_start < *end1 && new_end >= *end1 {
                    new_intervals.push((start2 + new_start - *start1, end2));
                    new_start = *end1 + 1;
                }
                if new_start >= new_end {
                    break;
                }
                i += 1
            }
            if i >= keys.len() {
                new_intervals.push((new_start, new_end));
            }

        } 

        prev_intervals = new_intervals;
    }
    
    println!("{:?}", prev_intervals.iter().min_by_key(|interval| interval.0).unwrap().0);
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
        intervals.insert((start2, start2 + length - 1), (start1, start1 + length - 1));
    }
    intervals
}