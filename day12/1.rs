use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    
    let ans = lines.iter().map(|line| process_line(line)).sum::<i32>();

    println!("{}", ans);
}

fn process_line(line: &str) -> i32 {
    let line_data = line.split_whitespace().collect::<Vec<&str>>();
    let data = line_data[0].chars().collect::<Vec<char>>();
    let groups = line_data[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let ans = backtrack(&data, &groups, 0, 0);
    ans as i32
}

fn backtrack(data: &[char], groups: &[i32], i: usize, j: usize) -> i32 {
    if i >= data.len() && j >= groups.len() {
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
        while m < k && l + m < data.len() {
            if data[l + m] == '?' || data[l + m] == '#' {
                m += 1;
            } else {
                break;
            }
        }
        if m == k {
            if l + m >= data.len() || (data[l + m] == '.' || data[l + m] == '?') {
                ans += backtrack(data, groups, l + m + 1, j + 1);
            }
        }
        if data[l] == '#' {
            break;
        }
    }
    ans
}