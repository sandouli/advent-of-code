#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:      55,493 ns/iter (+/- 3,820)
    test bench::bench_part_1      ... bench:      60,718 ns/iter (+/- 3,898)
    test bench::bench_part_2      ... bench:     189,749 ns/iter (+/- 17,608)

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

    let numbers = parse_input(&input)?;
    let preamble = 25;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&numbers, preamble)?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&numbers, preamble)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let mut numbers: Vec<usize> = vec![];

    for line in input.lines() {
        numbers.push(line.parse::<usize>()?);
    }

    Ok(numbers)
}

fn part_1(numbers: &[usize], preamble: usize) -> Result<usize> {
    if preamble > numbers.len() {
        err!("Not enough numbers for the preamble")
    }

    let mut current_combinations: Vec<usize> = numbers.iter().take(preamble).copied().collect();

    'outer: for &number_to_find in numbers.iter().skip(preamble) {
        for i in 0..(current_combinations.len() - 1) {
            for j in (i + 1)..current_combinations.len() {
                if current_combinations[i] == current_combinations[j] {
                    continue;
                }

                if current_combinations[i] + current_combinations[j] == number_to_find {
                    current_combinations.remove(0);
                    current_combinations.push(number_to_find);
                    continue 'outer;
                }
            }
        }

        return Ok(number_to_find);
    }

    err!("No combination in error found")
}

fn part_2(numbers: &[usize], preamble: usize) -> Result<usize> {
    let number_to_find = part_1(numbers, preamble)?;

    'outer: for i in 0..(numbers.len() - 1) {
        let mut current_addition = numbers[i];
        for j in (i + 1)..numbers.len() {
            current_addition += numbers[j];
            if current_addition > number_to_find {
                continue 'outer;
            } else if current_addition == number_to_find {
                return Ok(
                    numbers[i..=j].iter().min().unwrap() + numbers[i..=j].iter().max().unwrap()
                ); // Safe unwraps here since there are always at least two numbers in the range
            }
        }
    }

    err!("No contiguous series found")
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
        let numbers = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&numbers, 5)?, 127);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let numbers = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(&numbers, 5)?, 62);
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
        let numbers = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&numbers, 25)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let numbers = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&numbers, 25)));
        Ok(())
    }
}
