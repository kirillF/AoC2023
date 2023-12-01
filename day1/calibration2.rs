use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    value: Option<i32>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: None,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, key: &str, value: i32) {
        let mut node = &mut self.root;
        for c in key.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
        node.value = Some(value);
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("zero", 0);
    trie.insert("one", 1);
    trie.insert("two", 2);
    trie.insert("three", 3);
    trie.insert("four", 4);
    trie.insert("five", 5);
    trie.insert("six", 6);
    trie.insert("seven", 7);
    trie.insert("eight", 8);
    trie.insert("nine", 9);

    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split('\n');
    let mut sum = 1;
    for raw_line in lines {
        let line = process_line(raw_line, &trie);
        let first = line.chars().next().unwrap().to_digit(10).unwrap() as i32;
        let second = line.chars().last().unwrap().to_digit(10).unwrap() as i32;
        let number = first * 10 + second;
        println!("{} + {} = {}", first, second, number);
        sum += number;
    }
    println!("{}", sum);
}

fn process_line(line: &str, trie: &Trie) -> String {
    let root = &trie.root;
    let mut new_line = String::new();
    for i in 0..line.chars().count() {
        let mut node = root;
        let mut j = i;
        if line.chars().nth(i).unwrap().is_digit(10) {
            new_line.push(line.chars().nth(i).unwrap());
            continue;
        }
        while j < line.chars().count() {
            let c = line.chars().nth(j).unwrap();
            if let Some(child) = node.children.get(&c) {
                node = child;
                if node.is_end_of_word {
                    new_line.push_str(&node.value.unwrap().to_string());
                    break;
                }
            } else {
                break;
            }
            j += 1;
        }
    } 
    new_line
}