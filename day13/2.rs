use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();
    
    let blocks: Vec<&str> = data.split("\n\n").collect();
    let mut ans = 0;
    for block in blocks {
        let (rows, cols) = convert(block);
        let rv = process_nums(&cols);
        if rv != 0 {
            ans += rv;
            continue;
        }

        let cv = process_nums(&rows);
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
    let rows = matrix.iter().map(|row| {
        row.iter().enumerate().fold(0, |acc, (j, &val)| acc + val * 2_i32.pow(j as u32))
    }).collect();
    let cols = (0..matrix[0].len()).map(|i| {
        (0..matrix.len()).fold(0, |acc, j| acc + matrix[j][i] * 2_i32.pow(j as u32))
    }).collect();
    (rows, cols)
}

fn process_nums(rows: &[i32]) -> usize {
    for i in 0..rows.len() {
        let mut xor = rows[i] as u32 ^ rows[rows.len() - 1] as u32;
        if xor == 0 || xor.is_power_of_two() {
            if (rows.len() + i) % 2 == 0 && check(rows, i+1, rows.len() - 2, if xor == 0 { 0 } else { 1 }) {
                return (rows.len() + i) / 2;
            }
        }
        xor = rows[rows.len() - i - 1] as u32 ^ rows[0] as u32;
        if xor == 0 || xor.is_power_of_two() {
            if (rows.len() - i) % 2 == 0 && check(rows, 1, rows.len() - i - 2, if xor == 0 { 0 } else { 1 }) {
                return (rows.len() - i) / 2;
            }
        }
    }
    0
}

fn check(rows: &[i32], mut i: usize, mut j: usize, mut xor_count: usize) -> bool {
    while i < j {
        if xor_count > 1 {
            return false;
        }
        if rows[i] != rows[j] {
            let xor = rows[i] as u32 ^ rows[j] as u32;
            if xor.is_power_of_two() && xor != 0 {
                xor_count += 1;
            } else {
                return false;
            }
        }
        i += 1;
        j -= 1;
    }
    xor_count == 1
}
