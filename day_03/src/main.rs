use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let path = Path::new("day_03/input.txt");
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let p1_sum = part_one(reader);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let p2_sum = part_two(reader);
    println!("Sum: {}", p1_sum);
    println!("Sum: {}", p2_sum);
}

fn part_one<T: Sized + BufRead>(input: T) -> u32 {
    let mut sum = 0;
    let mut prev_line = "".to_string();
    // store the symbol positions for the current line to check the following line for adjacent numbers
    let mut symbols = vec![];
    for (current, line) in input.lines().enumerate() {
        let line = line.unwrap();
        // check for adjacency from the previous line
        for symbol in symbols.iter() {
            let left = symbol - 1;
            let right = symbol + 1;
            sum += get_adjacent_numbers(&line, *symbol, left, right).iter().sum::<u32>();
        }
        symbols.clear();
        for (center, c) in line.chars().enumerate() {
            match c {
                '.' | '0' | '1' | '2' | '3' | '4' |
                '5' | '6' | '7' | '8' | '9' => (),
                _ => {
                    symbols.push(center);
                    let left = center - 1;
                    let right = center + 1;

                    // skip first line
                    if current != 0 {
                        sum += get_adjacent_numbers(&prev_line, center, left, right).iter().sum::<u32>();
                    }
                    sum += get_adjacent_numbers(&line, center, left, right).iter().sum::<u32>()
                }
            }
        }
        prev_line = line.clone();
    }
    sum
}

fn part_two<T: Sized + BufRead>(input: T) -> u32 {
    let mut sum = 0;
    let mut prev_line = "".to_string();
    let mut gears = vec![];
    // gear_ratio is the product of the two adjacent numbers to a '*'
    let mut gear_ratios: HashMap<String, Vec<u32>>  = HashMap::new();
    for (current, line) in input.lines().enumerate() {
        let line = line.unwrap();
        // check for adjacency from the previous line
        for gear in gears.iter() {
            let left = gear - 1;
            let right = gear + 1;
            let gear_key = format!("{}:{}", current - 1, gear);
            let gear_numbers = get_adjacent_numbers(&line, *gear, left, right);
            if let Some(x) = gear_ratios.get_mut(&gear_key) {
                x.extend(gear_numbers)
            }
        }
        gears.clear();
        for (center, c) in line.chars().enumerate() {
            if c == '*' {
                gears.push(center);
                let left = center - 1;
                let right = center + 1;
                let gear_key = format!("{}:{}", current, center);
                gear_ratios.insert(gear_key.clone(), get_adjacent_numbers(&line, center, left, right));
                // skip first line
                if current != 0 {
                    let gear_numbers = get_adjacent_numbers(&prev_line, center, left, right);
                    if let Some(x) = gear_ratios.get_mut(&gear_key) {
                        x.extend(gear_numbers)
                    }
                }
            }
        }
        prev_line = line.clone();
    }

    for (_, gear_numbers) in gear_ratios.iter() {
        if gear_numbers.len() != 2 {
            continue;
        }
        sum += gear_numbers.iter().product::<u32>();
    }
    sum
}

fn get_adjacent_numbers(input: &str, center: usize, left: usize, right: usize) -> Vec<u32> {
    let mut left = left;
    let mut right = right;
    let mut left_c = input.chars().nth(left).unwrap_or('.');
    let center_c = input.chars().nth(center).unwrap_or('.');
    let mut right_c = input.chars().nth(right).unwrap_or('.');
    let mut num_str = "".to_string();
    while left_c.is_ascii_digit() {
        num_str.insert(0, left_c);
        // break if we are at the start of the line
        if left == 0 {
            break;
        }
        left -= 1;
        left_c = input.chars().nth(left).unwrap_or('.');
    }
    if center_c.is_ascii_digit() {
        num_str.push(center_c);
    } else {
        // Handle cases where a number could be on the left or right of the center
        num_str.push('.');
    }
    while right_c.is_ascii_digit() {
        num_str.push(right_c);
        // break if we are at the end of the line
        if right == input.chars().count() - 1 {
            break;
        }
        right += 1;
        right_c = input.chars().nth(right).unwrap_or('.');
    }

    num_str.split('.').filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn test_part_one() {
        let sum = 4361;
        let input_sum = part_one(INPUT.as_bytes());
        assert_eq!(sum, input_sum);
    }

    #[test]
    fn test_part_two() {
        let sum = 467835;
        let input_sum = part_two(INPUT.as_bytes());
        assert_eq!(sum, input_sum);
    }
}