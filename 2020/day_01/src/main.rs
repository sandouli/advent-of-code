#![cfg_attr(feature = "unstable", feature(test))]

// Lauch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

* When not sorting expenses during parsing
    running 3 tests
    test bench::bench_parse_input ... bench:       8,950 ns/iter (+/- 727)
    test bench::bench_part_1      ... bench:      12,541 ns/iter (+/- 1,166)
    test bench::bench_part_2      ... bench:   1,598,317 ns/iter (+/- 93,409)

* When sorting expenses during parsing
    running 3 tests
    test bench::bench_parse_input ... bench:      11,818 ns/iter (+/- 573)
    test bench::bench_part_1      ... bench:         535 ns/iter (+/- 52)
    test bench::bench_part_2      ... bench:      25,408 ns/iter (+/- 1,038)

*/

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
    expense_report.sort_unstable();
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
        let expense_report = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&expense_report)?, 514579);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let expense_report = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(&expense_report)?, 241861950);
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
        let expense_report = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&expense_report)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let expense_report = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&expense_report)));
        Ok(())
    }
}
