use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();
    let blocks = input.split("\n\n").collect::<Vec<&str>>();

    let rules = blocks[0].lines().map(|line| {
        let parts = line.split("{").collect::<Vec<_>>();
        let key = parts[0].trim().to_string();
        let value = parts[1].trim_matches(|c| c == '{' || c == '}');
        let rule = create_rule(value);
        (key, rule)
    }).collect::<HashMap<String, Box<dyn Fn((i32, i32, i32, i32)) -> String>>>();

    let items = blocks[1].lines().map(|line| {
        let parts = line.trim_matches(|c| c == '{' || c == '}').split(",").collect::<Vec<_>>();
        (
            parts[0].split("=").nth(1).unwrap().parse::<i32>().unwrap(),
            parts[1].split("=").nth(1).unwrap().parse::<i32>().unwrap(),
            parts[2].split("=").nth(1).unwrap().parse::<i32>().unwrap(),
            parts[3].split("=").nth(1).unwrap().parse::<i32>().unwrap(),
        )
    }).collect::<Vec<(i32, i32, i32, i32)>>();

    let mut ans = 0;
    let rule = rules.get("in").unwrap();
    for item in items.iter() {
        let mut result = rule(*item);
        while result != "A" && result != "R" {
            let rule = rules.get(&result).unwrap();
            result = rule(*item);
            if result == "A" {
                ans += item.0 + item.1 + item.2 + item.3;
            } 
        }
    }

    println!("{}", ans);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn create_rule(value: &str) -> Box<dyn Fn((i32, i32, i32, i32)) -> String> {
    let conditions = value.split(",").map(|val| val.to_string()).collect::<Vec<String>>();

    Box::new(move |tuple: (i32, i32, i32, i32)| {
        let values = [("x", tuple.0), ("m", tuple.1), ("a", tuple.2), ("s", tuple.3)].iter().cloned().collect::<HashMap<_, _>>();
        for condition in conditions.iter() {
            if !condition.contains(":") {
                return condition.to_string();
            }
            let parts = condition.split(":").collect::<Vec<_>>();
            let var = &parts[0][0..1];
            let op = &parts[0][1..2];
            let val = parts[0][2..].parse::<i32>().unwrap();

            match op {
                "<" => if values.get(var).unwrap() < &val { return parts[1].to_string(); },
                ">" => if values.get(var).unwrap() > &val { return parts[1].to_string(); },
                _ => continue,
            }
        }
        "###".to_string()
    })
}