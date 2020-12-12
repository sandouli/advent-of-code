#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:      37,525 ns/iter (+/- 1,678)
    test bench::bench_part_1      ... bench:      12,950 ns/iter (+/- 1,050)
    test bench::bench_part_2      ... bench:      12,034 ns/iter (+/- 930)

*/

use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_angle(&self) -> isize {
        match self {
            Direction::North => 90,
            Direction::East => 0,
            Direction::South => 270,
            Direction::West => 180,
        }
    }

    fn from_angle(mut angle: isize) -> Result<Self> {
        angle %= 360;
        if angle < 0 {
            angle += 360;
        }

        match angle {
            90 => Ok(Direction::North),
            0 => Ok(Direction::East),
            270 => Ok(Direction::South),
            180 => Ok(Direction::West),
            _ => err!("Angle could not determine a direction : {}", angle),
        }
    }
}

#[derive(Clone)]
enum Instruction {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Forward(isize),
    Right(isize),
    Left(isize),
}

#[derive(Clone)]
struct Ship {
    current_direction: Direction,
    current_position: (isize, isize),
    current_waypoint: (isize, isize),
    instructions: Vec<Instruction>,
}

impl Ship {
    fn execute_instructions(&mut self) -> Result<()> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::North(number) => self.current_position.1 += number,
                Instruction::East(number) => self.current_position.0 += number,
                Instruction::South(number) => self.current_position.1 -= number,
                Instruction::West(number) => self.current_position.0 -= number,
                Instruction::Forward(number) => match self.current_direction {
                    Direction::North => self.current_position.1 += number,
                    Direction::East => self.current_position.0 += number,
                    Direction::South => self.current_position.1 -= number,
                    Direction::West => self.current_position.0 -= number,
                },
                Instruction::Left(number) => {
                    self.current_direction =
                        Direction::from_angle(self.current_direction.to_angle() + number)?
                }
                Instruction::Right(number) => {
                    self.current_direction =
                        Direction::from_angle(self.current_direction.to_angle() - number)?
                }
            }
        }
        Ok(())
    }

    fn execute_instructions_with_waypoint(&mut self) -> Result<()> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::North(number) => self.current_waypoint.1 += number,
                Instruction::East(number) => self.current_waypoint.0 += number,
                Instruction::South(number) => self.current_waypoint.1 -= number,
                Instruction::West(number) => self.current_waypoint.0 -= number,
                Instruction::Forward(number) => {
                    for _ in 0..*number {
                        self.current_position.0 += self.current_waypoint.0;
                        self.current_position.1 += self.current_waypoint.1;
                    }
                }
                Instruction::Left(number) => {
                    self.current_waypoint = self.get_rotated_waypoint(false, *number)?
                }
                Instruction::Right(number) => {
                    self.current_waypoint = self.get_rotated_waypoint(true, *number)?
                }
            }
        }
        Ok(())
    }

    fn get_rotated_waypoint(&self, rotate_right: bool, mut angle: isize) -> Result<(isize, isize)> {
        if rotate_right {
            angle *= -1;
        }
        angle %= 360;

        match angle {
            0 => Ok(self.current_waypoint),
            -270 | 90 => Ok((-self.current_waypoint.1, self.current_waypoint.0)),
            -180 | 180 => Ok((-self.current_waypoint.0, -self.current_waypoint.1)),
            -90 | 270 => Ok((self.current_waypoint.1, -self.current_waypoint.0)),
            _ => err!("Invalid angle found : {}", angle),
        }
    }
}

impl TryFrom<&str> for Ship {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        let mut instructions: Vec<Instruction> = vec![];

        for line in value.lines() {
            let number = line[1..].parse::<isize>()?;
            match &line[..1] {
                "N" => instructions.push(Instruction::North(number)),
                "E" => instructions.push(Instruction::East(number)),
                "S" => instructions.push(Instruction::South(number)),
                "W" => instructions.push(Instruction::West(number)),
                "F" => instructions.push(Instruction::Forward(number)),
                "R" => instructions.push(Instruction::Right(number)),
                "L" => instructions.push(Instruction::Left(number)),
                other_char => err!("Invalid instruction char : {}", other_char),
            }
        }

        Ok(Ship {
            current_direction: Direction::East,
            current_position: (0, 0),
            current_waypoint: (10, 1),
            instructions,
        })
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ship = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(ship.clone())?)?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(ship)?)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Ship> {
    Ship::try_from(input)
}

fn part_1(mut ship: Ship) -> Result<isize> {
    ship.execute_instructions()?;
    Ok(ship.current_position.0.abs() + ship.current_position.1.abs())
}

fn part_2(mut ship: Ship) -> Result<isize> {
    ship.execute_instructions_with_waypoint()?;
    Ok(ship.current_position.0.abs() + ship.current_position.1.abs())
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
        let ship = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(ship)?, 25);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let ship = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(ship)?, 286);
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
        let processor = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(processor.clone())));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let processor = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(processor.clone())));
        Ok(())
    }
}
