#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:     889,796 ns/iter (+/- 52,820)
    test bench::bench_part_1      ... bench:   1,108,069 ns/iter (+/- 84,047)
    test bench::bench_part_2      ... bench:   1,240,781 ns/iter (+/- 89,467)

*/

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Debug)]
struct Group {
    answers: Vec<Vec<char>>,
}

impl Group {
    fn get_unique_answers(&self) -> HashSet<char> {
        let mut unique_answers = HashSet::new();

        for answer in self.answers.iter() {
            for c in answer {
                unique_answers.insert(*c);
            }
        }

        unique_answers
    }

    fn get_common_answers(&self) -> Vec<char> {
        let mut answers: HashMap<char, usize> = HashMap::new();
        for (i, answer) in self.answers.iter().enumerate() {
            for c in answer.iter() {
                if i == 0 {
                    answers.insert(*c, 1);
                } else if let Some(a) = answers.get_mut(&c) {
                    *a += 1;
                }
            }
        }

        answers
            .into_iter()
            .filter(|&(_, v)| v == self.answers.len())
            .map(|(k, _)| k)
            .collect()
    }
}

impl TryFrom<&str> for Group {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        let mut answers = vec![];
        for line in value.lines() {
            let mut currents_answers = vec![];
            for c in line.chars() {
                match c {
                    'a'..='z' => {
                        currents_answers.push(c);
                    }
                    _ => err!("Invalid input : {}", c),
                }
            }
            answers.push(currents_answers);
        }
        Ok(Group { answers })
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let groups = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&groups))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&groups))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Group>> {
    input.split("\n\n").map(Group::try_from).collect()
}

fn part_1(groups: &[Group]) -> usize {
    groups.iter().map(|v| v.get_unique_answers().len()).sum()
}

fn part_2(groups: &[Group]) -> usize {
    groups.iter().map(|v| v.get_common_answers().len()).sum()
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
        let groups = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&groups), 11);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let groups = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(&groups), 6);
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
        let groups = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&groups)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let groups = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&groups)));
        Ok(())
    }
}
