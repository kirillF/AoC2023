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

    let mut vd = 0;
    let mut ns = 0;
    let mut bh = 0;
    let mut dl = 0;
    
    let mut k = 0;
     

    while vd == 0 || ns == 0 || bh == 0 || dl == 0 {
        k += 1;
        queue.push_back("broadcaster");
        while let Some(module) = queue.pop_front() {
            let (_, module_value) = modules.get(module).unwrap().clone();
            for connection in graph.get(module).unwrap() {
                if !modules.contains_key(&connection.to_string()) {
                    continue;
                }
                let (connection_type, connection_value) = modules.get(&connection.to_string()).unwrap().clone();
                if connection_type == "&" {
                    let m = conj.get_mut(&connection.to_string()).unwrap();
                    m.insert(module, module_value);
                    if module_value == 1 {
                        match module {
                            "vd" => vd = k,
                            "ns" => ns = k,
                            "bh" => bh = k,
                            "dl" => dl = k,
                            _ => (),
                        }
                    }
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

    let ans = lcm(vec![vd, ns, bh, dl]);

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(nums: Vec<u64>) -> u64 {
    nums.iter().fold(1, |acc, &num| acc * num / gcd(acc, num))
}