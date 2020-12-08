#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:   6,387,662 ns/iter (+/- 142,930)
    test bench::bench_part_1      ... bench:     116,764 ns/iter (+/- 15,101)
    test bench::bench_part_2      ... bench:      27,432 ns/iter (+/- 2,558)

*/

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    contains: HashMap<String, usize>,
    contained_by: HashMap<String, usize>,
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bags = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&bags)?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&bags)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<HashMap<String, Bag>> {
    use regex::Regex;

    let mut bags: HashMap<String, Bag> = HashMap::new();

    lazy_static! {
        static ref DAY_07_CONTAINER_BAG_REGEX: Regex =
            Regex::new(r"^(?P<container_bag_color>[a-z]+ [a-z]+) bags contain (?P<contained_bags>(?:no other bags)|(\d+ [a-z]+ [a-z]+ bag(s)?)(, \d+ [a-z]+ [a-z]+ bag(s)?)*)\.$")
                .expect("Invalid DAY_07_CONTAINER_BAG_REGEX!");
        static ref DAY_07_CONTAINED_BAG_REGEX: Regex =
            Regex::new(r"(?P<contained_bag_number>\d+) (?P<contained_bag_color>[a-z]+ [a-z]+)")
                .expect("Invalid DAY_07_CONTAINED_BAG_REGEX!");
    }

    for line in input.lines() {
        if let Some(cap) = DAY_07_CONTAINER_BAG_REGEX.captures(line) {
            let current_bag = match bags.entry(cap["container_bag_color"].into()) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(Bag {
                    color: cap["container_bag_color"].to_string(),
                    contains: HashMap::new(),
                    contained_by: HashMap::new(),
                }),
            };

            if &cap["contained_bags"] != "no other bags" {
                for subcap in DAY_07_CONTAINED_BAG_REGEX.captures_iter(&cap["contained_bags"]) {
                    if current_bag
                        .contains
                        .insert(
                            subcap["contained_bag_color"].to_string(),
                            subcap["contained_bag_number"].parse::<usize>()?,
                        )
                        .is_some()
                    {
                        err!(
                            "Current bag already contains this bag color : {} => {}",
                            &cap["container_bag_color"],
                            &subcap["contained_bag_color"]
                        )
                    }
                }
            }
        } else {
            err!("Couldn't parse input line : {}", line)
        }
    }

    for (bag_color, bag) in &bags.clone() {
        // Cannot borrow as a mutable more than once, but can clone this one to update the real one
        for (contained_bag_color, contained_bag_number) in &bag.contains {
            let contained_bag = match bags.entry(contained_bag_color.into()) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(Bag {
                    color: contained_bag_color.clone(),
                    contains: HashMap::new(),
                    contained_by: HashMap::new(),
                }),
            };
            if contained_bag
                .contained_by
                .insert(bag_color.clone(), *contained_bag_number)
                .is_some()
            {
                err!(
                    "Current bag already is already contained by this bag color : {} => {}",
                    &contained_bag_color,
                    &bag_color
                )
            }
        }
    }

    Ok(bags)
}

fn part_1(bags: &HashMap<String, Bag>) -> Result<usize> {
    let mut bags_checked: HashSet<String> = HashSet::new();
    let mut bags_to_check: Vec<String> = vec![];

    if let Some(shiny_gold_bag) = bags.get("shiny gold") {
        bags_to_check = shiny_gold_bag
            .contained_by
            .iter()
            .map(|(k, _)| k.clone())
            .collect();
    } else {
        err!("Couldn't find the shiny gold bag !")
    }

    while let Some(current_bag) = bags_to_check.pop() {
        if bags_checked.insert(current_bag.clone()) {
            if let Some(container_bag) = bags.get(&current_bag) {
                bags_to_check.extend(
                    container_bag
                        .contained_by
                        .iter()
                        .map(|(k, _)| k.clone())
                        .collect::<Vec<String>>(),
                );
            } else {
                err!("Couldn't find a container bag : {}", &current_bag)
            }
        }
    }

    Ok(bags_checked.len())
}

fn part_2(bags: &HashMap<String, Bag>) -> Result<usize> {
    let mut number_of_bags = 0;
    let mut bags_to_check: Vec<(String, usize)> = vec![];

    if let Some(shiny_gold_bag) = bags.get("shiny gold") {
        bags_to_check = shiny_gold_bag
            .contains
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
    } else {
        err!("Couldn't find the shiny gold bag !")
    }

    while let Some((current_bag_color, current_bag_number)) = bags_to_check.pop() {
        if let Some(contained_bag) = bags.get(&current_bag_color) {
            number_of_bags += current_bag_number;
            bags_to_check.extend(
                contained_bag
                    .contains
                    .iter()
                    .map(|(k, v)| (k.clone(), v * current_bag_number))
                    .collect::<Vec<(String, usize)>>(),
            );
        } else {
            err!("Couldn't find a container bag : {}", &current_bag_color)
        }
    }

    Ok(number_of_bags)
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
        let bags = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&bags)?, 4);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let bags = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(&bags)?, 32);
        let bags = parse_input(&read_test_file_2()?)?;
        assert_eq!(part_2(&bags)?, 126);
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
        let bags = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&bags)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let bags = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&bags)));
        Ok(())
    }
}
