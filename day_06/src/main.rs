use std::error::Error;
use std::io::{self, Read, Write};

use std::collections::{HashMap, HashSet};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let o: Vec<&str> = line.split(')').collect();
        let orbitee = o.get(0).expect("Orbitee code not found!").to_string();
        let orbiter = o.get(1).expect("Orbiter code not found!").to_string();

        match orbits.get_mut(&orbitee) {
            Some(orbit) => {
                orbit.push(orbiter);
            }
            None => {
                orbits.insert(orbitee, vec![orbiter]);
            }
        }
    }

    let mut result = 0;

    let mut current_orbit_distance = 1;
    let mut orbits_to_check: Vec<String> = vec!["COM".to_string()];
    loop {
        let mut next_orbits_to_check: Vec<String> = vec![];
        while !orbits_to_check.is_empty() {
            let current_orbit = orbits_to_check.pop().unwrap();

            if let Some(o) = orbits.get_mut(&current_orbit) {
                result += o.len() * current_orbit_distance;
                next_orbits_to_check.extend_from_slice(&o);
            }
        }

        current_orbit_distance += 1;
        orbits_to_check = next_orbits_to_check;
        if orbits_to_check.is_empty() {
            break;
        }
    }

    writeln!(io::stdout(), "Part 1 : {}", result)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    let mut current_distance = 0;

    for line in input.lines() {
        let o: Vec<&str> = line.split(')').collect();
        let orbitee = o.get(0).expect("Orbitee code not found!").to_string();
        let orbiter = o.get(1).expect("Orbiter code not found!").to_string();

        match orbits.get_mut(&orbitee) {
            Some(orbit) => {
                orbit.push(orbiter.clone());
            }
            None => {
                orbits.insert(orbitee.clone(), vec![orbiter.clone()]);
            }
        }
        match orbits.get_mut(&orbiter) {
            Some(orbit) => {
                orbit.push(orbitee);
            }
            None => {
                orbits.insert(orbiter, vec![orbitee]);
            }
        }
    }

    let mut orbits_checked: HashSet<String> = HashSet::new();
    orbits_checked.insert("YOU".to_string());

    let mut orbits_to_check: Vec<String> = orbits["YOU"].clone();
    'outer: loop {
        let mut next_orbits_to_check: Vec<String> = vec![];
        while !orbits_to_check.is_empty() {
            let current_orbit = orbits_to_check.pop().unwrap();
            if current_orbit == "SAN" {
                break 'outer;
            }

            for i in &orbits[&current_orbit] {
                if !orbits_checked.contains(i) {
                    next_orbits_to_check.push(i.to_string());
                    orbits_checked.insert(i.to_string());
                }
            }
        }

        current_distance += 1;
        orbits_to_check = next_orbits_to_check;

        if orbits_to_check.is_empty() {
            err("No path found!")?;
            break;
        }
    }

    writeln!(io::stdout(), "Part 2 : {}", current_distance - 1)?;
    Ok(())
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(s.to_string()))
}
