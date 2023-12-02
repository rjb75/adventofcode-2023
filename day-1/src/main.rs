// Advent of Code 2023 - Day 1

use std::fs;

fn check_number_word(line: &str) -> i32 {
    let chars = line.chars();

    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    for (i, char) in chars.into_iter().enumerate() {
        if char.is_digit(10) {
            if first.is_none() {
                first = Some(char.to_digit(10).unwrap_or_default() as i32);
                last = first;
                continue;
            } else {
                last = Some(char.to_digit(10).unwrap_or_default() as i32);
                continue;
            }
        }

        let mut has_match: Option<usize>;
        let mut match_val: Option<i32>;
        match char {
            'o' => {
                has_match = line[i..].find("one");
                match_val = Some(1);
            }
            't' => {
                has_match = line[i..].find("two");
                match_val = Some(2);
                if has_match.is_none() || has_match.ne(&Some(0)) {
                    has_match = line[i..].find("three");
                    match_val = Some(3);
                }
            }
            'f' => {
                has_match = line[i..].find("four");
                match_val = Some(4);
                if has_match.is_none() || has_match.ne(&Some(0)) {
                    has_match = line[i..].find("five");
                    match_val = Some(5);
                }
            }
            's' => {
                has_match = line[i..].find("six");
                match_val = Some(6);
                if has_match.is_none() || has_match.ne(&Some(0)) {
                    has_match = line[i..].find("seven");
                    match_val = Some(7);
                }
            }
            'e' => {
                has_match = line[i..].find("eight");
                match_val = Some(8);
            }
            'n' => {
                has_match = line[i..].find("nine");
                match_val = Some(9);
            }
            'z' => {
                has_match = line[i..].find("zero");
                match_val = Some(0);
            }
            _ => continue,
        }

        if has_match.is_none() || has_match.ne(&Some(0)) {
            continue;
        }

        if first.is_none() {
            first = match_val;
            last = first;
            continue;
        } else {
            last = match_val;
            continue;
        }
    }

    if first.is_none() {
        return 0;
    }

    return (first.unwrap_or_default() * 10) + last.unwrap_or_default();
}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Not working");
    let lines = contents.lines();

    let sum: i32 = lines.map(|line| check_number_word(line)).sum();
    println!("Sum is {sum}");
}
