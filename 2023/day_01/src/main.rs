use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let reader = loader::load_day("01");

    let mut sum_one: u32 = 0;
    let mut sum_two: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let first_one = get_digit(&line, true, false).unwrap();
        let last_one = get_digit(&line, true, true).unwrap();
        sum_one += first_one*10 + last_one;

        let first_two = get_digit(&line, false, false).unwrap();
        let last_two = get_digit(&line, false, true).unwrap();
        sum_two += first_two*10 + last_two;
    }
    println!("Part One Sum: {}", sum_one);
    println!("Part Two Sum: {}", sum_two);
}

fn get_digit(line: &str, part_one: bool, rev: bool) -> Option<u32> {
    let mut word = String::new();
    let chars: Vec<char> = if rev {
        line.chars().rev().collect()
    } else {
        line.chars().collect()
    };
    for c in chars {
        if c.is_ascii_digit() {
            return Some(c.to_digit(10).unwrap());
        // part two checks for digits as words
        } else if !part_one {
            word.push(c);
            if word.len() >= 3 {
                if let Some(digit) = get_word_digit(&word, rev) {
                    return Some(digit);
                }
            }
        }
    }
    None
}

fn get_word_digit(word: &String, rev: bool) -> Option<u32> {
    let digit_words = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    for i in 0..word.len() - 2 {
        let slice = if rev {
            word[i..].chars().rev().collect::<String>()
        } else {
            word[i..].to_string()
        };
        if let Some(digit) = digit_words.get(slice.as_str()) {
            return Some(*digit);
        }
    }
    None
}