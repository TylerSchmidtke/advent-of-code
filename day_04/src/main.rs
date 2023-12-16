use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::iter::FilterMap;
use std::str::Split;

fn main() {
    let reader = loader::load_day("04");
    let p1_sum = part_one(reader);
    let reader = loader::load_day("04");
    let p2_sum = part_two(reader);
    println!("Sum: {}", p1_sum);
    println!("Sum: {}", p2_sum);
}

fn part_one<T: Sized + BufRead>(input: T) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (winning, hand) = parse_line(&line);
        match winning.intersection(&hand).count() {
            0 => continue,
            count => sum += calculate_match_sum(count),
        }
    }

    sum
}

fn part_two<T: Sized + BufRead>(input: T) -> u32 {
    let mut counts: HashMap<usize, u32> = HashMap::new();
    counts.insert(1, 1);
    let mut total = 0;
    let mut i = 1;
    for line in input.lines() {
        let line = line.unwrap();
        let card_count = *counts.entry(i).or_insert(1);
        let (winning, hand) = parse_line(&line);
        match winning.intersection(&hand).count() {
            0 => (),
            count => (1..count + 1).for_each(|n| {
                *counts.entry(i + n).or_insert(1) += card_count;
            })
        }

        total += card_count;
        i += 1;
    }

    total
}

fn calculate_match_sum(i: usize) -> u32 {
    // sum is 1 for the first iteration
    let mut sum = 1;
    (1..i).for_each(|_| sum *= 2);
    sum
}

fn parse_line(l: &str) -> (HashSet<u32>, HashSet<u32>) {
    let mut cards = l.split(':').last().unwrap().split('|');
    let winning = parse_card(cards.next().unwrap()).collect::<HashSet<u32>>();
    let hand = parse_card(cards.next().unwrap()).collect::<HashSet<u32>>();
    (winning, hand)
}

type CardFilter<'a> = FilterMap<Split<'a, char>, fn(&str) -> Option<u32>>;

// parse_card create a map of all the numbers in a card with the values as keys
fn parse_card(s: &str) -> CardFilter {
    s.trim()
        .split(' ')
        // drop values that don't parse, should just be extra spaces
        .filter_map(|n| n.parse::<u32>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_part_one() {
        let sum = 13;
        let input_sum = part_one(INPUT.as_bytes());
        assert_eq!(sum, input_sum);
    }

    #[test]
    fn test_part_two() {
        let sum = 30;
        let input_sum = part_two(INPUT.as_bytes());
        assert_eq!(sum, input_sum);
    }
}