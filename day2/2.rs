use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    for line in data.lines() {
        let game_info: Vec<&str> = line.split(":").collect();
        if game_info.len() != 2 {
            continue;
        }
        let game_data = game_info[1].split(";");

        let mut colors = [0; 3];
        for gd in game_data {
            let parts: Vec<&str> = gd.split(",").collect();
            for part in &parts {
                let color_data: Vec<&str> = part.split_whitespace().collect();
                let color = color_data[1];
                let number = color_data[0].parse::<i32>().unwrap();
                match color {
                    "red" => colors[0] = colors[0].max(number),
                    "green" => colors[1] = colors[1].max(number),
                    "blue" => colors[2] = colors[2].max(number),
                    _ => (),
                }
            }
        }
        sum += colors.iter().product();
    }

    println!("{}", sum);
}