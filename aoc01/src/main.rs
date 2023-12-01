use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let part_1 = input
        .lines()
        .filter_map(find_calibration_value_1)
        .sum::<usize>();
    dbg!(&part_1);

    let part_2 = input
        .lines()
        .filter_map(find_calibration_value_2)
        .sum::<u32>();
    dbg!(&part_2);

    Ok(())
}

fn find_calibration_value_1(s: &str) -> Option<usize> {
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

fn find_calibration_value_2(s: &str) -> Option<u32> {
    let first_digit = find_num(s, false)?;
    let last_digit = find_num(s, true)?;
    Some(first_digit * 10 + last_digit)
}

fn find_num(s: &str, backwards: bool) -> Option<u32> {
    static NUMBERS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    let letters: Box<dyn Iterator<Item = _>> = if backwards {
        Box::new(s.chars().rev())
    } else {
        Box::new(s.chars())
    };

    let mut maybe_num = Vec::new();
    for c in letters {
        if c.is_ascii_digit() {
            return c.to_digit(10);
        }

        maybe_num.push(c);
        let maybe_num = if backwards {
            maybe_num.iter().rev().collect::<String>()
        } else {
            maybe_num.iter().collect::<String>()
        };
        if let Some(&n) = NUMBERS.iter().find(|&n| maybe_num.contains(n)) {
            return Some(match n {
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
