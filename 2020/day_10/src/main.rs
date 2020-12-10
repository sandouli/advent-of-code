#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:       5,684 ns/iter (+/- 435)
    test bench::bench_part_1      ... bench:         811 ns/iter (+/- 44)
    test bench::bench_part_2      ... bench:       1,292 ns/iter (+/- 47)

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

    let adapters = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&adapters)?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&adapters)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let mut adapters = vec![];

    for line in input.lines() {
        adapters.push(line.parse::<usize>()?);
    }

    adapters.sort_unstable();

    Ok(adapters)
}

fn part_1(adapters: &[usize]) -> Result<usize> {
    let mut difference_1 = 0;
    let mut difference_3 = 1;

    for i in 0..adapters.len() {
        let difference = if i == 0 {
            adapters[i]
        } else {
            adapters[i] - adapters[i - 1]
        };
        match difference {
            0 | 2 => {}
            1 => difference_1 += 1,
            3 => difference_3 += 1,
            other_value => err!(
                "Difference between two adapters can't be greater than 3 : {}",
                other_value
            ),
        }
    }

    Ok(difference_1 * difference_3)
}

fn part_2(adapters: &[usize]) -> Result<usize> {
    let mut differences = vec![];
    let mut ones_found = 0;
    let mut result = 1;

    for i in 0..adapters.len() {
        let difference = if i == 0 {
            adapters[i]
        } else {
            adapters[i] - adapters[i - 1]
        };
        differences.push(difference);
    }
    differences.push(3);

    /*
    Not quite happy with this solution.
    In the case of adapters with 5 or more consecutive diff of 1 (eg:"(0), 1, 2, 3, 4, 5, 6, (9)"), the answer would be wrong.
    But I can't find any case where this happens
    */

    for diff in differences {
        if diff == 1 {
            ones_found += 1;
        } else {
            match ones_found {
                0 | 1 => {}
                2 => result *= 2,
                3 => result *= 4,
                4 => result *= 7,
                _ => err!("Too many consecutive differences of 1 found, need to update the algorithm!"),
            }
            ones_found = 0;
        }
    }

    Ok(result)
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
        let adapters = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&adapters)?, 7 * 5);
        let adapters = parse_input(&read_test_file_2()?)?;
        assert_eq!(part_1(&adapters)?, 22 * 10);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let adapters = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(&adapters)?, 8);
        let adapters = parse_input(&read_test_file_2()?)?;
        assert_eq!(part_2(&adapters)?, 19208);
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
        let adapters = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&adapters)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let adapters = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&adapters)));
        Ok(())
    }
}
