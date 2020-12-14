#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:     877,667 ns/iter (+/- 37,690)
    test bench::bench_part_1      ... bench:     671,550 ns/iter (+/- 120,493)
    test bench::bench_part_2      ... bench:  73,346,001 ns/iter (+/- 1,989,818)

*/

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Clone)]
enum Command {
    Mask(Vec<char>),
    Mem(usize, usize),
}

#[derive(Clone)]
struct System {
    program: Vec<Command>,
    current_mask: Vec<char>,
    floating_bits: usize,
    memory: HashMap<usize, usize>,
}

impl System {
    fn execute_program(&mut self, decode_memory_address: bool) -> Result<()> {
        for command in &self.program {
            match command {
                Command::Mask(mask) => {
                    self.current_mask = mask.to_vec();
                    self.floating_bits = self.current_mask.iter().filter(|&v| *v == 'X').count();
                }
                Command::Mem(address, value) => {
                    if decode_memory_address {
                        let masked_address = self
                            .current_mask
                            .iter()
                            .zip(format!("{:036b}", address).chars())
                            .map(|(&mask, address)| if mask == '0' { address } else { mask })
                            .collect::<Vec<char>>();

                        for i in 0..2_u64.pow(self.floating_bits as u32) {
                            let mut bits_to_replace =
                                format!("{:b}", i).chars().collect::<Vec<char>>();
                            let decoded_address = masked_address
                                .iter()
                                .rev()
                                .map(|&v| {
                                    if v == 'X' {
                                        match bits_to_replace.pop() {
                                            Some(b) => b,
                                            None => '0',
                                        }
                                    } else {
                                        v
                                    }
                                })
                                .rev()
                                .collect::<String>();
                            if let Ok(resulting_address) =
                                usize::from_str_radix(&decoded_address, 2)
                            {
                                self.memory.insert(resulting_address, *value);
                            } else {
                                err!(
                                    "Could not parse binary address to usize : {:?}",
                                    decoded_address
                                )
                            }
                        }
                    } else {
                        let masked_value = self
                            .current_mask
                            .iter()
                            .zip(format!("{:036b}", value).chars())
                            .map(|(&mask, value)| if mask == 'X' { value } else { mask })
                            .collect::<String>();

                        if let Ok(resulting_value) = usize::from_str_radix(&masked_value, 2) {
                            self.memory.insert(*address, resulting_value);
                        } else {
                            err!("Could not parse binary value to usize : {:?}", masked_value)
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<&str> for System {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        use regex::Regex;
        lazy_static! {
            static ref DAY_14_PROGRAM_LINE_REGEX: Regex =
                Regex::new(r"^(?P<command>mask|mem\[(?P<address>\d+)\]) = ((?P<mask>[X01]{36})|(?P<value>\d+))$")
                    .expect("Invalid DAY_14_PROGRAM_LINE_REGEX!");
        }

        let mut program: Vec<Command> = vec![];

        for line in value.lines() {
            if let Some(cap) = DAY_14_PROGRAM_LINE_REGEX.captures(line) {
                match &cap["command"] {
                    "mask" => {
                        if let Some(mask) = cap.name("mask") {
                            program.push(Command::Mask(mask.as_str().chars().collect()));
                        } else {
                            err!("Invalid input mask : {}", line)
                        }
                    }
                    _ => {
                        if let Some(value) = cap.name("value") {
                            program.push(Command::Mem(
                                cap["address"].parse::<usize>()?,
                                value.as_str().parse::<usize>()?,
                            ))
                        } else {
                            err!("Invalid input memory value : {}", line)
                        }
                    }
                }
            } else {
                err!("Couldn't parse input : {}", line)
            }
        }

        Ok(System {
            program,
            current_mask: vec!['X'; 36],
            floating_bits: 36,
            memory: HashMap::new(),
        })
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let system = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(system.clone())?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(system)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<System> {
    System::try_from(input)
}

fn part_1(mut system: System) -> Result<usize> {
    system.execute_program(false)?;
    Ok(system.memory.iter().map(|(_, &v)| v).sum())
}

fn part_2(mut system: System) -> Result<usize> {
    system.execute_program(true)?;
    Ok(system.memory.iter().map(|(_, &v)| v).sum())
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
        let system = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(system)?, 165);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let system = parse_input(&read_test_file_2()?)?;
        assert_eq!(part_2(system)?, 208);
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
        let system = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(system.clone())));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let system = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(system.clone())));
        Ok(())
    }
}
