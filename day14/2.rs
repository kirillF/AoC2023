use std::fs;
use std::collections::HashMap;

fn main() {
    let start = std::time::Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();

    let ans = solve2(rotate_left(convert(&data)));

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}

fn convert(data: &str) -> Vec<Vec<char>> {
    let matrix = data.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    matrix
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix = vec![];
    for i in 0..matrix[0].len() {
        let mut row = vec![];
        for j in 0..matrix.len() {
            row.push(matrix[j][i]);
        }
        new_matrix.push(row);
    }
    new_matrix
}

fn rotate_left(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix = vec![];
    for i in 0..matrix.len() {
        new_matrix.push(matrix[i].iter().rev().map(|c| *c).collect::<Vec<char>>());
    }
    new_matrix = transpose(new_matrix);
    new_matrix
}

fn rotate_right(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let matrix = transpose(matrix);
    let mut new_matrix = vec![];
    for i in 0..matrix.len() {
        new_matrix.push(matrix[i].iter().rev().map(|c| *c).collect::<Vec<char>>());
    }
    new_matrix
}

fn solve2(matrix: Vec<Vec<char>>) -> usize {
    let mut dp = vec![];
    let mut detector = HashMap::<usize, usize>::new();
    let mut tm = matrix.clone();
    let mut start_cycle = 0;
    let mut cycle_length = 0;
    for k in 0..1_000_000_000 {
        for _ in 0..4 {
            for row in &mut tm {
                let mut stop = 0;
                for i in 0..row.len() {
                    if row[i] == 'O' {
                        row.swap(stop, i);
                        stop += 1;
                    } else if row[i] == '#' {
                        stop = i + 1;
                    }             
                }
            }
            tm = rotate_right(tm);
        }
        let mut curr = 0;
        for row in &tm {
            curr += solve(row);
        }
        dp.push(curr);
        if let Some(index) = detector.get(&curr).cloned() {
            start_cycle = index as usize;
            if cycle_length == 0 {
                cycle_length = k - index;
            } else if k - index == cycle_length && cycle_length > 2 {
                if check_cycle(&dp, start_cycle, cycle_length) {
                    break;
                }
            } else {
                cycle_length = k - index;
            }
        }
        detector.insert(curr, k);
    } 
    start_cycle = start_cycle - cycle_length + 1;
    return dp[start_cycle + (1000000000 - start_cycle) % (cycle_length) - 1];
}

fn check_cycle(dp: &[usize], start: usize, length: usize) -> bool {
    for i in 0..length {
        if dp[start + i] != dp[start - length + i] {
            return false;
        }
    }
    return true;
}

fn solve(row: &[char]) -> usize {
    let mut ans = 0;
    for i in 0..row.len() {
        if row[i] == 'O' {
            ans += row.len() - i;
        }
    }
    ans
}