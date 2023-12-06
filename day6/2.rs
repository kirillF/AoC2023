use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let times = parse_values(lines[0]);
    let dist = parse_values(lines[1]);

    let time: i64 = times.iter().map(|&time| time as i64).collect::<String>().parse().unwrap();
    let dist: i64 = dist.iter().map(|&dist| dist as i64).collect::<String>().parse().unwrap();

    let s = find_s(time, dist);
    let count = time - 2 * s + 1;

    println!("{}", count);
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

fn find_s(time: i64, distance: i64) -> i64 {
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