use std::io::{self, Read, Write};
use std::error::Error;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    let mut expense_report: Vec<usize> = vec![];

    for line in input.lines() {
        expense_report.push(line.parse::<usize>()?);
    }

    for i in 0..(expense_report.len() - 1) {
        for j in (i + 1)..expense_report.len() {
            if expense_report[i] + expense_report[j] == 2020 {
                writeln!(io::stdout(), "Part 1 : {}", expense_report[i] * expense_report[j])?;
                return Ok(());
            }
        }
    }

    err!("Part 1 : No combination found!")
}

fn part_2(input: &str) -> Result<()> {
    let mut expense_report: Vec<usize> = vec![];

    for line in input.lines() {
        expense_report.push(line.parse::<usize>()?);
    }

    for i in 0..(expense_report.len() - 2) {
        for j in (i + 1)..(expense_report.len() - 1) {
            for k in (j + 1)..expense_report.len() {
                if expense_report[i] + expense_report[j] + expense_report[k] == 2020 {
                    writeln!(io::stdout(), "Part 2 : {}", expense_report[i] * expense_report[j] * expense_report[k])?;
                    return Ok(());
                }
            }
        }
    }

    err!("Part 2 : No combination found!")
}