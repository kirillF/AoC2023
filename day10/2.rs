use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;


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
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut scaled_matris = vec![vec!['#'; matrix[0].len() * 2 - 1]; matrix.len() * 2 - 1];
    let graph = build_graph(&matrix, &allowed);
    let path = dfs(&graph, find_start(&matrix), &mut HashSet::new());
    
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            scaled_matris[i * 2][j * 2] = matrix[i][j];
        }
    }

    let m = matrix.len();
    let n = matrix[0].len();
    for i in 0..n {
        for j in 0..m {
            if matrix[j][i] == '|' || matrix[j][i] == '7' || matrix[j][i] == 'F' || matrix[j][i] == 'J' || matrix[j][i] == 'L' {
                if path.contains(&(j, i)) {
                    if j * 2 + 1 < scaled_matris.len() &&  i * 2 < scaled_matris[0].len() && matrix[j][i] != 'J' && matrix[j][i] != 'L' {
                        scaled_matris[j * 2 + 1][i * 2] = '|';
                    }
                } else {
                    scaled_matris[j * 2][i * 2] = '.'; 
                }
            }
            if matrix[j][i] == 'S' {
                if j + 1 < m && matrix[j + 1][i] == '|' || matrix[j + 1][i] == 'J' || matrix[j + 1][i] == 'L' {
                    if j * 2 + 1 < scaled_matris.len() && i * 2 < scaled_matris[0].len() {
                        scaled_matris[j * 2 + 1][i * 2] = '|';
                    }
                }
                if matrix[j - 1][i] == '|' || matrix[j - 1][i] == '7' || matrix[j - 1][i] == 'F' {
                    if j * 2 + 1 < scaled_matris.len() && i * 2 < scaled_matris[0].len() {
                        scaled_matris[j * 2 + 1][i * 2] = '|';
                    }
                }
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == '-' || matrix[i][j] == 'L' || matrix[i][j] == 'F' || matrix[i][j] == 'J' || matrix[i][j] == '7' {
                if path.contains(&(i, j)) {
                    if i * 2 < scaled_matris.len() && j * 2 + 1 < scaled_matris[0].len() && matrix[i][j] != '7' && matrix[i][j] != 'J' {
                        scaled_matris[i * 2][j * 2 + 1] = '-';
                    }
                } else {
                    scaled_matris[i * 2][j * 2] = '.'; 
                }
            }
            if matrix[i][j] == 'S' {
                if j + 1 < n && matrix[i][j + 1] == '-' || matrix[i][j + 1] == 'J' || matrix[i][j + 1] == '7' {
                    if i * 2 < scaled_matris.len() && j * 2 + 1 < scaled_matris[0].len() {
                        scaled_matris[i * 2][j * 2 + 1] = 'S';
                    }
                }
                if matrix[i][j - 1] == '-' || matrix[i][j - 1] == 'L' || matrix[i][j - 1] == 'F' {
                    if i * 2 < scaled_matris.len() && j * 2 - 1 < scaled_matris[0].len() {
                        scaled_matris[i * 2][j * 2 - 1] = '|';
                    }
                }
            }
        }
    }   

    for i in 0..m * 2 - 1 {
        if scaled_matris[i][0] == '.' || scaled_matris[i][0] == '#' {
            color(i, 0, &mut scaled_matris);
        }
        if scaled_matris[i][scaled_matris[0].len() - 1] == '.' || scaled_matris[i][scaled_matris[0].len() - 1] == '#' {
            color(i, scaled_matris[0].len() - 1, &mut scaled_matris);
        }
    }

    for i in 0..n * 2 - 1 {
        if scaled_matris[0][i] == '.' || scaled_matris[0][i] == '#' {
            color(0, i, &mut scaled_matris);
        }
        if scaled_matris[scaled_matris.len() - 1][i] == '.' || scaled_matris[scaled_matris.len() - 1][i] == '#' {
            color(scaled_matris.len() - 1, i, &mut scaled_matris);
        }
    }

    let mut count = 0;
    for i in 0..scaled_matris.len() {
        for j in 0..scaled_matris[i].len() {
            if scaled_matris[i][j] == '.' {
                count += 1;
            }
        }
    }

    println!("{}", count);

}

fn dfs(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut path = HashSet::new(); 
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(start);
    path.insert(start);
    visited.insert(start);

    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        for neighbor in graph.get(&node).unwrap() {
            if visited.contains(&neighbor) {
                continue;
            }
            stack.push(*neighbor);
            path.insert(*neighbor);
            visited.insert(*neighbor);
            break;
        }

    }
    path
}

fn build_graph(matrix: &Vec<Vec<char>>, allowed: &HashMap<char, HashSet<(i32, i32)>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
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

fn find_start(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No start found");
}

fn color(i: usize, j: usize, matrix: &mut Vec<Vec<char>>) {
    matrix[i][j] = 'X';
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let (x, y) = (i as i32 + dir.0, j as i32 + dir.1);
        if x >= 0 && y >= 0 && x < matrix.len() as i32 && y < matrix[0].len() as i32 {
            if matrix[x as usize][y as usize] == '.' || matrix[x as usize][y as usize] == '#' {
                color(x as usize, y as usize, matrix);
            }
        }
    }
}

