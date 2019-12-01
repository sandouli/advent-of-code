use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {

    let mut total_fuel: i64 = 0;

    for line in input.lines() {
        let mass: i64 = line.parse()?;
        // If dividing two integers, floor() function seems to be automatically applied
        total_fuel += (mass / 3) - 2;
    }
    writeln!(io::stdout(), "Part 1 : {}", total_fuel)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut total_fuel: i64 = 0;

    for line in input.lines() {
        let mass: i64 = line.parse()?;
        let mut current_fuel = (mass / 3) - 2;

        while current_fuel > 0 {
            total_fuel += current_fuel;
            current_fuel = (current_fuel / 3) - 2;
        }
    }
    writeln!(io::stdout(), "Part 2 : {}", total_fuel)?;
    Ok(())
}