#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 4 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_part_1 ... bench:     118,604 ns/iter (+/- 11,914)
    test bench::bench_part_2 ... bench:     100,205 ns/iter (+/- 26,558)

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

    writeln!(io::stdout(), "Part 1 : {}", part_1(&input)?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&input)?)?;
    Ok(())
}

fn part_1(input: &str) -> Result<isize> {
    use std::convert::TryFrom;

    Ok(
        isize::try_from(input.chars().filter(|&v| v == '(').count())?
            - isize::try_from(input.chars().filter(|&v| v == ')').count())?,
    )
}

fn part_2(input: &str) -> Result<usize> {
    let mut current_floor = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => err!("Invalid character found: {}", c),
        }
        if current_floor < 0 {
            return Ok(i + 1);
        }
    }

    err!("Part 2 : No position found where going to the basement!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part_1("(())")?, 0);
        assert_eq!(part_1("()()")?, 0);
        assert_eq!(part_1("(((")?, 3);
        assert_eq!(part_1("(()(()(")?, 3);
        assert_eq!(part_1("))(((((")?, 3);
        assert_eq!(part_1("())")?, -1);
        assert_eq!(part_1("))(")?, -1);
        assert_eq!(part_1(")))")?, -3);
        assert_eq!(part_1(")())())")?, -3);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part_2(")")?, 1);
        assert_eq!(part_2("()())")?, 5);
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
