use std::{error::Error, fs, num::ParseIntError};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let histories = parse(&input)?;
    let part_1 = histories.iter().map(|h| predict_next(h)).sum::<isize>();
    println!("part 1: {part_1}");
    Ok(())
}

fn parse(s: &str) -> Result<Vec<Vec<isize>>, ParseIntError> {
    s.lines()
        .map(|l| {
            l.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}

fn predict_next(history: &[isize]) -> isize {
    let mut extrapolation = Vec::<Vec<isize>>::new();
    let mut extrapolation_done = false;
    extrapolation.push(history.to_vec());
    while !extrapolation_done {
        let steps = extrapolation
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(&x1, &x2)| x2 - x1)
            .collect::<Vec<_>>();

        if steps.iter().all(|&dx| dx == 0) {
            extrapolation_done = true
        }

        extrapolation.push(steps);
    }

    extrapolation
        .into_iter()
        .rev()
        .map(|seq| *seq.last().unwrap())
        .reduce(|val_below, val_left| val_left + val_below)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_REPORT: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn it_works_1() {
        let histories = parse(TEST_REPORT).unwrap();
        assert_eq!(114isize, histories.iter().map(|h| predict_next(h)).sum())
    }
}
