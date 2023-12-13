
use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();
    
    let blocks: Vec<&str> = data.split("\n\n").collect();
    let mut ans = 0;
    for block in blocks {
        let (rows, cols) = convert(block);
        let rv = process_nums(cols);
        if rv != 0 {
            ans += rv;
            continue;
        }

        let cv = process_nums(rows);
        if cv != 0 {
            ans += 100 * cv;
            continue;
        }
    }

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed)

}

fn convert(block: &str) -> (Vec<i32>, Vec<i32>) {
    let matrix = block.split("\n").map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut rows = vec![];
    for i in 0..matrix.len() {
        let mut row = 0;
        for j in 0..matrix[0].len() {
            row += matrix[i][j] * 2_i32.pow(j as u32);
        }
        rows.push(row);
    }
    let mut cols = vec![];
    for i in 0..matrix[0].len() {
        let mut col = 0;
        for j in 0..matrix.len() {
            col += matrix[j][i] * 2_i32.pow(j as u32);
        }
        cols.push(col);
    }
    (rows, cols)
}

fn process_nums(rows: Vec<i32>) -> usize {
    for i in 0..rows.len() {
        if rows[i] == rows[rows.len() - 1] && i != rows.len() - 1 {
            if (rows.len() + i) % 2 == 0 && check(&rows, i, rows.len() - 1) {
                return (rows.len() + i) / 2;
            }
        }
        if rows[rows.len() - i - 1] == rows[0] && rows.len() - i - 1 != 0 {
            if (rows.len() - i) % 2 == 0 && check(&rows, 0, rows.len() - i - 1) {
                return (rows.len() - i) / 2;
            }
        }
    }
    return 0;
}

fn check(rows: &[i32], mut i: usize, mut j: usize) -> bool {    
    while i < j {
        if rows[i] != rows[j] {
            return false
        }
        i += 1;
        j -= 1;
    }
    true
}
