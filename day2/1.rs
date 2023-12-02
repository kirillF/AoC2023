use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.lines();
    let mut sum = 0;
    let (red, green, blue) = (12, 13, 14);

    for line in lines {
        let game_info: Vec<&str> = line.split(":").collect();
        if game_info.len() != 2 {
            continue;
        }
        let game_id = game_info[0].split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
        let game_data = game_info[1].split(";");

        let mut valid = true;
        for gd in game_data {
            let parts: Vec<&str> = gd.split(",").collect();
            if !valid {
                break;
            }
            for part in parts {
                let color_data: Vec<&str> = part.split_whitespace().collect();
                if color_data.len() == 2 {
                    let (number, color) = (color_data[0].parse::<i32>().unwrap(), color_data[1]);
                    if (color == "red" && number > red) || (color == "green" && number > green) || (color == "blue" && number > blue) {
                        valid = false;
                        break;
                    }
                }
            }
        }
        if valid {
            sum += game_id;
        }
    }

    println!("{}", sum);
}