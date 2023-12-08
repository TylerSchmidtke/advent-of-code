use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let path = Path::new("day_02/input.txt");
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    // rgb
    let cubes = (12, 13, 14);
    let mut part_one_sum = 0;
    let mut part_two_sum= 0;
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
        if draw.get_red() > max.0 {
            max.0 = draw.get_red();
        }
        if draw.get_green() > max.1 {
            max.1 = draw.get_green();
        }
        if draw.get_blue() > max.2 {
            max.2 = draw.get_blue();
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
            "blue" => draw.blue = Some(count),
            "red" => draw.red = Some(count),
            "green" => draw.green = Some(count),
            _ => (),
        }
    });
    draw
}

struct Draw {
    pub blue: Option<i32>,
    pub red: Option<i32>,
    pub green: Option<i32>,
}

impl Draw {
    fn default() -> Draw {
        Draw {
            blue: None,
            red: None,
            green: None,
        }
    }
    fn valid(self, r: i32, g: i32, b: i32) -> bool {
        self.red.unwrap_or(0) <= r &&
            self.green.unwrap_or(0) <= g &&
            self.blue.unwrap_or(0) <= b
    }
    fn get_red(&self) -> i32 {
        self.red.unwrap_or(0)
    }

    fn get_green(&self) -> i32 {
        self.green.unwrap_or(0)
    }

    fn get_blue(&self) -> i32 {
        self.blue.unwrap_or(0)
    }
}