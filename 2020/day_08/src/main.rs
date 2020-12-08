#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:     584,677 ns/iter (+/- 32,414)
    test bench::bench_part_1      ... bench:      25,170 ns/iter (+/- 2,822)
    test bench::bench_part_2      ... bench:   4,322,023 ns/iter (+/- 341,828)

*/

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Clone)]
enum Command {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "nop" => Ok(Command::Nop),
            "acc" => Ok(Command::Acc),
            "jmp" => Ok(Command::Jmp),
            _ => err!("Could not determine the command : {}", input),
        }
    }
}

#[derive(Clone)]
struct Processor {
    program: Vec<(Command, isize)>,
    current_position: isize,
    accumulator: isize,
}

impl Processor {
    fn next_step(&mut self) -> Result<()> {
        if self.current_position >= 0 {
            if let Some(program_line) = self.program.get(self.current_position as usize) {
                match program_line {
                    (Command::Nop, _) => self.current_position += 1,
                    (Command::Acc, number) => {
                        self.accumulator += number;
                        self.current_position += 1;
                    }
                    (Command::Jmp, number) => self.current_position += number,
                }
            } else {
                err!(
                    "Program is out of bounds : Accumulator = {}; Position = {}",
                    self.accumulator,
                    self.current_position
                )
            }
        } else {
            err!(
                "Program current position can't be negative : {}",
                self.current_position
            )
        }
        Ok(())
    }

    fn is_program_terminated(&self) -> bool {
        self.current_position >= 0 && self.current_position as usize == self.program.len()
    }
}

impl TryFrom<&str> for Processor {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        use regex::Regex;
        lazy_static! {
            static ref DAY_08_PROGRAM_LINE_REGEX: Regex =
                Regex::new(r"^(?P<command>jmp|acc|nop) (?P<number>[+-]\d+)$")
                    .expect("Invalid DAY_08_PROGRAM_LINE_REGEX!");
        }

        let mut program: Vec<(Command, isize)> = vec![];

        for line in value.lines() {
            if let Some(cap) = DAY_08_PROGRAM_LINE_REGEX.captures(line) {
                program.push((
                    Command::from_str(&cap["command"])?,
                    cap["number"].parse::<isize>()?,
                ));
            } else {
                err!("Couldn't parse input : {}", line)
            }
        }

        Ok(Processor {
            program,
            current_position: 0,
            accumulator: 0,
        })
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let processor = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(processor.clone())?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(processor)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Processor> {
    Processor::try_from(input)
}

fn part_1(mut processor: Processor) -> Result<isize> {
    let mut positions_executed: HashSet<isize> = HashSet::new();

    while positions_executed.insert(processor.current_position) {
        processor.next_step()?;
    }

    Ok(processor.accumulator)
}

fn part_2(processor: Processor) -> Result<isize> {
    'outer: for i in 0..processor.program.len() {
        let mut processor = processor.clone();
        match processor.program[i] {
            (Command::Nop, number) => processor.program[i] = (Command::Jmp, number),
            (Command::Jmp, number) => processor.program[i] = (Command::Nop, number),
            _ => {}
        }

        let mut positions_executed: HashSet<isize> = HashSet::new();
        loop {
            if processor.is_program_terminated() {
                return Ok(processor.accumulator);
            }
            match positions_executed.insert(processor.current_position) {
                true => processor.next_step()?,
                false => continue 'outer,
            }
        }
    }

    err!("Couldn't find a swap that lets us finish the program")
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

    #[test]
    fn test_part_1() -> Result<()> {
        let processor = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(processor)?, 5);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let processor = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(processor)?, 8);
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
        let processor = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(processor.clone())));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let processor = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(processor.clone())));
        Ok(())
    }
}
