use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let part_1 = input
        .lines()
        .filter_map(find_calibration_value_1)
        .sum::<u32>();
    dbg!(&part_1);

    let part_2 = input
        .lines()
        .filter_map(find_calibration_value_2)
        .sum::<u32>();
    dbg!(&part_2);

    Ok(())
}

fn find_calibration_value_1(s: &str) -> Option<u32> {
    let first_digit = s.chars().find(char::is_ascii_digit)?;
    let last_digit = s.chars().rev().find(char::is_ascii_digit)?;
    Some(format!("{first_digit}{last_digit}").parse::<u32>().unwrap())
}

fn find_calibration_value_2(s: &str) -> Option<u32> {
    let first_digit = find_num(s, false)?;
    let last_digit = find_num(s, true)?;
    Some(first_digit * 10 + last_digit)
}

fn find_num(s: &str, backwards: bool) -> Option<u32> {
    static DIGITS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let letters: Box<dyn Iterator<Item = _>> = if backwards {
        Box::new(s.chars().rev())
    } else {
        Box::new(s.chars())
    };

    let mut iterated_chars = Vec::new();
    for c in letters {
        if c.is_ascii_digit() {
            return c.to_digit(10);
        }

        iterated_chars.push(c);
        let maybe_digit = if backwards {
            iterated_chars.iter().rev().collect::<String>()
        } else {
            iterated_chars.iter().collect::<String>()
        };
        if let Some(&digit) = DIGITS.iter().find(|&n| maybe_digit.contains(n)) {
            return Some(match digit {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => unreachable!(),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(Some(12), find_calibration_value_1("1abc2"));
        assert_eq!(Some(38), find_calibration_value_1("pqr3stu8vwx"));
        assert_eq!(Some(15), find_calibration_value_1("a1b2c3d4e5f"));
        assert_eq!(Some(77), find_calibration_value_1("treb7uchet"));
        assert_eq!(None, find_calibration_value_1("string-with-no-numbers"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(Some(29), find_calibration_value_2("two1nine"));
        assert_eq!(Some(83), find_calibration_value_2("eightwothree"));
        assert_eq!(Some(13), find_calibration_value_2("abcone2threexyz"));
        assert_eq!(Some(24), find_calibration_value_2("xtwone3four"));
        assert_eq!(Some(42), find_calibration_value_2("4nineeightseven2"));
    }
}
