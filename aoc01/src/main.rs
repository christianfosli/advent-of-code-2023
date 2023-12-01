use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let result = input
        .lines()
        .filter_map(find_calibration_value)
        .sum::<usize>();
    dbg!(&result);
    Ok(())
}

fn find_calibration_value(s: &str) -> Option<usize> {
    let first_digit = s.chars().find(|c| c.is_ascii_digit())?;
    let last_digit = s.chars().rev().find(|c| c.is_ascii_digit())?;

    Some(
        [first_digit, last_digit]
            .into_iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Some(12), find_calibration_value("1abc2"));
        assert_eq!(Some(38), find_calibration_value("pqr3stu8vwx"));
        assert_eq!(Some(15), find_calibration_value("a1b2c3d4e5f"));
        assert_eq!(Some(77), find_calibration_value("treb7uchet"));
        assert_eq!(None, find_calibration_value("string-with-no-numbers"));
    }
}
