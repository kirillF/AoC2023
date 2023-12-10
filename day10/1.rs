use std::fs;
use std::collections::{HashMap, HashSet};
use std::cmp::max;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let allowed: HashMap<char, HashSet<(i32, i32)>> = vec![
        ('|', [(1, 0), (-1, 0)].iter().cloned().collect()),
        ('-', [(0, 1), (0, -1)].iter().cloned().collect()),
        ('L', [(0, 1), (-1, 0)].iter().cloned().collect()),
        ('J', [(0, -1), (-1, 0)].iter().cloned().collect()),
        ('7', [(0, -1), (1, 0)].iter().cloned().collect()),
        ('F', [(0, 1), (1, 0)].iter().cloned().collect()),
        ('S', [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().cloned().collect()),
        ('.', [].iter().cloned().collect())]
        .into_iter()
        .collect();

    let matrix: Vec<Vec<char>> = data.lines()
        .map(|line| line.chars().collect())
        .collect();

    let graph = build_graph(&matrix, &allowed);
    let start = find_start(&matrix);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let max_path = dfs(&graph, start, &mut visited);  

    println!("{}", max_path / 2 + max_path % 2);
}

fn dfs(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> i32 {
    let mut max_path = 0;
    let mut stack: Vec<((usize, usize), i32)> = Vec::new();
    stack.push((start, 0));

    while let Some((node, path)) = stack.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        max_path = max(max_path, path);
        for neighbor in graph.get(&node).unwrap() {
            stack.push((*neighbor, path + 1));
        }
    }
    max_path
}

fn build_graph(matrix: &[Vec<char>], allowed: &HashMap<char, HashSet<(i32, i32)>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            for (r, s) in allowed.get(c).unwrap() {
                let (x, y) = (r + i as i32, s + j as i32);
                if x >= 0 && y >= 0 && x < matrix.len() as i32 && y < row.len() as i32 {
                    let adj = matrix[x as usize][y as usize];
                    if allowed.get(&adj).unwrap().contains(&(-r, -s)) {
                        neighbors.push((x as usize, y as usize));
                    }
                }
            }
            graph.insert((i, j), neighbors);
        }
    }
    graph
}

fn find_start(matrix: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No start found");
}
