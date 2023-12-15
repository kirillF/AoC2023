use std::fs;
use std::time::Instant;
use std::fmt;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut curr = &self.next;
        let mut s = String::new();
        while let Some(node) = curr {
            s.push_str(&format!("{} -> ", node.value));
            curr = &node.next;
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let start = Instant::now();
    let data = fs::read_to_string("input.txt").unwrap();

    let strings = data.split(",").collect::<Vec<&str>>();

    let mut buckets = vec![Node::new(("", 0)); 256];

    let ans = solve(strings, &mut buckets);

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}

fn hash(string: &str) -> i32 {
    let mut curr = 0;
    for c in string.chars() {
        let ascii = c as u8;
        curr += ascii as i32;
        curr *= 17;
        curr %= 256;
    }
    curr
}

fn solve<'a>(strings: Vec<&'a str>, buckets: &mut Vec<Node<(&'a str, i32)>>) -> i32 {
    for s in strings {
        if s.ends_with('-') {
            let s = &s[..s.len() - 1];
            let label = hash(s) as usize;
            let mut curr = &mut buckets[label].next;
            loop {
                match curr {
                    Some(node) if node.value.0 == s => {
                        *curr = node.next.take();
                        break;
                    }
                    Some(node) => curr = &mut node.next,
                    None => break,
                }
            }
        } else {
            let data: Vec<&str> = s.split("=").collect();
            let label = hash(data[0]) as usize;
            let val = data[1].parse::<i32>().unwrap();
            let mut curr = &mut buckets[label].next;
            
            loop {
                match curr {
                    Some(node) if node.value.0 == data[0] => {
                        node.value.1 = val;
                        break;
                    }
                    Some(node) => curr = &mut node.next,
                    None => {
                        let new = Box::new(Node::new((data[0], val)));
                        *curr = Some(new);
                        break;
                    }
            
                }
            }
        }
    }

    let mut ans = 0;
    for (i, bucket) in buckets.iter().enumerate() {
        let mut curr = &bucket.next;
        let mut count = 1;
        while let Some(node) = curr {
            let val = (1 + i as i32) * count * node.value.1;
            curr = &node.next;
            ans += val;
            count += 1;
        }
    }

    ans as i32
}