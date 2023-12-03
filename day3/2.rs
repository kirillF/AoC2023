use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let matrix: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (-1, 1), (1, -1)];

    let mut stars_map = HashMap::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c.is_numeric() {
                let number = c.to_string();
                let mut start = vec![];
                for dir in &dirs {
                    let (x, y) = (i as isize + dir.0, j as isize + dir.1);
                    if x >= 0 && x < matrix.len() as isize && y >= 0 && y < row.len() as isize && matrix[x as usize][y as usize] == '*' {
                        start.push((x, y));
                    }
                }
                if start.len() == 1 {
                    stars_map.entry(start[0]).or_insert(vec![]).push(number.parse::<i32>().unwrap());
                }
            }
        }
    }

    let sum: i32 = stars_map.values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum();
    println!("{}", sum);
}