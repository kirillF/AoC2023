
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    
    let m = lines.len();
    let n = lines[0].len();

    let mut row = vec![0; n];
    let mut col = vec![0; m];

    let mut points: Vec<(usize, usize)> = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                row[i] += 1;
                col[j] += 1;
                points.push((i, j));
            }
        }
    }

    let mut zeros = 0;
    for i in 0..n {
        if col[i] == 0 {
            zeros += 999999;
        }
        col[i] = zeros;
    }

    zeros = 0;
    for i in 0..m {
        if row[i] == 0 {
            zeros += 999999;
        }
        row[i] = zeros;
    }

    let mut ans = 0 as i64;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let (x1, y1) = (points[i].0 + row[points[i].0], points[i].1 + col[points[i].1]);
            let (x2, y2) = (points[j].0 + row[points[j].0], points[j].1 + col[points[j].1]);
            
            let dia = (x1 as i32 - x2 as i32).abs().min((y1 as i32 - y2 as i32).abs());
            let rm = (x1 as i32 - x2 as i32).abs().max((y1 as i32 - y2 as i32).abs()) - dia;

            let dist = 2 * dia + rm;
            ans += dist as i64;
        }
    }

    println!("{}", ans);

}