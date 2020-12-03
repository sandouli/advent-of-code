#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:      67,188 ns/iter (+/- 11,135)
    test bench::bench_part_1      ... bench:     540,429 ns/iter (+/- 29,689)
    test bench::bench_part_2      ... bench:   3,979,029 ns/iter (+/- 1,322,497)


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

    let (line_length, max_line, tree_positions) = parse_input(&input)?;

    writeln!(
        io::stdout(),
        "Part 1 : {}",
        part_1(line_length, max_line, &tree_positions)
    )?;
    writeln!(
        io::stdout(),
        "Part 2 : {}",
        part_2(line_length, max_line, &tree_positions)
    )?;
    Ok(())
}

fn parse_input(input: &str) -> Result<(usize, usize, Vec<(usize, usize)>)> {
    let mut tree_positions = vec![];
    let mut line_length = None;
    let mut max_line = 0;

    for (i, line) in input.lines().enumerate() {
        max_line += 1;
        if let Some(length) = line_length {
            if line.len() != length {
                err!("Invalid input: every line should have the same length!")
            }
        } else {
            line_length = Some(line.len());
        }
        for (j, character) in line.chars().enumerate() {
            match character {
                '.' => {}
                '#' => tree_positions.push((j, i)),
                _ => err!(
                    "Invalid character found while parsing input : {}",
                    character
                ),
            }
        }
    }

    if let Some(length) = line_length {
        Ok((length, max_line, tree_positions))
    } else {
        err!("Input is empty!")
    }
}

fn part_1(line_length: usize, max_line: usize, tree_positions: &[(usize, usize)]) -> usize {
    //Path is Right 3, Down 1
    traverse_trees_slope(line_length, max_line, tree_positions, (3, 1))
}

fn part_2(line_length: usize, max_line: usize, tree_positions: &[(usize, usize)]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|v| traverse_trees_slope(line_length, max_line, tree_positions, *v))
        .product()
}

fn traverse_trees_slope(
    line_length: usize,
    max_line: usize,
    tree_positions: &[(usize, usize)],
    slope: (usize, usize),
) -> usize {
    let mut trees_crossed = 0;
    let mut current_x: usize = 0;
    let mut current_y: usize = 0;

    for _ in 0..(max_line - 1) {
        current_x += slope.0;
        current_y += slope.1;
        if tree_positions.contains(&(current_x % line_length, current_y)) {
            trees_crossed += 1;
        }
    }

    trees_crossed
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
        let (line_length, max_line, tree_positions) = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(line_length, max_line, &tree_positions), 7);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let (line_length, max_line, tree_positions) = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(line_length, max_line, &tree_positions), 336);
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
        let (line_length, max_line, tree_positions) = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(line_length, max_line, &tree_positions)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let (line_length, max_line, tree_positions) = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(line_length, max_line, &tree_positions)));
        Ok(())
    }
}
