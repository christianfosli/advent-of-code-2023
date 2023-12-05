use std::{error::Error, fs, ops::Range, str::FromStr};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", find_lowest_location_1(&input)?);
    Ok(())
}

fn find_lowest_location_1(almanac: &str) -> Result<u32, Box<dyn Error>> {
    let init_seeds = almanac
        .lines()
        .next()
        .ok_or("invalid format - no lines")?
        .split(':')
        .nth(1)
        .ok_or("invalid format - no ':'")?
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let maps = almanac
        .split("\n\n")
        .skip(1)
        .map(str::parse::<CategoryMap>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(init_seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, el| el.map_num(acc)))
        .min()
        .ok_or("invalid format - no seeds")?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CategoryMap {
    from: String,
    to: String,
    maps: Vec<Map>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    src_range: Range<u32>,
    offset: i64,
}

impl CategoryMap {
    fn map_num(&self, num: u32) -> u32 {
        if let Some(matching_map) = self.maps.iter().find(|&map| map.src_range.contains(&num)) {
            u32::try_from(num as i64 + matching_map.offset).unwrap()
        } else {
            num
        }
    }
}

impl FromStr for CategoryMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let categories_re = Regex::new(r"(.+)-to-(.+) map").unwrap();
        let map_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
        let mut lines = s.lines();

        let categories_caps = categories_re
            .captures(lines.next().ok_or("missing header")?)
            .ok_or("incorrect header format")?;
        let from = categories_caps[1].to_owned();
        let to = categories_caps[2].to_owned();

        let maps = lines
            .map(|line| {
                let map_caps = map_re.captures(line).ok_or("missing map nums")?;
                let dest_range_start = map_caps[1].parse::<u32>()?;
                let source_range_start = map_caps[2].parse::<u32>()?;
                let range_len = map_caps[3].parse::<u32>()?;
                Ok(Map {
                    src_range: source_range_start..source_range_start + range_len + 1,
                    offset: dest_range_start as i64 - source_range_start as i64,
                })
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(CategoryMap { from, to, maps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ALMANAC: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn it_works_1() {
        assert_eq!(35, find_lowest_location_1(TEST_ALMANAC).unwrap());
    }
}
