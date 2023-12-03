use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let matrix: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut sum = 0;

    for i in 0..140 {
        let mut j = 0;
        while j < 140 {
            if matrix[i][j].is_numeric() {
                let mut number = String::new();
                let mut is_valid = false;

                while j < 140 && matrix[i][j].is_numeric() {
                    number.push(matrix[i][j]);

                    if !is_valid {
                        is_valid = dirs.iter().any(|dir| {
                            let (x, y) = (i as isize + dir.0, j as isize + dir.1);
                            x >= 0 && x < 140 && y >= 0 && y < 140 && !matrix[x as usize][y as usize].is_numeric() && matrix[x as usize][y as usize] != '.'
                        });
                    }
                    j += 1;
                }

                if is_valid {
                    sum += number.parse::<i32>().unwrap();
                }
            }
            j += 1;
        }
    }
    println!("{}", sum);
}