#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    let mut coordinates: Vec<(isize, isize, isize)> = parse_input(input)?;
    let mut velocities: Vec<(isize, isize, isize)> =
        coordinates.iter().map(|_| (0, 0, 0)).collect();

    // Simulate 1000 steps
    simulate_moon_motions(&mut coordinates, &mut velocities, 1000);

    let total_energy = get_total_energy(&coordinates, &velocities);

    writeln!(io::stdout(), "Part 1 : {}", total_energy)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let coordinates: Vec<(isize, isize, isize)> = parse_input(input)?;
    let velocities: Vec<(isize, isize, isize)> = coordinates.iter().map(|_| (0, 0, 0)).collect();

    let result = get_first_repeated_state_step(coordinates, velocities);

    writeln!(io::stdout(), "Part 2: {}", result)?;
    Ok(())
}

fn simulate_moon_motions(
    coordinates: &mut Vec<(isize, isize, isize)>,
    velocities: &mut Vec<(isize, isize, isize)>,
    steps_to_simulate: usize,
) {
    for _ in 0..steps_to_simulate {
        // Apply gravity to each pair of moon and update the velocity
        for i in 0..(coordinates.len() - 1) {
            for j in i..coordinates.len() {
                // Update x velocities
                if coordinates[i].0 < coordinates[j].0 {
                    velocities[i].0 += 1;
                    velocities[j].0 -= 1;
                } else if coordinates[i].0 > coordinates[j].0 {
                    velocities[i].0 -= 1;
                    velocities[j].0 += 1;
                }

                // Update y velocities
                if coordinates[i].1 < coordinates[j].1 {
                    velocities[i].1 += 1;
                    velocities[j].1 -= 1;
                } else if coordinates[i].1 > coordinates[j].1 {
                    velocities[i].1 -= 1;
                    velocities[j].1 += 1;
                }

                // Update z velocities
                if coordinates[i].2 < coordinates[j].2 {
                    velocities[i].2 += 1;
                    velocities[j].2 -= 1;
                } else if coordinates[i].2 > coordinates[j].2 {
                    velocities[i].2 -= 1;
                    velocities[j].2 += 1;
                }
            }
        }

        for (c, v) in coordinates.iter_mut().zip(velocities.iter()) {
            c.0 += v.0;
            c.1 += v.1;
            c.2 += v.2;
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(isize, isize, isize)>> {
    use regex::Regex;

    lazy_static! {
        static ref DAY_12_REGEX: Regex =
            Regex::new("^<x=(?P<x>-?[0-9]+), y=(?P<y>-?[0-9]+), z=(?P<z>-?[0-9]+)>$")
                .expect("Invalid DAY_12_REGEX!");
    }

    let mut coordinates: Vec<(isize, isize, isize)> = vec![];

    for line in input.lines() {
        match DAY_12_REGEX.captures(&line) {
            None => {
                err(&format!("Invalid input coordinate found : {}", &line))?;
            }
            Some(cap) => {
                coordinates.push((cap["x"].parse()?, cap["y"].parse()?, cap["z"].parse()?))
            }
        }
    }

    Ok(coordinates)
}

fn get_total_energy(
    coordinates: &[(isize, isize, isize)],
    velocities: &[(isize, isize, isize)],
) -> isize {
    let mut total_energy = 0;

    for (c, v) in coordinates.iter().zip(velocities.iter()) {
        total_energy += (c.0.abs() + c.1.abs() + c.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs());
    }
    total_energy
}

fn get_first_repeated_state_step(
    mut coordinates: Vec<(isize, isize, isize)>,
    mut velocities: Vec<(isize, isize, isize)>,
) -> usize {
    let mut steps_velocities_zero: (usize, usize, usize) = (0, 0, 0);

    let mut current_step: usize = 0;
    loop {
        simulate_moon_motions(&mut coordinates, &mut velocities, 1);
        current_step += 1;

        // Check x velocities
        if velocities[0].0 == 0
            && velocities[1].0 == 0
            && velocities[2].0 == 0
            && velocities[3].0 == 0
            && steps_velocities_zero.0 == 0
        {
            steps_velocities_zero.0 = current_step;
        }
        // Check y velocities
        if velocities[0].1 == 0
            && velocities[1].1 == 0
            && velocities[2].1 == 0
            && velocities[3].1 == 0
            && steps_velocities_zero.1 == 0
        {
            steps_velocities_zero.1 = current_step;
        }
        // Check z velocities
        if velocities[0].2 == 0
            && velocities[1].2 == 0
            && velocities[2].2 == 0
            && velocities[3].2 == 0
            && steps_velocities_zero.2 == 0
        {
            steps_velocities_zero.2 = current_step;
        }

        if steps_velocities_zero.0 != 0
            && steps_velocities_zero.1 != 0
            && steps_velocities_zero.2 != 0
        {
            break;
        }
    }

    2 * get_lowest_common_multiple(
        get_lowest_common_multiple(steps_velocities_zero.0, steps_velocities_zero.1),
        steps_velocities_zero.2,
    )
}

fn get_lowest_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / get_greatest_common_divisor(a, b)
}

fn get_greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = r"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

        let mut coordinates: Vec<(isize, isize, isize)> = parse_input(input).unwrap();
        let mut velocities: Vec<(isize, isize, isize)> =
            coordinates.iter().map(|_| (0, 0, 0)).collect();

        simulate_moon_motions(&mut coordinates, &mut velocities, 0);
        assert_eq!(
            (&coordinates, &velocities),
            (
                &vec![(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)],
                &vec![(0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)]
            ),
            "Step 0"
        );

        simulate_moon_motions(&mut coordinates, &mut velocities, 1);
        assert_eq!(
            (&coordinates, &velocities),
            (
                &vec![(2, -1, 1), (3, -7, -4), (1, -7, 5), (2, 2, 0)],
                &vec![(3, -1, -1), (1, 3, 3), (-3, 1, -3), (-1, -3, 1),]
            ),
            "Step 1"
        );

        simulate_moon_motions(&mut coordinates, &mut velocities, 1);
        assert_eq!(
            (&coordinates, &velocities),
            (
                &vec![(5, -3, -1), (1, -2, 2), (1, -4, -1), (1, -4, 2)],
                &vec![(3, -2, -2), (-2, 5, 6), (0, 3, -6), (-1, -6, 2)]
            ),
            "Step 2"
        );

        simulate_moon_motions(&mut coordinates, &mut velocities, 8);
        assert_eq!(
            get_total_energy(&coordinates, &velocities),
            179,
            "Energy at step 10!"
        )
    }

    #[test]
    fn test_2() {
        let input = r"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

        let coordinates: Vec<(isize, isize, isize)> = parse_input(input).unwrap();
        let velocities: Vec<(isize, isize, isize)> =
            coordinates.iter().map(|_| (0, 0, 0)).collect();

        assert_eq!(
            get_first_repeated_state_step(coordinates, velocities),
            2772,
            "2772 steps to get to previous state!"
        )
    }

    #[test]
    fn test_3() {
        let input = r"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
";

        let coordinates: Vec<(isize, isize, isize)> = parse_input(input).unwrap();
        let velocities: Vec<(isize, isize, isize)> =
            coordinates.iter().map(|_| (0, 0, 0)).collect();

        assert_eq!(
            get_first_repeated_state_step(coordinates, velocities),
            4686774924,
            "4686774924 steps to get to previous state!"
        );
    }
}
