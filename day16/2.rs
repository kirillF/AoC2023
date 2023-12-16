use std::fs;
use std::time::Instant;
use std::collections::HashSet;
use std::cmp::max;

fn main() {
    let start = Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();

    let matrix = data.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let ans = solve(matrix);


    println!("{}", ans);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}

fn solve(matrix: Vec<Vec<char>>) -> i32 {
    let mut ans = 0;
    for i in 0..matrix.len() {
        let l1 = dfs(&matrix, (i as i32, 0), (0, 1), &mut HashSet::new());
        let l2 = dfs(&matrix, (i as i32, matrix[0].len() as i32 - 1), (0, -1), &mut HashSet::new());
        ans = max(ans, max(l1, l2));
    }
    for j in 0..matrix[0].len() {
        let r1 = dfs(&matrix, (0, j as i32), (1, 0), &mut HashSet::new());
        let r2 = dfs(&matrix, (matrix.len() as i32 - 1, j as i32), (-1, 0), &mut HashSet::new());
        ans = max(ans, max(r1, r2));
    }
    println!("{}", ans);
    ans as i32
}

fn dfs(matrix: &Vec<Vec<char>>, node: (i32, i32), dir: (i32, i32), visited: &mut HashSet<((i32, i32), (i32, i32))>) -> i32 {
    if node.0 < 0 || node.0 >= matrix.len() as i32 || node.1 < 0 || node.1 >= matrix[0].len() as i32 || visited.contains(&(node, dir)){
        return 0;
    }
    visited.insert((node, dir));
    let mut count = 1;
    if visited.contains(&(node, (-dir.0, -dir.1))) || visited.contains(&(node, (dir.1, dir.0))) || visited.contains(&(node, (-dir.1, -dir.0))) {
        count = 0;    
    }
    match matrix[node.0 as usize][node.1 as usize] {
        '/' => {
            count += dfs(matrix, (node.0 - dir.1, node.1 - dir.0), (-dir.1, -dir.0), visited);
        }
        '\\' => {
            count += dfs(matrix, (node.0 + dir.1, node.1 + dir.0), (dir.1, dir.0), visited);  
        }
        '-' => {
            if dir.1 == 0 {
                count += dfs(matrix, (node.0, node.1 - 1), (0, -1), visited);
                count += dfs(matrix, (node.0, node.1 + 1), (0, 1), visited);
            } else {
                count += dfs(matrix, (node.0 + dir.0, node.1 + dir.1), (dir.0, dir.1), visited);
            }
        }
        '|' => {
            if dir.0 == 0 {
                count += dfs(matrix, (node.0 - 1 , node.1), (-1, 0), visited);
                count += dfs(matrix, (node.0 + 1, node.1), (1, 0), visited);
            } else {
                count += dfs(matrix, (node.0 + dir.0, node.1 + dir.1), (dir.0, dir.1), visited);
            }
        }
        '.' => {
            count += dfs(matrix, (node.0 + dir.0, node.1 + dir.1), (dir.0, dir.1), visited);
        }
        _ => {}
    }
    count
}
