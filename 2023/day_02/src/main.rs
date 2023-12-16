use std::io::BufRead;

fn main() {
    let reader = loader::load_day("02");
    // rgb
    let cubes = (12, 13, 14);
    let mut part_one_sum = 0;
    let mut part_two_sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        part_one_sum += part_one(&line, &cubes);
        part_two_sum += part_two(&line);
    }
    println!("Part One Sum: {}", part_one_sum);
    println!("Part Two Sum: {}", part_two_sum);
}

fn part_one(line: &str, cubes: &(i32, i32, i32)) -> i32 {
    let game_id = get_game_id(line);
    for draw in get_draws(line) {
        if !draw.valid(cubes.0, cubes.1, cubes.2) {
            return 0;
        }
    }

    game_id
}

fn part_two(line: &str) -> i32 {
    // rgb
    let mut max = (0, 0, 0);
    for draw in get_draws(line) {
        if draw.red> max.0 {
            max.0 = draw.red;
        }
        if draw.green > max.1 {
            max.1 = draw.green;
        }
        if draw.blue > max.2 {
            max.2 = draw.blue;
        }
    }

    max.0 * max.1 * max.2
}

fn get_game_id(line: &str) -> i32 {
    line.split(": ")
        .next().unwrap()
        .split(' ').last().unwrap()
        .parse::<i32>().unwrap()
}

fn get_draws(line: &str) -> Vec<Draw> {
    let mut draws = Vec::new();
    for game in line.split(": ").skip(1) {
        game.split(';').for_each(|draw_str| {
            draws.push(get_draw(draw_str));
        });
    }
    draws
}

fn get_draw(draw_str: &str) -> Draw {
    let mut draw = Draw::default();
    draw_str.split(',').for_each(|cube| {
        let mut cube = cube.trim().split(' ');
        let count = cube.next().unwrap().parse::<i32>().unwrap();
        let color = cube.next().unwrap();
        match color {
            "blue" => draw.blue = count,
            "red" => draw.red = count,
            "green" => draw.green = count,
            _ => (),
        }
    });
    draw
}

struct Draw {
    pub blue: i32,
    pub red: i32,
    pub green: i32,
}

impl Draw {
    fn default() -> Draw {
        Draw {
            blue: 0,
            red: 0,
            green: 0,
        }
    }
    fn valid(self, r: i32, g: i32, b: i32) -> bool {
        self.red <= r &&
            self.green <= g &&
            self.blue <= b
    }
}