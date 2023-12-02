use std::{error::Error, fs, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let part_1 = sum_valid_games(
        &input,
        &Cubes {
            red: 12,
            green: 13,
            blue: 14,
        },
    )?;
    dbg!(part_1);
    Ok(())
}

fn sum_valid_games(s: &str, actual: &Cubes) -> Result<usize, Box<dyn Error>> {
    Ok(s.lines()
        .map(|l| l.parse::<Game>().unwrap())
        .filter(|g| {
            g.reveals.iter().all(|r| {
                r.red <= actual.red
                    && r.green <= actual.green
                    && r.blue <= actual.blue
                    && r.sum() <= actual.sum()
            })
        })
        .map(|g| g.id as usize)
        .sum())
}

#[derive(Debug, Clone)]
pub struct Game {
    id: usize,
    reveals: Vec<Cubes>,
}

#[derive(Debug, Clone, Copy)]
pub struct Cubes {
    red: u8,
    green: u8,
    blue: u8,
}

impl Cubes {
    fn sum(&self) -> u8 {
        self.red + self.green + self.blue
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static GAME_ID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (\d+)").unwrap());
        static RED_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) red").unwrap());
        static GREEN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) green").unwrap());
        static BLUE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) blue").unwrap());

        if let [game, reveals] = s.split(':').collect::<Vec<_>>()[..] {
            let id = GAME_ID_RE.captures(game).ok_or("No game id found")?[1].parse::<usize>()?;
            let reveals = reveals
                .split(';')
                .map(|rev| Cubes {
                    red: RED_RE
                        .captures(rev)
                        .map(|caps| caps[1].parse::<u8>().unwrap())
                        .unwrap_or(0),
                    green: GREEN_RE
                        .captures(rev)
                        .map(|caps| caps[1].parse::<u8>().unwrap())
                        .unwrap_or(0),
                    blue: BLUE_RE
                        .captures(rev)
                        .map(|caps| caps[1].parse::<u8>().unwrap())
                        .unwrap_or(0),
                })
                .collect::<Vec<_>>();

            return Ok(Game { id, reveals });
        }
        Err("Format error - expected exactly one ':'".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GAMES: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    pub fn it_works_1() {
        assert_eq!(
            8,
            sum_valid_games(
                TEST_GAMES,
                &Cubes {
                    red: 12,
                    green: 13,
                    blue: 14
                }
            )
            .unwrap()
        );
    }
}
