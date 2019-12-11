use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type AsteroidPosition = (usize, usize);
type AsteroidsByAngle = HashMap<String, Vec<AsteroidPosition>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    let asteroids = parse_input(input)?;
    let (_best_position, asteroids_by_angle) = get_asteroids_by_angle_for_best_position(&asteroids);

    writeln!(io::stdout(), "Part 1 : {}", asteroids_by_angle.len())?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let asteroids = parse_input(input)?;
    let (best_position, asteroids_by_angle) = get_asteroids_by_angle_for_best_position(&asteroids);

    let asteroids_to_destroy = 200;

    if asteroids.len() < asteroids_to_destroy + 1 {
        err(&format!(
            "Need at least {} asteroids to execute day 10 part 2!",
            asteroids_to_destroy + 1
        ))?;
    }

    let resultant_asteroid = get_destroyed_asteroid_at_position(
        asteroids_to_destroy,
        best_position,
        &asteroids_by_angle,
    )?;

    let result = resultant_asteroid.0 * 100 + resultant_asteroid.1;

    writeln!(io::stdout(), "Part 2 : {}", result)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<(f64, f64)>> {
    let mut asteroids: Vec<(f64, f64)> = vec![];

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    // Empty space, nothing to do here
                }
                '#' => {
                    asteroids.push((j as f64, i as f64));
                }
                _ => {
                    err(&format!(
                        "Invalid character `{}` found in position {},{}!",
                        c, j, i
                    ))?;
                }
            }
        }
    }

    Ok(asteroids)
}

fn get_asteroids_by_angle_for_best_position(
    asteroids: &[(f64, f64)],
) -> (AsteroidPosition, AsteroidsByAngle) {
    let mut best_position: (f64, f64) = (0.0, 0.0);
    let mut best_position_angles: AsteroidsByAngle = HashMap::new();

    for i in 0..asteroids.len() {
        let current_asteroid = asteroids[i];
        let mut current_position_angles: AsteroidsByAngle = HashMap::new();

        for (j, other_asteroid) in asteroids.iter().enumerate() {
            if j != i {
                // No need to compare an asteroid to itself
                //                let other_asteroid = asteroids[j];
                let delta_x = other_asteroid.0 - current_asteroid.0;
                let delta_y = other_asteroid.1 - current_asteroid.1;
                let angle = (delta_y.atan2(delta_x) * 180.0 / std::f64::consts::PI).to_string();

                match current_position_angles.get_mut(&angle) {
                    Some(positions) => {
                        positions.push((other_asteroid.0 as usize, other_asteroid.1 as usize));
                    }
                    None => {
                        current_position_angles.insert(
                            angle,
                            vec![(other_asteroid.0 as usize, other_asteroid.1 as usize)],
                        );
                    }
                }
            }
        }
        if current_position_angles.len() > best_position_angles.len() {
            // Does not account for cases where two positions have same number of possible angles
            best_position = current_asteroid;
            best_position_angles = current_position_angles;
        }
    }

    (
        (best_position.0 as usize, best_position.1 as usize),
        best_position_angles,
    )
}

fn get_destroyed_asteroid_at_position(
    number_to_destroy: usize,
    current_position: AsteroidPosition,
    asteroids_by_angle: &AsteroidsByAngle,
) -> Result<AsteroidPosition> {
    let sorted_angles = get_sorted_angles(&asteroids_by_angle)?;
    let mut destroyed_asteroids_by_angle: HashMap<String, usize> = HashMap::new();
    let mut asteroids_destroyed: usize = 0;

    // First angle to check is -90 since laser's first position is up
    let mut first_angle_to_check = -90.0;

    let mut current_angle_position_to_check = 0;
    while sorted_angles[current_angle_position_to_check] < first_angle_to_check {
        current_angle_position_to_check += 1;
        if current_angle_position_to_check > sorted_angles.len() {
            // On the off chance no asteroids available on angles -90 to +180, restart search starting from -180
            first_angle_to_check = -180.0;
            current_angle_position_to_check = 0;
        }
    }

    loop {
        let current_angle = sorted_angles[current_angle_position_to_check].to_string();
        match destroyed_asteroids_by_angle.get_mut(&current_angle) {
            Some(n) => {
                if asteroids_by_angle[&current_angle].len() < *n {
                    *n += 1;
                    asteroids_destroyed += 1;
                }
            }
            None => {
                destroyed_asteroids_by_angle.insert(current_angle.clone(), 1);
                asteroids_destroyed += 1;
            }
        }

        if asteroids_destroyed == number_to_destroy {
            let resultant_asteroid = get_nth_nearest_asteroid(
                current_position,
                destroyed_asteroids_by_angle[&current_angle],
                asteroids_by_angle[&current_angle].clone(),
            );
            return Ok(resultant_asteroid);
        }

        // Check next available angle
        current_angle_position_to_check += 1;
        if current_angle_position_to_check >= sorted_angles.len() {
            current_angle_position_to_check = 0;
        }
    }
}

fn get_sorted_angles(asteroids_by_angle: &AsteroidsByAngle) -> Result<Vec<f64>> {
    let mut angles: Vec<f64> = vec![];
    for angle in asteroids_by_angle.keys() {
        angles.push(angle.parse()?);
    }

    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(angles)
}

fn get_nth_nearest_asteroid(
    current_position: AsteroidPosition,
    n: usize,
    mut asteroids: Vec<AsteroidPosition>,
) -> AsteroidPosition {
    asteroids.sort_by(|&a, &b| {
        ((((a.0 as f64 - current_position.0 as f64).powi(2))
            + ((a.1 as f64 - current_position.1 as f64).powi(2)))
        .sqrt())
        .partial_cmp(
            &((((b.0 as f64 - current_position.0 as f64).powi(2))
                + ((b.1 as f64 - current_position.1 as f64).powi(2)))
            .sqrt()),
        )
        .unwrap()
    });

    asteroids[n - 1]
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = r".#..#
.....
#####
....#
...##
";

        let asteroids = parse_input(input).unwrap();
        let (best_position, asteroids_by_angle) =
            get_asteroids_by_angle_for_best_position(&asteroids);
        assert_eq!((best_position, asteroids_by_angle.len()), ((3, 4), 8));
    }

    #[test]
    fn test_2() {
        let input = r"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";

        let asteroids = parse_input(input).unwrap();
        let (best_position, asteroids_by_angle) =
            get_asteroids_by_angle_for_best_position(&asteroids);
        assert_eq!((best_position, asteroids_by_angle.len()), ((5, 8), 33));
    }

    #[test]
    fn test_3() {
        let input = r"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";

        let asteroids = parse_input(input).unwrap();
        let (best_position, asteroids_by_angle) =
            get_asteroids_by_angle_for_best_position(&asteroids);
        assert_eq!((best_position, asteroids_by_angle.len()), ((1, 2), 35));
    }

    #[test]
    fn test_4() {
        let input = r".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";

        let asteroids = parse_input(input).unwrap();
        let (best_position, asteroids_by_angle) =
            get_asteroids_by_angle_for_best_position(&asteroids);
        assert_eq!((best_position, asteroids_by_angle.len()), ((6, 3), 41));
    }

    #[test]
    fn test_5() {
        let input = r".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";

        let asteroids = parse_input(input).unwrap();
        let (best_position, asteroids_by_angle) =
            get_asteroids_by_angle_for_best_position(&asteroids);
        assert_eq!((best_position, asteroids_by_angle.len()), ((11, 13), 210));
    }
}
