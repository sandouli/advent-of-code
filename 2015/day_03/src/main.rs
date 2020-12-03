#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 4 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_part_1 ... bench:     683,135 ns/iter (+/- 46,081)
    test bench::bench_part_2 ... bench:     699,459 ns/iter (+/- 59,966)

*/

use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&input)?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&input)?)?;
    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let mut x = 0;
    let mut y = 0;
    let mut positions = vec![(x, y)];

    for c in input.chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => err!("Invalid character found: {}", c),
        }
        positions.push((x, y));
    }

    let unique_positions: HashSet<(i32, i32)> = positions.iter().cloned().collect();
    Ok(unique_positions.len())
}

fn part_2(input: &str) -> Result<usize> {
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robot_x = 0;
    let mut robot_y = 0;

    let mut positions = vec![(santa_x, santa_y)];

    for (i, c) in input.chars().enumerate() {
        match c {
            '>' => {
                if i % 2 == 0 {
                    santa_x += 1;
                } else {
                    robot_x += 1;
                }
            }
            '<' => {
                if i % 2 == 0 {
                    santa_x -= 1;
                } else {
                    robot_x -= 1;
                }
            }
            '^' => {
                if i % 2 == 0 {
                    santa_y += 1;
                } else {
                    robot_y += 1;
                }
            }
            'v' => {
                if i % 2 == 0 {
                    santa_y -= 1;
                } else {
                    robot_y -= 1;
                }
            }
            _ => err!("Invalid character found: {}", c),
        }
        if i % 2 == 0 {
            positions.push((santa_x, santa_y));
        } else {
            positions.push((robot_x, robot_y));
        }
    }

    let unique_positions: HashSet<(i32, i32)> = positions.iter().cloned().collect();
    Ok(unique_positions.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part_1(">")?, 2);
        assert_eq!(part_1("^>v<")?, 4);
        assert_eq!(part_1("^v^v^v^v^v")?, 2);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part_2("^v")?, 3);
        assert_eq!(part_2("^>v<")?, 3);
        assert_eq!(part_2("^v^v^v^v^v")?, 11);
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
    fn bench_part_1(b: &mut Bencher) -> Result<()> {
        b.iter(|| test::black_box(part_1(&read_input_file()?)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        b.iter(|| test::black_box(part_2(&read_input_file()?)));
        Ok(())
    }
}
