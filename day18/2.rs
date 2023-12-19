use std::fs;
use std::time::Instant;


fn parse_line(line: &str) -> Option<((i32, i32), i32)> {
    let mut parts = line.split_whitespace();
    let _ = parts.next()?; // Ignore the first part
    let _ = parts.next()?; // Ignore the second part
    let color_code = parts.next()?.trim_matches(|c| c == '(' || c == ')');
    let hex = &color_code[1..6];
    let dec = u32::from_str_radix(hex, 16).ok()? as i32;
    let last_digit = color_code.chars().last()?;
    let dir = match last_digit {
        '0' => (0, 1),
        '1' => (1, 0),
        '2' => (0, -1),
        '3' => (-1, 0),
        _ => return None,
    };
    Some((dir, dec))
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input
        .lines()
        .filter_map(|line| parse_line(line))
        .collect::<Vec<((i32, i32), i32)>>();

    let ans = solve(&data);

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn solve(data: &Vec<((i32, i32), i32)>) -> i64 {
    let mut border: Vec<(i32, i32)> = Vec::new();
    let mut point = (0, 0);

    let mut p = 0 as i64;

    for (dir, l) in data.iter().map(|(dir, l)| (dir, l)) {
        let (dx, dy) = dir;
        let (x, y) = point;
        let (nx, ny) = (x + l * dx, y + l * dy);
        point = (nx, ny);
        border.push(point);
        p += *l as i64;
    }

    // Shoelace formula
    let mut a = 0 as i64;
    for i in 1..border.len() + 1 {
        a += (border[i-1].0 as i64 * border[i % border.len()].1 as i64 - border[i % border.len()].0 as i64 * border[i-1].1 as i64) as i64;
    }    

    // Pick's theorem - A = i + b/2 - 1
    let ans = a.abs() / 2 + p / 2 + 1;    
    ans
}
