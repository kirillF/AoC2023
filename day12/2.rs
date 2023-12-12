use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    
    let ans = lines.iter().map(|line| process_line(line)).sum::<i64>();

    println!("{}", ans);
}

fn process_line(line: &str) -> i64 {
    let line_data = line.split_whitespace().collect::<Vec<&str>>();
    let mut data = line_data[0].chars().collect::<Vec<char>>();
    let orig_data = data.clone();
    for _ in 0..4 {
        data.push('?');
        data.extend(orig_data.clone());
    }
    let mut groups = line_data[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let orig_groups = groups.clone();
    for _ in 0..4 {
        groups.extend(orig_groups.clone());
    }
    let mut memo = HashMap::new();
    let ans = backtrack(&data, &groups, 0, 0, &mut memo);
    ans as i64
}

fn backtrack(data: &Vec<char>, groups: &Vec<i32>, i: usize, j: usize, memo: &mut HashMap<(usize, usize), i64>) -> i64 {
    if let Some(&result) = memo.get(&(i, j)) {
        return result;
    }
    if i >= data.len() && j >= groups.len(){
        return 1;
    }
    if i >= data.len() {
        return 0;
    }
    if j >= groups.len() {
        for k in i..data.len() {
            if data[k] == '#' {
                return 0;
            }
        }
        return 1;
    }
    let mut ans = 0;
    let k = groups[j] as usize;
    if k > data.len() - i {
        return 0;
    }
    for l in i..data.len() {
        let mut m = 0;
        if data[l] == '.' {
            continue;
        }
        while m < k && l + m < data.len() {
            if data[l + m] == '?' || data[l + m] == '#' {
                m += 1;
            } else {
                break;
            }
        }
        if m == k {
            if l + m >= data.len() || (data[l + m] == '.' || data[l + m] == '?') {
                ans += backtrack(data, groups, l + m + 1, j + 1, memo);
            }
        }
        if data[l] == '#' {
            break;
        }
    }
    memo.insert((i, j), ans as i64); 
    ans
}
