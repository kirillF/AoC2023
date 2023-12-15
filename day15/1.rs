use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();

    let strings = data.split(",").collect::<Vec<&str>>();

    let ans = solve(strings);

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}

fn solve(string: Vec<&str>) -> i32 {
    let mut ans = 0;
    for s in string {
        let mut curr = 0;
        for c in s.chars() {
            let ascii = c as u8;
            curr += ascii as i32;
            curr *= 17;
            curr %= 256;
        }
        ans += curr;
    }
    ans
}