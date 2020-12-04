#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:   1,105,911 ns/iter (+/- 59,725)
    test bench::bench_part_1      ... bench:       1,567 ns/iter (+/- 87)
    test bench::bench_part_2      ... bench:     633,970 ns/iter (+/- 66,265)

*/

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_2(&self) -> bool {
        lazy_static! {
            static ref DAY_04_YEAR_REGEX: Regex =
                Regex::new(r"^(?P<year>\d{4})$").expect("Invalid DAY_04_YEAR_REGEX!");
            static ref DAY_04_HEIGHT_REGEX: Regex =
                Regex::new(r"^(?P<height>\d{2,3})(?P<measurement>(cm)|(in))$")
                    .expect("Invalid DAY_04_HEIGHT_REGEX!");
            static ref DAY_04_COLOR_REGEX: Regex =
                Regex::new(r"^(?P<color>#[0-9a-f]{6})$").expect("Invalid DAY_04_COLOR_REGEX!");
            static ref DAY_04_PID_REGEX: Regex =
                Regex::new(r"^(?P<pid>\d{9})$").expect("Invalid DAY_04_PID_REGEX!");
        }
        if let Some(ref byr) = self.byr {
            if let Some(cap) = DAY_04_YEAR_REGEX.captures(&byr) {
                let year = cap["year"].parse::<usize>().unwrap(); // Safe unwrap ensured by the regex
                if year < 1920 || year > 2002 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(ref iyr) = self.iyr {
            if let Some(cap) = DAY_04_YEAR_REGEX.captures(&iyr) {
                let year = cap["year"].parse::<usize>().unwrap(); // Safe unwrap ensured by the regex
                if year < 2010 || year > 2020 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(ref eyr) = self.eyr {
            if let Some(cap) = DAY_04_YEAR_REGEX.captures(&eyr) {
                let year = cap["year"].parse::<usize>().unwrap(); // Safe unwrap ensured by the regex
                if year < 2020 || year > 2030 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(ref hgt) = self.hgt {
            if let Some(cap) = DAY_04_HEIGHT_REGEX.captures(&hgt) {
                let height = cap["height"].parse::<usize>().unwrap(); // Safe unwrap ensured by the regex
                match &cap["measurement"] {
                    "cm" => {
                        if height < 150 || height > 193 {
                            return false;
                        }
                    }
                    "in" => {
                        if height < 59 || height > 76 {
                            return false;
                        }
                    }
                    _ => unreachable!("DAY_04_HEIGHT_REGEX should have taken care of all cases"),
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(ref hcl) = self.hcl {
            if DAY_04_COLOR_REGEX.captures(&hcl).is_none() {
                return false;
            }
        } else {
            return false;
        }
        if let Some(ref ecl) = self.ecl {
            match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => return false,
            }
        } else {
            return false;
        }
        if let Some(ref pid) = self.pid {
            if DAY_04_PID_REGEX.captures(&pid).is_none() {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&passports))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&passports))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Passport>> {
    let mut passports = vec![];

    let mut current_passport = Passport {
        ..Default::default()
    };

    for line in input.lines() {
        if line.is_empty() {
            passports.push(current_passport);
            current_passport = Passport {
                ..Default::default()
            };
        } else {
            for split_line in line.split(' ') {
                let key_value: Vec<&str> = split_line.split(':').collect();
                if key_value.len() == 2 {
                    match key_value[0] {
                        "byr" => current_passport.byr = Some(key_value[1].into()),
                        "iyr" => current_passport.iyr = Some(key_value[1].into()),
                        "eyr" => current_passport.eyr = Some(key_value[1].into()),
                        "hgt" => current_passport.hgt = Some(key_value[1].into()),
                        "hcl" => current_passport.hcl = Some(key_value[1].into()),
                        "ecl" => current_passport.ecl = Some(key_value[1].into()),
                        "pid" => current_passport.pid = Some(key_value[1].into()),
                        "cid" => current_passport.cid = Some(key_value[1].into()),
                        _ => err!("Invalid passport key found : {}", split_line),
                    }
                } else {
                    err!("Invalid input : {}", split_line)
                }
            }
        }
    }

    passports.push(current_passport);

    Ok(passports)
}

fn part_1(passports: &[Passport]) -> usize {
    passports.iter().filter(|v| v.is_valid()).count()
}

fn part_2(passports: &[Passport]) -> usize {
    passports.iter().filter(|v| v.is_valid_2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn read_test_file() -> Result<String> {
        let mut input = String::new();
        File::open("input/test.txt")?.read_to_string(&mut input)?;
        Ok(input)
    }

    fn read_test_file_2() -> Result<String> {
        let mut input = String::new();
        File::open("input/test2.txt")?.read_to_string(&mut input)?;
        Ok(input)
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let passports = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&passports), 2);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let passports = parse_input(&read_test_file_2()?)?;
        assert_eq!(part_2(&passports), 4);
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
    fn bench_parse_input(b: &mut Bencher) -> Result<()> {
        let input = read_input_file()?;
        b.iter(|| test::black_box(parse_input(&input)));
        Ok(())
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) -> Result<()> {
        let passports = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&passports)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let passports = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&passports)));
        Ok(())
    }
}
