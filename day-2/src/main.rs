// Advent of Code 2023 - Day 2

use std::fs::File;
use std::str;
use regex::Regex;
use std::io::{self, BufRead};
use std::path::Path;

fn solve_games(red: i32, green: i32, blue: i32, games: io::Lines<io::BufReader<File>>) -> (i32, i32) {
    let results = games.map(|line| line.unwrap_or_default()).map(|game| parse_game(red, green, blue, &game));
    let mut game_sum = 0;
    let mut min_sum = 0;

    results.for_each(|(g,m)| {
        game_sum += g;
        min_sum += m;
    });
    
    return (game_sum, min_sum);
}

fn parse_game(red_max: i32, green_max: i32, blue_max: i32, game: &str) -> (i32, i32) {
    let name = Regex::new(r"Game ([0-9]+): ([^\n]+)").expect("Error creating regex");
    let Some(caps) = name.captures(game) else {return (0, 0)};
    
    let game_num: i32 = caps[1].parse().unwrap();
    let trial_data = &caps[2] as &str;

    let red_regex = Regex::new(r"([0-9]+) red").unwrap();
    let blue_regex = Regex::new(r"([0-9]+) blue").unwrap();
    let green_regex = Regex::new(r"([0-9]+) green").unwrap();

    let mut red_min: Option<i32> = None;
    let mut blue_min: Option<i32> = None;
    let mut green_min: Option<i32> = None;

    for trial in trial_data.split(";") {
        let red = red_regex.captures(trial);
        let blue = blue_regex.captures(trial);
        let green = green_regex.captures(trial);

        if red.is_some() {
            let blocks: i32 = red.unwrap()[1].parse().unwrap();

            if red_min.is_none() {
                red_min = Some(blocks);
            } else if red_min.unwrap() < blocks {
                red_min = Some(blocks);
            }
        }

        if blue.is_some() {
            let blocks: i32 = blue.unwrap()[1].parse().unwrap();

            if blue_min.is_none() {
                blue_min = Some(blocks);
            } else if blue_min.unwrap() < blocks {
                blue_min = Some(blocks);
            }

        }

        if green.is_some() {
            let blocks: i32 = green.unwrap()[1].parse().unwrap();

            if green_min.is_none() {
                green_min = Some(blocks);
            } else if green_min.unwrap() < blocks {
                green_min = Some(blocks);
            }
        }
    }

    if red_min.unwrap() > red_max || green_min.unwrap() > green_max || blue_min.unwrap() > blue_max {
        return (0, red_min.unwrap()*green_min.unwrap()*blue_min.unwrap()); 
    }

    // println!("{:?} with min blocks  r[{:?}] g[{:?}] b[{:?}] total{}", game_num, red_min.unwrap(), green_min.unwrap(), blue_min.unwrap(), red_min.unwrap()*green_min.unwrap()*blue_min.unwrap());

    return (game_num, red_min.unwrap()*green_min.unwrap()*blue_min.unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef <Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let results = solve_games(12, 13, 14, lines);
        println!("part 1 is {}", results.0);
        println!("part 2 is {}", results.1);
    }
}
