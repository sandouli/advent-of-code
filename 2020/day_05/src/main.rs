#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 4 tests
    test tests::test_part_1 ... ignored
    test bench::bench_parse_input ... bench:      98,888 ns/iter (+/- 9,621)
    test bench::bench_part_1      ... bench:         681 ns/iter (+/- 83)
    test bench::bench_part_2      ... bench:       3,128 ns/iter (+/- 224)

*/

use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn get_seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl TryFrom<&str> for Seat {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() != 10 {
            err!("Input line should have a length of 10 chars")
        } else {
            let mut max_row = 127;
            let mut min_row = 0;
            let mut max_column = 7;
            let mut min_column = 0;

            for r in value[..7].chars() {
                match r {
                    'F' => max_row = (max_row + min_row - 1) / 2,
                    'B' => min_row = (max_row + min_row + 1) / 2,
                    _ => err!("Invalid character found while determining row : {}", r),
                }
            }

            for c in value[7..].chars() {
                match c {
                    'L' => max_column = (max_column + min_column - 1) / 2,
                    'R' => min_column = (max_column + min_column + 1) / 2,
                    _ => err!("Invalid character found while determining column : {}", c),
                }
            }

            Ok(Seat {
                row: max_row,
                column: max_column,
            })
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut seats = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&seats)?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&mut seats)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Seat>> {
    input.lines().map(Seat::try_from).collect()
}

fn part_1(seats: &[Seat]) -> Result<usize> {
    match seats.iter().map(|v| v.get_seat_id()).max() {
        Some(max) => Ok(max),
        None => err!("Input is empty!"),
    }
}

fn part_2(seats: &mut Vec<Seat>) -> Result<usize> {
    seats.sort();
    let mut current_seat_id = 0;

    for (i, seat) in seats.iter().enumerate() {
        if i == 0 {
            current_seat_id = seat.get_seat_id();
        } else {
            current_seat_id += 1;
            if current_seat_id != seat.get_seat_id() {
                return Ok(seat.get_seat_id() - 1);
            }
        }
    }
    err!("Couldn't find santa's seat!")
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
        let seats = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&seats)?, 820);
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
        let seats = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&seats)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let mut seats = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&mut seats)));
        Ok(())
    }
}
