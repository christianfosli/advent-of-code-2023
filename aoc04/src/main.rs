use std::{collections::HashSet, error::Error, fs};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", count_points(&input));
    Ok(())
}

fn count_points(s: &str) -> u32 {
    s.lines()
        .map(|line| {
            let line = line.split(':').into_iter().skip(1).next().unwrap();
            let (mine, winning) = line
                .split('|')
                .map(|nums| {
                    nums.split_whitespace()
                        .map(|n| n.parse::<u8>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .next_tuple()
                .unwrap();
            let win_count = mine.intersection(&winning).count();
            match win_count {
                0 => 0,
                _ => 2u32.pow(win_count as u32 - 1),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CARDS: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn it_works_1() {
        assert_eq!(13, count_points(TEST_CARDS));
    }
}
