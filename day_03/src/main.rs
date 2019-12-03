#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::{self, Read, Write};
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    let coordinates = parse_input(input)?;

    let mut min_distance = ::std::u64::MAX;
    use std::cmp;

    for (coord, wires) in &coordinates {
        if wires.0.len() > 1 {
            min_distance = cmp::min(min_distance, (coord.0.abs() + coord.1.abs()) as u64)
        }
    }

    writeln!(io::stdout(), "Part 1 : {}", min_distance)?;

    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let coordinates = parse_input(input)?;

    let mut min_step = ::std::usize::MAX;
    use std::cmp;

    for (coord, wires) in &coordinates {
        if wires.0.len() > 1 {
            min_step = cmp::min(min_step, wires.1)
        }
    }

    writeln!(io::stdout(), "Part 1 : {}", min_step)?;

    Ok(())
}

fn add_coordinates(c: &mut HashMap<(i64, i64), (Vec<usize>, usize)>, coord: (i64, i64), wire_number: usize, current_step: usize) {
    match c.get_mut(&coord) {
        Some(wires) => {
            if !wires.0.contains(&wire_number) {
                wires.0.push(wire_number);
                wires.1 += current_step;
            }
        }
        None => {
            c.insert(coord, (vec![wire_number], current_step));
        }
    }
}

fn parse_input(input: &str) -> Result<HashMap<(i64, i64), (Vec<usize>, usize)>> {
    lazy_static! {
        static ref DAY_03_REGEX: Regex = Regex::new("^(?P<direction>[RLUD])(?P<distance>[0-9]+)$").expect("Invalid DAY_03_REGEX!");
    }

    // Keep in memory all coordinates where wires appear and their minimum step_count sum
    let mut coordinates: HashMap<(i64, i64), (Vec<usize>, usize)> = HashMap::new();

    for (wire_number, line) in input.lines().enumerate() {
        let mut current_x = 0;
        let mut current_y = 0;
        let mut current_step = 0;

        for path in line.split(',') {
            match DAY_03_REGEX.captures(&path) {
                None => {
                    err(&format!("Invalid input path found : {}", &path))?;
                }
                Some(cap) => {
                    let distance: i64 = cap["distance"].parse()?;
                    match &cap["direction"] {
                        "R" => {
                            for _ in 0..distance {
                                current_x += 1;
                                current_step += 1;
                                add_coordinates(&mut coordinates, (current_x, current_y), wire_number, current_step);
                            }
                        },
                        "L" => {
                            for _ in 0..distance {
                                current_x -= 1;
                                current_step += 1;
                                add_coordinates(&mut coordinates, (current_x, current_y), wire_number, current_step);
                            }
                        },
                        "U" => {
                            for _ in 0..distance {
                                current_y += 1;
                                current_step += 1;
                                add_coordinates(&mut coordinates, (current_x, current_y), wire_number, current_step);
                            }
                        },
                        "D" => {
                            for _ in 0..distance {
                                current_y -= 1;
                                current_step += 1;
                                add_coordinates(&mut coordinates, (current_x, current_y), wire_number, current_step);
                            }
                        },
                        _ => unreachable!("DAY_03_REGEX shouldn't have captured this case : {}", &path),
                    }
                }
            }
        }
    }

    Ok(coordinates)
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(format!("{}", s)))
}