#[macro_use]
extern crate lazy_static;
extern crate regex;

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
        let occurrences = self
            .password
            .matches(self.character)
            .count();
        self.first_number <= occurrences && occurrences <= self.second_number
    }

    fn is_password_valid_2(&self) -> bool {
        let first_char = self
            .password
            .chars()
            .nth(self.first_number - 1);
        let second_char = self
            .password
            .chars()
            .nth(self.second_number - 1);

        first_char != second_char
            && (first_char == Some(self.character)
                || second_char == Some(self.character))
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let password_rules: Vec<PasswordRules> = parse_input(&input)?;

    part_1(&password_rules)?;
    part_2(&password_rules)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<PasswordRules>> {
    use regex::Regex;

    let mut password_rules = vec![];

    lazy_static! {
        static ref DAY_02_PASSWORD_RULE_REGEX: Regex = Regex::new(
            r"^(?P<minimum>\d+)-(?P<maximum>\d+) (?P<character>[a-z]): (?P<password>[a-z]+)$"
        )
        .expect("Invalid DAY_02_PASSWORD_RULE_REGEX!");
    }

    for line in input.lines() {
        if let Some(cap) = DAY_02_PASSWORD_RULE_REGEX.captures(line) {
            let first_number = cap["minimum"].parse::<usize>()?;
            let second_number = cap["maximum"].parse::<usize>()?;

            if first_number > second_number {
                err!(
                    "First number should be less than or equal to second number: {}",
                    line
                )
            }

            password_rules.push(PasswordRules {
                first_number,
                second_number,
                character: cap["character"].chars().next().unwrap(), // Safe unwrap ensured by the regex
                password: cap["password"].to_string(),
            });
        } else {
            err!("Couldn't parse input: {}", line)
        }
    }

    Ok(password_rules)
}

fn part_1(password_rules: &[PasswordRules]) -> Result<()> {
    let valid_passwords = password_rules.iter().filter(|v| v.is_password_valid_1()).count();
    writeln!(io::stdout(), "Part 1 : {}", valid_passwords)?;
    Ok(())
}

fn part_2(password_rules: &[PasswordRules]) -> Result<()> {
    let valid_passwords = password_rules.iter().filter(|v| v.is_password_valid_2()).count();
    writeln!(io::stdout(), "Part 2 : {}", valid_passwords)?;
    Ok(())
}
