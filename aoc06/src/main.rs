use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let races = fs::read_to_string("input.txt")?;
    println!("{}", product_of_ways_to_win_1(&races)?);
    println!("{}", count_ways_to_win_2(&races)?);
    Ok(())
}

fn product_of_ways_to_win_1(races: &str) -> Result<usize, Box<dyn Error>> {
    let mut iter = races.lines().map(|line| {
        Ok::<Vec<usize>, Box<dyn Error>>(
            line.split(':')
                .nth(1)
                .ok_or("format error - nothing after ':'")?
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        )
    });

    let times = iter.next().ok_or("Missing times")??;
    let distances = iter.next().ok_or("Missing distances")??;

    Ok(times
        .into_iter()
        .zip(distances)
        .map(|(time, dist)| count_ways_to_win(time, dist))
        .product())
}

fn count_ways_to_win_2(races: &str) -> Result<usize, Box<dyn Error>> {
    let mut iter = races.lines().map(|line| {
        Ok::<_, Box<dyn Error>>(
            line.split(':')
                .nth(1)
                .ok_or("format error - nothing after ':'")?
                .chars()
                .filter(|&c| !c.is_whitespace())
                .collect::<String>()
                .parse()?,
        )
    });

    let time = iter.next().ok_or("Missing time")??;
    let record_dist = iter.next().ok_or("Missing dist")??;
    Ok(count_ways_to_win(time, record_dist))
}

fn count_ways_to_win(race_ms: usize, record_dist: usize) -> usize {
    (1..race_ms)
        .map(|hold_ms| hold_ms * (race_ms - hold_ms))
        .filter(|&traveled_dist| traveled_dist > record_dist)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RACES: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn it_works_1() {
        assert_eq!(288, product_of_ways_to_win_1(TEST_RACES).unwrap());
    }

    #[test]
    fn it_works_2() {
        assert_eq!(71503, count_ways_to_win_2(TEST_RACES).unwrap());
    }
}
