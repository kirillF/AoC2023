use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let masks: HashMap<String, i32> = [
        ("A".to_string(), 13),
        ("K".to_string(), 12),
        ("Q".to_string(), 11),
        ("J".to_string(), 10),
        ("T".to_string(), 9),
        ("9".to_string(), 8),
        ("8".to_string(), 7),
        ("7".to_string(), 6),
        ("6".to_string(), 5),
        ("5".to_string(), 4),
        ("4".to_string(), 3),
        ("3".to_string(), 2),
        ("2".to_string(), 1),
    ].iter().cloned().collect();
    
    let mut types: Vec<Vec<(String, i32)>> = vec![Vec::new(); 7];
    for line in lines {
        let line_data: Vec<&str> = line.split_whitespace().collect();
        let hand = line_data[0].to_string();
        let t = get_type(&hand);
        types[t as usize].push((hand, line_data[1].parse::<i32>().unwrap()));
    }

    let mut count = 0;
    let mut score = 0;
    for t in types.iter_mut() {
        t.sort_by(|a, b| {
            for i in 0..a.0.len() {
                if a.0.chars().nth(i).unwrap() != b.0.chars().nth(i).unwrap() {
                    return masks[&a.0.chars().nth(i).unwrap().to_string()].cmp(&masks[&b.0.chars().nth(i).unwrap().to_string()]);
                }
            }
            std::cmp::Ordering::Equal
        });
    }
    for t in types.iter() {
        for (_, value) in t.iter() {
            count += 1;
            score += count * value;
        }
    }
    println!("{}", score);
}

fn get_type(card: &str) -> i32 {
    let mut counts = HashMap::new(); 
    for c in card.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut pairs = 0;
    let mut max_freq = 0;
    for (_, v) in &counts {
        max_freq = max_freq.max(*v);
        if *v == 2 {
            pairs += 1;
        }
    }
    match max_freq {
        5 => 6,
        4 => 5,
        3 => {
            if pairs == 1 {
                4
            } else {
                3
            }
        },
        2 => {
            if pairs == 2 {
                2
            } else {
                1
            }
        }
        _ => 0,
    }
}