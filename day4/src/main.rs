use std::io::{self, BufRead};
use std::{
    str::FromStr,
    collections::HashSet,
    path::Path,
    fs::File,
};

fn main() {
    run_puzzle_one("./puzzle.txt");
    run_puzzle_two("./puzzle.txt");
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
    copies: u32,
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
            copies: 1,
            winning: collect_num_line_set(numbers[0]),
            card: collect_num_line_set(numbers[1]),
        });
    }
}

impl Game {
    pub fn score(&self) -> u32 {
        return self.card.clone().into_iter()
            .fold(0, |points, number| {
                if self.winning.contains(&number) {
                    return if points == 0 { 1 } else { points * 2 };
                }
                return points;
            }
        );
    }

    pub fn matching_numbers(&self) -> usize {
        return self.card.clone().into_iter().filter(|number| self.winning.contains(number)).count();
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

    println!("{}", games.map(|game| game.score()).sum::<u32>());
}

fn run_puzzle_two<P>(filename: P) where P: AsRef<Path> {
    let lines = read_lines(filename).unwrap();

    let mut games = lines
        .map(|line| {
            match line {
                Ok(game_line) =>
                    match Game::from_str(game_line.as_str()) {
                        Ok(game) => game,
                        Err(_) => panic!("bad line {}", game_line)
                    }
                Err(_) => panic!("There was a line we were unable to read."),
            }
        })
        .collect::<Vec<Game>>();

    // Yeah I'm ditching functional style for this part, I'm just not there yet
    for i in 0_usize..games.len() {
        for _ in 0..games[i].copies {
            let matching: usize = games[i].matching_numbers();
            for n in 1_usize..=matching {
                if i+n < games.len() {
                    println!("adding a copy to game {}", games[i+n].id);
                    games[i+n].copies += 1;
                }
            }
        }
    }

    println!("{}", games.into_iter().fold(0, |sum, game| sum + game.copies));
}