use std::fs;
use std::collections::VecDeque;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let ans: i64 = lines
        .iter()
        .map(|line| {
            let items: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            predict(items)
        })
        .sum();

    println!("{}", ans);
}

fn predict(items: Vec<i32>) -> i64 {
    let mut queue: VecDeque<i32> = items.into_iter().rev().collect();
    let mut k = queue.len() - 1;
    let mut ans: i64 = 0;

    while k > 0 {
        let mut prev = queue.pop_front().unwrap();
        let mut zeros = 0;
        while k > 0 {
            let op = queue.pop_front().unwrap();
            let diff = op - prev;
            queue.push_back(diff);
            if diff == 0 {
                zeros += 1;
            }
            prev = op;
            k -= 1;
        }
        ans += prev as i64;
        if zeros == queue.len() {
            break;
        }
        k = queue.len() - 1;
    }
    ans
}