use std::{collections::HashMap, error::Error, fs};

use itertools::Itertools;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", steps_until_zzz(&input)?);
    Ok(())
}

fn steps_until_zzz(s: &str) -> Result<usize, Box<dyn Error>> {
    let map_item_re = Regex::new(r"(?<key>\w\w\w) = \((?<left>\w\w\w), (?<right>\w\w\w)\)")?;

    let (lr, nodes) = s.split("\n\n").next_tuple().ok_or("invalid format")?;
    let lr = lr.chars().collect::<Vec<_>>();

    let nodes = nodes
        .lines()
        .map(|line| {
            if let Some(caps) = map_item_re.captures(line) {
                let key = caps["key"].to_owned();
                let (left, right) = (caps["left"].to_owned(), caps["right"].to_owned());
                Ok((key, (left, right)))
            } else {
                Err("Regex didn't match")
            }
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let mut current_node = "AAA".to_owned();
    let mut steps = 0;

    while &current_node != "ZZZ" {
        let dir = lr[steps % lr.len()];
        current_node = nodes
            .get(&current_node)
            .map(|(left, right)| match dir {
                'L' => left.clone(),
                'R' => right.clone(),
                _ => panic!("LR contained unexpected char {dir}"),
            })
            .ok_or("current node is missing in nodes map")?;
        steps += 1;
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use crate::steps_until_zzz;

    const TEST_MAP_1: &'static str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_MAP_2: &'static str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn it_works_1() {
        assert_eq!(2, steps_until_zzz(TEST_MAP_1).unwrap());
        assert_eq!(6, steps_until_zzz(TEST_MAP_2).unwrap());
    }
}
