use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();

    let ans = transpose(convert(&data)).iter().map(|row| solve(row)).sum::<usize>();

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

fn solve(row: &[char]) -> usize {
    let mut stop = 0;
    let mut ans = 0;
    for i in 0..row.len() {
        if row[i] == 'O' {
            ans += row.len() - stop;
            stop += 1;
        }
        if row[i] == '#' {
            stop = i + 1;
        }
    }
    ans
}
