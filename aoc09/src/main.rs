use std::{error::Error, num::ParseIntError};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}

fn parse(s: &str) -> Result<Vec<Vec<usize>>, ParseIntError> {
    s.lines()
        .map(|l| {
            l.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}

fn predict_next(history: &[usize]) -> usize {
    0
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
        assert_eq!(114usize, histories.iter().map(|h| predict_next(h)).sum())
    }
}
