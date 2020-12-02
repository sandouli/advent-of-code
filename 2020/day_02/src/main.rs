#![cfg_attr(feature = "unstable", feature(test))]

// Lauch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 3 tests
    test bench::bench_parse_input ... bench:   1,507,414 ns/iter (+/- 33,321)
    test bench::bench_part_1      ... bench:     153,004 ns/iter (+/- 3,151)
    test bench::bench_part_2      ... bench:      69,954 ns/iter (+/- 2,362)

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

#[derive(Debug)]
struct PasswordRules {
    first_number: usize,
    second_number: usize,
    character: char,
    password: String,
}

impl PasswordRules {
    fn is_password_valid_1(&self) -> bool {
        let occurrences = self.password.matches(self.character).count();
        self.first_number <= occurrences && occurrences <= self.second_number
    }

    fn is_password_valid_2(&self) -> bool {
        let first_char = self.password.chars().nth(self.first_number - 1);
        let second_char = self.password.chars().nth(self.second_number - 1);

        first_char != second_char
            && (first_char == Some(self.character) || second_char == Some(self.character))
    }
}

impl TryFrom<&str> for PasswordRules {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        use regex::Regex;

        lazy_static! {
            static ref DAY_02_PASSWORD_RULE_REGEX: Regex = Regex::new(
                r"^(?P<first_number>\d+)-(?P<second_number>\d+) (?P<character>[a-z]): (?P<password>[a-z]+)$"
            )
            .expect("Invalid DAY_02_PASSWORD_RULE_REGEX!");
        }

        if let Some(cap) = DAY_02_PASSWORD_RULE_REGEX.captures(value) {
            let first_number = cap["first_number"].parse::<usize>()?;
            let second_number = cap["second_number"].parse::<usize>()?;

            if first_number > second_number {
                err!(
                    "First number should be less than or equal to second number: {}",
                    value
                )
            }

            Ok(Self {
                first_number,
                second_number,
                character: cap["character"].chars().next().unwrap(), // Safe unwrap ensured by the regex
                password: cap["password"].to_string(),
            })
        } else {
            err!("Couldn't parse input: {}", value)
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let password_rules = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&password_rules))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&password_rules))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<PasswordRules>> {
    input.lines().map(PasswordRules::try_from).collect()
}

fn part_1(password_rules: &[PasswordRules]) -> usize {
    password_rules
        .iter()
        .filter(|v| v.is_password_valid_1())
        .count()
}

fn part_2(password_rules: &[PasswordRules]) -> usize {
    password_rules
        .iter()
        .filter(|v| v.is_password_valid_2())
        .count()
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
    fn bench_parse_input(b: &mut Bencher) -> Result<()> {
        let input = read_input_file()?;
        b.iter(|| test::black_box(parse_input(&input)));
        Ok(())
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) -> Result<()> {
        let password_rules = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&password_rules)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let password_rules = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&password_rules)));
        Ok(())
    }
}
