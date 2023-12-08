use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let path = lines[0];

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut starts: Vec<String> = Vec::new();   
    for line in lines[2..].iter() {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        let node = parts[0].to_string();
        let values: Vec<String> = parts[1].trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        if node.ends_with("A") {
            starts.push(node.clone());
        }
        graph.insert(node, values);
    }
    
    let mut steps_vec: Vec<usize> = Vec::new();
    for start in starts {
        let mut node = start;
        let mut steps = 0;
        let mut path_idx = 0;
        while !node.ends_with("Z") {
            let next_move = path.chars().nth(path_idx).unwrap();
            if next_move == 'L' {
                node = graph.get(&node).unwrap()[0].clone();
            } else if next_move == 'R' {
                node = graph.get(&node).unwrap()[1].clone();
            }
            steps += 1;
            path_idx = (path_idx + 1) % path.len();
        }
        steps_vec.push(steps);
    }

    let mut ans = steps_vec[0];
    for &x in steps_vec.iter() {
        ans = (ans * x) /  gcd(ans, x);
    }

    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}