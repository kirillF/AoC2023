
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
    }).collect::<HashMap<String, Box<dyn Fn(((i32, i32), (i32, i32), (i32, i32), (i32, i32))) -> Vec<(String, ((i32, i32), (i32, i32), (i32, i32), (i32, i32)))>>>>();

    let s = ((1,4000), (1,4000), (1,4000), (1,4000));
    let mut ans = 0;
    let mut stack = vec![rules.get("in").unwrap()(s)];
    let mut accepted = Vec::new();

    while let Some(ref result) = stack.pop() {
        for (key, value) in result.iter() {
            if key == "A" {
                accepted.push(value.clone());
            } else if key != "R" {
                stack.push(rules.get(key).unwrap()(*value));
            }
        }
    }

    for item in accepted.iter() {
        let total = ((item.0 .1 - item.0 .0 + 1) as i64)
            * ((item.1 .1 - item.1 .0 + 1) as i64)
            * ((item.2 .1 - item.2 .0 + 1) as i64)
            * ((item.3 .1 - item.3 .0 + 1) as i64);
        ans += total;
    }

    println!("{}", ans);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn create_rule(value: &str) -> Box<dyn Fn(((i32, i32), (i32, i32), (i32, i32), (i32, i32))) -> Vec<(String, ((i32, i32), (i32, i32), (i32, i32), (i32, i32)))>> {
    let conditions = value.split(",").map(|val| val.to_string()).collect::<Vec<String>>();

    Box::new(move |tuple: ((i32, i32), (i32, i32), (i32, i32), (i32, i32))| {
        let mut values = [("x", tuple.0), ("m", tuple.1), ("a", tuple.2), ("s", tuple.3)].iter().cloned().collect::<HashMap<_, _>>();
        let mut result = Vec::new();
        for condition in conditions.iter() {
            if !condition.contains(":") {
                let out = hashmap_to_tuple(&values); 
                result.push((condition.to_string(), out));
                continue;
            }
            let parts = condition.split(":").collect::<Vec<_>>();
            let var = &parts[0][0..1];
            let op = &parts[0][1..2];
            let val = parts[0][2..].parse::<i32>().unwrap();
            let value = values.get(var).unwrap().clone();
            match op {
                "<" => {
                    if value.1 < val {
                        let out = hashmap_to_tuple(&values); 
                        result.push((parts[1].to_string(), out));
                    } else if value.1 >= val && value.0 < val {
                        values.insert(var, (value.0, val - 1));
                        let out = hashmap_to_tuple(&values);
                        values.insert(var, (val, value.1));
                        result.push((parts[1].to_string(), out));
                    } 
                },
                ">" => {
                    if value.0 > val {
                        let out = hashmap_to_tuple(&values);
                        result.push((parts[1].to_string(), out));
                    } else if value.0 <= val && value.1 > val {
                        values.insert(var, (val + 1, value.1));
                        let out = hashmap_to_tuple(&values);
                        result.push((parts[1].to_string(), out));
                        values.insert(var, (value.0, val));
                    }
                },
                _ => continue,
            }
        }
        result
    })
}

fn hashmap_to_tuple(values: &HashMap<&str, (i32, i32)>) -> ((i32, i32), (i32, i32), (i32, i32), (i32, i32)) {
    (
        *values.get("x").unwrap(),
        *values.get("m").unwrap(),
        *values.get("a").unwrap(),
        *values.get("s").unwrap(),
    )
}