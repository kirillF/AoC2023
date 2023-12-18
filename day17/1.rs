use std::fs;
use std::time::Instant;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let start = Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();    
    let matrix = data
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let ans = min_cost(&matrix);

    println!("{}", ans);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn min_cost(matrix: &[Vec<i32>]) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp = vec![vec![vec![vec![i32::MAX; 4]; 4]; n]; m];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut pq = BinaryHeap::new();
    dp[0][0][0][0] = 0;
    pq.push(Reverse((dp[0][0][0][0], 0, 0, 0, 1)));
    pq.push(Reverse((dp[0][0][0][0], 0, 0, 1, 1)));

    let mut min_cost = i32::MAX;

    while let Some(Reverse((cost, x, y, dir, streak))) = pq.pop() {
        if cost > dp[x][y][dir][streak] {
            continue;
        }
        if cost > min_cost {
            continue;
        }
        if x == m - 1 && y == n - 1 {
            min_cost = min_cost.min(cost);
            continue;
        }

        for &i in [dir, (dir + 1) % 4, (dir + 3) % 4].iter() {
            let (dx, dy) = dirs[i];
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if (0..m as i32).contains(&nx) && (0..n as i32).contains(&ny) {
                let (nx, ny) = (nx as usize, ny as usize);
                let ncost = cost + matrix[nx][ny];
                let nstreak = if i == dir { streak + 1 } else { 1 };
                if nstreak <= 3 && ncost < dp[nx][ny][i][nstreak] {
                    dp[nx][ny][i][nstreak] = ncost;
                    pq.push(Reverse((ncost, nx, ny, i, nstreak)));
                }
            }
        }
    }
    min_cost
}