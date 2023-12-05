use std::io::{self, BufRead};
use std::{
    str::FromStr,
    collections::HashSet,
    path::Path,
    fs::File,
};

fn main() {
    run_puzzle_one("./puzzle.txt");
}

// Copied shamelessly from Rust By Example.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Game {
    id: u32,
    winning: HashSet<u32>,
    card: HashSet<u32>
}

struct ParseGameErr;

impl FromStr for Game {
    type Err = ParseGameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_split = s.split(":").collect::<Vec<&str>>();
        let game_id_result = start_split[0]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap() // yes I am evil
            .parse::<u32>();
        let game_id = match game_id_result {
            Ok(game_id) => game_id,
            Err(error) => panic!("couldn't parse game ID: {}", error)
        };

        let numbers = start_split[1].split("|").collect::<Vec<&str>>();

        return Ok(Game {
            id: game_id,
            winning: collect_num_line_set(numbers[0]),
            card: collect_num_line_set(numbers[1]),
        });
    }
}

fn collect_num_line_set(num_line: &str) -> HashSet<u32> {
    return HashSet::from_iter(
        num_line
        .split(" ")
        .filter_map(|num_str| {
            if num_str == "" {
                return None;
            }
            let num_result = num_str.parse::<u32>();
            let num: Option<u32> = match num_result {
                Ok(num) => Some(num),
                Err(_) => panic!("invalid number in line! {}", num_str)
            };
            return num;
        })
    );
}

fn run_puzzle_one<P>(filename: P) where P: AsRef<Path> {
    let lines = read_lines(filename).unwrap();

    let games = lines
        .map(|line| {
            match line {
                Ok(game_line) =>
                    match Game::from_str(game_line.as_str()) {
                        Ok(game) => game,
                        Err(_) => panic!("bad line {}", game_line)
                    }
                Err(_) => panic!("There was a line we were unable to read."),
            }
        });

    let _points: u32 = games
        .map(|game|
            game.card.into_iter()
                .fold(0, |points, number| {
                    if game.winning.contains(&number) {
                        return if points == 0 { 1 } else { points * 2 };
                    }
                    return points;
                })
        )
        .sum();

    println!("{}", _points);
}