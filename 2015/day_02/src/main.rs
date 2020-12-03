#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 4 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_part_1 ... bench:       4,563 ns/iter (+/- 831)
    test bench::bench_part_2 ... bench:       3,687 ns/iter (+/- 89)

*/

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

struct Dimensions {
    length: usize,
    width: usize,
    height: usize,
}

impl Dimensions {
    fn calculate_surface_area(&self) -> usize {
        2 * self.length * self.width
            + 2 * self.width * self.height
            + 2 * self.height * self.length
            + std::cmp::min(
                self.length * self.width,
                std::cmp::min(self.width * self.height, self.height * self.length),
            )
    }

    fn calculate_ribbon_length(&self) -> usize {
        self.length * 2 + self.width * 2 + self.height * 2
            - std::cmp::max(self.length, std::cmp::max(self.width, self.height)) * 2
            + self.length * self.width * self.height
    }
}

impl TryFrom<&str> for Dimensions {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        use regex::Regex;

        lazy_static! {
            static ref DAY_02_DIMENSIONS_REGEX: Regex =
                Regex::new(r"^(?P<length>\d+)x(?P<width>\d+)x(?P<height>\d+)$")
                    .expect("Invalid DAY_02_DIMENSIONS_REGEX!");
        }

        if let Some(cap) = DAY_02_DIMENSIONS_REGEX.captures(value) {
            Ok(Self {
                length: cap["length"].parse::<usize>()?,
                width: cap["width"].parse::<usize>()?,
                height: cap["height"].parse::<usize>()?,
            })
        } else {
            err!("Couldn't parse input: {}", value)
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let dimensions: Vec<Dimensions> = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&dimensions))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&dimensions))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Dimensions>> {
    input.lines().map(Dimensions::try_from).collect()
}

fn part_1(dimensions: &[Dimensions]) -> usize {
    dimensions.iter().map(|v| v.calculate_surface_area()).sum()
}

fn part_2(dimensions: &[Dimensions]) -> usize {
    dimensions.iter().map(|v| v.calculate_ribbon_length()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        "2x3x4\n1x1x10".to_string()
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let dimensions = parse_input(&get_input())?;
        assert_eq!(part_1(&dimensions), 58 + 43);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let dimensions = parse_input(&get_input())?;
        assert_eq!(part_2(&dimensions), 34 + 14);
        Ok(())
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use std::fs::File;
    use test::Bencher;

    fn read_input_file() -> Result<String> {
        let mut input = String::new();
        File::open("input/input.txt")?.read_to_string(&mut input)?;
        Ok(input)
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) -> Result<()> {
        let dimensions = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&dimensions)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let dimensions = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&dimensions)));
        Ok(())
    }
}
