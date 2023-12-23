use std::fs;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("test_input.txt").unwrap();
    let lines = input.lines();

    let matrix = lines.map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let s = find_start(&matrix);
    
    let ans = solve(&matrix, s, 100);

    println!("Answer: {}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn find_start(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

 
fn solve(matrix: &Vec<Vec<char>>, start: (usize, usize), steps: u32) -> u32 {
    let ans = bfs(matrix, start, steps);
    return ans.len() as u32;
} 

fn bfs(matrix: &Vec<Vec<char>>, start: (usize, usize), steps: u32) -> HashSet<(usize, usize)> {
    let mut dst: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<((usize, usize), u32)> = HashSet::new();
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue: VecDeque<((usize, usize), u32)>  = VecDeque::new();
    queue.push_back((start, 0));
    while let Some(((i, j), step)) = queue.pop_front() {
        if visited.contains(&((i, j), step)) {
            continue;
        }
        if matrix[i % matrix.len()][j % matrix[0].len()] == '#' {

            continue;
        }
        if step == steps {
            dst.insert((i, j));
            continue;
        }
        visited.insert(((i, j), step));
        for (di, dj) in dirs.iter() {
            let (ni, nj) = (i as i32 + di + matrix.len() as i32, j as i32 + dj + matrix[0].len() as i32);
            queue.push_back(((ni as usize, nj as usize), step + 1));
        }
    }
    return dst;
}
