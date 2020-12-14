#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:       2,488 ns/iter (+/- 338)
    test bench::bench_part_1      ... bench:         141 ns/iter (+/- 7)
    test bench::bench_part_2      ... bench:      14,131 ns/iter (+/- 1,273)

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

    let (earliest_depart_time, buses) = parse_input(&input)?;

    writeln!(
        io::stdout(),
        "Part 1 : {}",
        part_1(earliest_depart_time, &buses)?
    )?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&buses))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<(usize, Vec<usize>)> {
    let mut earliest_depart_time = 0;
    let mut buses = vec![];

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            earliest_depart_time = line.parse::<usize>()?;
        } else {
            for bus in line.split(',') {
                if bus != "x" {
                    buses.push(bus.parse::<usize>()?);
                } else {
                    buses.push(1);
                }
            }
        }
    }

    Ok((earliest_depart_time, buses))
}

fn part_1(earliest_depart_time: usize, buses: &[usize]) -> Result<usize> {
    if let Some((bus, waiting)) = buses
        .iter()
        .filter(|&v| *v != 1)
        .map(|bus_id| (bus_id, bus_id - (earliest_depart_time % bus_id)))
        .min_by(|a, b| a.1.cmp(&b.1))
    {
        Ok(bus * waiting)
    } else {
        err!("Could not find a minimum, is the input empty?")
    }
}

fn part_2(buses: &[usize]) -> usize {
    // Chinese remainder theorem
    // Only works since all buses ID are prime numbers

    let global_coprime: usize = buses.iter().product();

    let mut factors = vec![];

    for (i, bus) in buses.iter().rev().enumerate() {
        let current_factor = global_coprime / bus;
        let mut j = 1;
        while (current_factor * j) % bus != i % bus {
            j += 1;
        }
        factors.push(current_factor * j);
    }

    factors.iter().sum::<usize>() % global_coprime - buses.len() + 1
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
        let (earliest_depart_time, buses) = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(earliest_depart_time, &buses)?, 295);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let (_, buses) = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(&buses), 1068781);
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
        let (earliest_depart_time, buses) = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(earliest_depart_time, &buses)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let (_, buses) = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&buses)));
        Ok(())
    }
}
