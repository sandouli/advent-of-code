#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:     395,472 ns/iter (+/- 30,503)
    test bench::bench_part_1      ... bench:      53,094 ns/iter (+/- 2,143)
    test bench::bench_part_2      ... bench:     728,510 ns/iter (+/- 38,054)

*/

#[macro_use]
extern crate lazy_static;
extern crate regex;

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

    let (ticket_rules, my_ticket, nearby_tickets) = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(&ticket_rules, &nearby_tickets))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(&ticket_rules, &my_ticket, &nearby_tickets))?;
    Ok(())
}

fn parse_input(
    input: &str,
) -> Result<(
    HashMap<String, ((usize, usize), (usize, usize))>,
    Vec<usize>,
    Vec<Vec<usize>>,
)> {
    let mut ticket_rules: HashMap<String, ((usize, usize), (usize, usize))> = HashMap::new();
    let mut my_ticket: Vec<usize> = vec![];
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];

    for (i, input_split) in input.split("\n\n").enumerate() {
        match i {
            0 => {
                // Ticket rules
                use regex::Regex;
                lazy_static! {
                    static ref DAY_16_TICKET_RULES_REGEX: Regex =
                        Regex::new(r"^(?P<rule>[a-z ]+): (?P<number_1>\d+)-(?P<number_2>\d+) or (?P<number_3>\d+)-(?P<number_4>\d+)$")
                            .expect("Invalid DAY_16_TICKET_RULES_REGEX!");
                }
                for line in input_split.lines() {
                    if let Some(cap) = DAY_16_TICKET_RULES_REGEX.captures(line) {
                        match ticket_rules.insert(
                            cap["rule"].to_string(),
                            (
                                (
                                    cap["number_1"].parse::<usize>()?,
                                    cap["number_2"].parse::<usize>()?,
                                ),
                                (
                                    cap["number_3"].parse::<usize>()?,
                                    cap["number_4"].parse::<usize>()?,
                                ),
                            ),
                        ) {
                            Some(_) => err!("Ticket rule is defined twice : {}", &cap["rule"]),
                            None => {}
                        }
                    } else {
                        err!("Couldn't parse input : {}", line)
                    }
                }
            }
            1 => {
                // My ticket
                for (j, line) in input_split.lines().enumerate() {
                    match j {
                        0 => {}
                        1 => {
                            for number in line.split(',') {
                                my_ticket.push(number.parse::<usize>()?);
                            }
                        }
                        _ => err!("Input invalid : my ticket have too many lines"),
                    }
                }
            }
            2 => {
                for (j, line) in input_split.lines().enumerate() {
                    match j {
                        0 => {}
                        _ => {
                            let mut current_nearby_ticket = vec![];
                            for number in line.split(',') {
                                current_nearby_ticket.push(number.parse::<usize>()?);
                            }
                            nearby_tickets.push(current_nearby_ticket);
                        }
                    }
                }
            }
            _ => err!("Invalid input"),
        }
    }

    Ok((ticket_rules, my_ticket, nearby_tickets))
}

fn part_1(
    ticket_rules: &HashMap<String, ((usize, usize), (usize, usize))>,
    nearby_tickets: &[Vec<usize>]
) -> usize {
    let mut result = 0;

    for current_nearby_ticket in nearby_tickets {
        'inner: for &number in current_nearby_ticket {
            for rule in ticket_rules.values() {
                if (number >= rule.0.0 && number <= rule.0.1)
                    || (number >= rule.1.0 && number <= rule.1.1)
                {
                    continue 'inner;
                }
            }
            result += number;
        }
    }

    result
}

fn part_2(ticket_rules: &HashMap<String, ((usize, usize), (usize, usize))>, my_ticket: &[usize], nearby_tickets: &[Vec<usize>]) -> usize {
    let mut valid_nearby_tickets: Vec<Vec<usize>> = vec![];

    let mut possible_positions: HashMap<String, Vec<usize>> = ticket_rules
        .iter()
        .map(|(rule_name, _)| (rule_name.to_string(), (0..my_ticket.len()).collect()))
        .collect();


    // Filter invalid nearby tickets
    'outer: for current_nearby_ticket in nearby_tickets {
        valid_nearby_tickets.push(current_nearby_ticket.clone());
        'inner: for &number in current_nearby_ticket {
            for rule in ticket_rules.values() {
                if (number >= rule.0.0 && number <= rule.0.1)
                    || (number >= rule.1.0 && number <= rule.1.1) {
                    continue 'inner;
                }
            }
            valid_nearby_tickets.pop();
            continue 'outer;
        }
    }

    for (rule_name, rule) in ticket_rules {
        let mut current_invalid_positions = vec![];
        for current_valid_nearby_ticket in &valid_nearby_tickets {
            let mut x: Vec<usize> = current_valid_nearby_ticket
                .iter()
                .enumerate()
                .filter(|(_, &v)| !((v >= rule.0.0 && v <= rule.0.1) || (v >= rule.1.0 && v <= rule.1.1)))
                .map(|(i, _)| i)
                .collect();
            current_invalid_positions.append(&mut x);
        }
        current_invalid_positions.sort_unstable();
        current_invalid_positions.dedup();
        if let Some(p) = possible_positions.get_mut(rule_name) {
            *p = p
                .iter()
                .filter(|v| !current_invalid_positions.contains(v))
                .copied()
                .collect();
        } else {
            unreachable!();
        }
    }

    let mut result = 1;

    let mut positions_found = vec![];
    let mut last_position_found = 0;

    loop {
        let mut new_position_found = false;
        for (rule_name, positions) in &possible_positions {
            if positions.len() == 1 {
                last_position_found = positions[0];
                positions_found.push(last_position_found);
                new_position_found = true;
                if rule_name.starts_with("departure") {
                    result *= my_ticket[last_position_found];
                }
            }
        }

        if new_position_found {
            for (_, positions) in possible_positions.iter_mut() {
                *positions = positions
                    .iter()
                    .filter(|&v| *v != last_position_found)
                    .copied()
                    .collect()
            }
        } else {
            break
        }
    }

    result
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
        let (ticket_rules, _, nearby_tickets) = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(&ticket_rules, &nearby_tickets), 71);
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
        let (ticket_rules, _, nearby_tickets) = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(&ticket_rules, &nearby_tickets)));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let (ticket_rules, my_ticket, nearby_tickets) = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(&ticket_rules, &my_ticket, &nearby_tickets)));
        Ok(())
    }
}
