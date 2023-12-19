use std::fs;
use std::time::Instant;


fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        (
            parts.next().map(|s| {
                match s {
                    "U" => (-1, 0),
                    "D" => (1, 0),
                    "L" => (0, -1),
                    "R" => (0, 1),
                    _ => panic!("Invalid direction"),
                }
            }).unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap(),
        )
    }).collect::<Vec<((i32, i32), i32, &str)>>();

    let ans = solve(&data);

    println!("{}", ans);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn solve(data: &Vec<((i32, i32), i32, &str)>) -> i32 {
    let mut border: Vec<(i32, i32)> = Vec::new();
    let mut point = (0, 0);

    for (dir, l) in data.iter().map(|(dir, l, _)| (dir, l)) {
        let (dx, dy) = dir;
        for _ in 0..*l {
            let (x, y) = point;
            border.push(point);
            let (nx, ny) = (x + dx, y + dy);
            point = (nx, ny);
        }
    }

    let mut x_diff = i32::MAX;
    let mut y_diff = i32::MAX;

    let mut x_max = i32::MIN;
    let mut y_max = i32::MIN;

    for border_point in border.iter() {
        let (x, y) = border_point;
        if *x < x_diff {
            x_diff = *x;
        }
        if *x > x_max {
            x_max = *x;
        }
        if *y < y_diff {
            y_diff = *y;
        }
        if *y > y_max {
            y_max = *y;
        }
    }

    let mut matrix = vec![vec![0; (y_max - y_diff + 1) as usize]; (x_max - x_diff + 1) as usize];

    for border_point in border.iter() {
        let (x, y) = border_point;
        matrix[(x - x_diff) as usize][(y - y_diff) as usize] = 1;
    }

    let mut ans = 0;
    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        if matrix[i][0] == 0 {
            ans += floodfill(&mut matrix, (i as i32, 0));
        }
        if matrix[i][n - 1] == 0 {
            ans += floodfill(&mut matrix, (i as i32, n as i32 - 1));
        }
    }
    for j in 0..n {
        if matrix[0][j] == 0 {
            ans += floodfill(&mut matrix, (0, j as i32));
        }
        if matrix[m - 1][j] == 0 {
            ans += floodfill(&mut matrix, (m as i32 - 1, j as i32));
        }
    }

    (m * n) as i32 - ans
}

fn floodfill(matrix: &mut Vec<Vec<i32>>, start: (i32, i32)) -> i32 {
    let mut ans = 0;

    let mut stack = vec![start];

    while let Some((x, y)) = stack.pop() {
        if matrix[x as usize][y as usize] == 1 || matrix[x as usize][y as usize] == 2 {
            continue;
        }

        matrix[x as usize][y as usize] = 2;
        ans += 1;
        for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && ny >= 0 && nx < matrix.len() as i32 && ny < matrix[0].len() as i32{
                stack.push((nx, ny));
            }
        }
    }
    ans
}