#![cfg_attr(feature = "unstable", feature(test))]
// Use this command to launch the program : cargo run --release < input/input.txt
// Use this command to benchmark : cargo +nightly bench --features "unstable"

use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let expense_report = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&expense_report)?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&expense_report)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let mut expense_report = vec![];
    for line in input.lines() {
        expense_report.push(line.parse::<usize>()?);
    }
    Ok(expense_report)
}

fn part_1(expense_report: &[usize]) -> Result<usize> {
    for i in 0..(expense_report.len() - 1) {
        for j in (i + 1)..expense_report.len() {
            if expense_report[i] + expense_report[j] == 2020 {
                return Ok(expense_report[i] * expense_report[j]);
            }
        }
    }

    err!("Part 1 : No combination found!")
}

fn part_2(expense_report: &[usize]) -> Result<usize> {
    for i in 0..(expense_report.len() - 2) {
        for j in (i + 1)..(expense_report.len() - 1) {
            for k in (j + 1)..expense_report.len() {
                if expense_report[i] + expense_report[j] + expense_report[k] == 2020 {
                    return Ok(expense_report[i] * expense_report[j] * expense_report[k]);
                }
            }
        }
    }

    err!("Part 2 : No combination found!")
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use std::fs::File;
    use test::Bencher;

    fn open_input_file() -> Result<Vec<usize>> {
        let mut input = String::new();
        File::open("input/input.txt")?.read_to_string(&mut input)?;
        parse_input(&input)
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) -> Result<()> {
        let expense_report = open_input_file()?;
        b.iter(|| test::black_box(part_1(&expense_report)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let expense_report = open_input_file()?;
        b.iter(|| test::black_box(part_2(&expense_report)));
        Ok(())
    }
}
