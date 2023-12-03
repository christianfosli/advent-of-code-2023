use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("{}", sum_part_numbers_1(&input));
    Ok(())
}

fn sum_part_numbers_1(s: &str) -> usize {
    let line_len = s.lines().next().unwrap().len();
    let s = s.replace("\n", "");
    let has_adjacent_symbol = |start_ix, stop_ix| {
        if start_ix % line_len > 0 && s.chars().nth(start_ix - 1).is_some_and(|c| c.is_symbol()) {
            true
        } else if stop_ix % line_len != line_len - 1
            && s.chars().nth(stop_ix + 1).is_some_and(|c| c.is_symbol())
        {
            true
        } else if start_ix >= line_len
            && s.chars()
                .enumerate()
                .skip(if start_ix % line_len == 0 {
                    start_ix - line_len
                } else {
                    start_ix - line_len - 1
                })
                .take_while(|&(i, _)| {
                    i <= if stop_ix % line_len == line_len - 1 {
                        stop_ix - line_len
                    } else {
                        stop_ix - line_len + 1
                    }
                })
                .any(|(_, c)| c.is_symbol())
        {
            true
        } else if start_ix < s.len() - line_len
            && s.chars()
                .enumerate()
                .skip(if start_ix % line_len == 0 {
                    start_ix + line_len
                } else {
                    start_ix + line_len - 1
                })
                .take_while(|&(i, _)| {
                    i <= if stop_ix % line_len == line_len - 1 {
                        stop_ix + line_len
                    } else {
                        stop_ix + line_len + 1
                    }
                })
                .any(|(_, c)| c.is_symbol())
        {
            true
        } else {
            false
        }
    };

    let mut sum = 0;
    let mut current_part = None;
    for (i, c) in s.chars().enumerate() {
        if i % line_len == 0 {
            current_part = None;
        }

        if current_part.is_none() && c.is_ascii_digit() {
            current_part = Some(Part {
                number: c.to_digit(10).unwrap() as usize,
                start_ix: i,
                stop_ix: i,
            });
        } else if current_part.is_some() && c.is_ascii_digit() {
            current_part = Some(Part {
                number: current_part.unwrap().number * 10 + c.to_digit(10).unwrap() as usize,
                start_ix: current_part.unwrap().start_ix,
                stop_ix: i,
            })
        } else {
            current_part = None;
        }

        if current_part.is_some()
            && (i % line_len == line_len - 1
                || s.chars().nth(i + 1).is_some_and(|c| !c.is_ascii_digit()))
        {
            let current_part = current_part.unwrap();
            if has_adjacent_symbol(current_part.start_ix, current_part.stop_ix) {
                sum += current_part.number;
            }
        }
    }
    sum
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Part {
    number: usize,
    start_ix: usize,
    stop_ix: usize,
}

trait CharExt {
    fn is_symbol(self) -> bool;
}

impl CharExt for char {
    fn is_symbol(self) -> bool {
        !self.is_ascii_digit() && !self.is_ascii_whitespace() && self != '.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_ENGINE_SCHEMATIC: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn it_works_1() {
        assert_eq!(4361, sum_part_numbers_1(TEST_ENGINE_SCHEMATIC));
    }
}
