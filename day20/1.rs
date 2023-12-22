use std::fs;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();

    let (graph, mut modules) = input.lines().fold((HashMap::new(), HashMap::new()), |(mut graph, mut modules), line| {
        let parts: Vec<_> = line.split(" -> ").collect();
        let (module_type, module_label) = if parts[0] == "broadcaster" {
            ("broadcaster", "broadcaster")
        } else {
            (&parts[0][..1], &parts[0][1..])
        };
        let connections: Vec<_> = parts[1].split(", ").collect();
        graph.insert(module_label.to_string(), connections);
        modules.insert(module_label.to_string(), (module_type.to_string(), 0));
        (graph, modules)
    });

    let mut conj = HashMap::<String, HashMap<&str, u32>>::new();
    for module in modules.keys() {
        if modules.get(module).unwrap().0 == "&" {
            let mut m = HashMap::new();
            for key in graph.keys() {
                if graph.get(key).unwrap().contains(&module.as_str()) {
                    m.insert(key.as_str(), 0);
                }
            }
            conj.insert(module.to_string(), m);
        }
    }

    let mut queue = VecDeque::new();

    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000 {
        lows += 1;
        queue.push_back("broadcaster");
        while let Some(module) = queue.pop_front() {
            let (_, module_value) = modules.get(module).unwrap().clone();
            for connection in graph.get(module).unwrap() {
                if module_value == 1 {
                    highs += 1;
                } else {
                    lows += 1;
                }
                if !modules.contains_key(&connection.to_string()) {
                    continue;
                }
                let (connection_type, connection_value) = modules.get(&connection.to_string()).unwrap().clone();
                if connection_type == "&" {
                    let m = conj.get_mut(&connection.to_string()).unwrap();
                    m.insert(module, module_value);
                    let flag = m.values().all(|value| *value != 0);
                    modules.insert(connection.to_string(), (connection_type, if flag { 0 } else { 1 }));
                    queue.push_back(connection);
                } else if connection_type == "%" && module_value == 0 {
                    let val = 1 ^ connection_value;
                    modules.insert(connection.to_string(), (connection_type, val));
                    queue.push_back(connection);
                }
            }
        }
    }

    let ans = lows * highs;

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}