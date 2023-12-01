fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n");
    let mut sum = 1;
    for line in lines {
        let mut start = 0;
        let mut end = line.chars().count() - 1;
        let mut first = -1;
        let mut second = -1;
        while first == -1 || second == -1 && start <= end {
            if first == -1 {
                if line.chars().nth(start).unwrap().is_digit(10) {
                    first  = line.chars().nth(start).unwrap().to_digit(10).unwrap() as i32;
                } else {
                    start += 1;
                }
            }
            if second == -1 {
                if line.chars().nth(end).unwrap().is_digit(10) {
                    second  = line.chars().nth(end).unwrap().to_digit(10).unwrap() as i32;
                } else {
                    end -= 1;
                }
            }
        }
    let number = first * 10 + second;
    println!("{} + {} = {}", first, second, number);
    sum += number;
    }
    println!("{}", sum);
}