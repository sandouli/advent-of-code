#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:         347 ns/iter (+/- 38)
    test bench::bench_part_1      ... bench:     121,703 ns/iter (+/- 6,267)
    test bench::bench_part_2      ... bench: 3,809,101,967 ns/iter (+/- 314,283,380)

*/

use std::collections::HashMap;
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

    writeln!(io::stdout(), "Part 1 : {}", part_1(&numbers))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&numbers))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let mut numbers = vec![];
    for number in input.split(',') {
        numbers.push(number.parse::<usize>()?);
    }
    Ok(numbers)
}

fn part_1(numbers: &[usize]) -> usize {
    execute_turns(numbers, 2020)
}

fn part_2(numbers: &[usize]) -> usize {
    execute_turns(numbers, 30000000)
}

fn execute_turns(numbers: &[usize], final_turn: usize) -> usize {
    let mut spoken_numbers: HashMap<usize, usize> = HashMap::new();
    let mut last_number_spoken = 0;

    for (i, number) in numbers.iter().enumerate() {
        if i != 0 {
            spoken_numbers.insert(last_number_spoken, i);
        }
        last_number_spoken = *number;
    }

    for i in numbers.len()..final_turn {
        last_number_spoken = match spoken_numbers.insert(last_number_spoken, i) {
            Some(age) => i - age,
            None => 0,
        };
    }

    last_number_spoken
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
        assert_eq!(execute_turns(&numbers, 10), 0);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let numbers = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(&numbers), 175594);
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
        b.iter(|| test::black_box(part_1(&numbers)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let numbers = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&numbers)));
        Ok(())
    }
}
