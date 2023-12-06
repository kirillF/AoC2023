use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let times: Vec<i32> = parse_values(lines[0]);
    let dist: Vec<i32> = parse_values(lines[1]);

    let mut ans = 1;
    for (time, distance) in times.iter().zip(dist.iter()) {
        let s = find_s(*time, *distance);
        ans *= time - 2 * s + 1;
    }

    println!("{}", ans);
}

fn parse_values(input: &str) -> Vec<i32> {
    input
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_s(time: i32, distance: i32) -> i32 {
    let mut s = 0;
    let mut e = time;
    while s < e {
        let mid = (s + e) / 2;
        if mid * (time - mid) <= distance {
            s = mid + 1;
        } else {
            e = mid;
        }
    }
    s
}