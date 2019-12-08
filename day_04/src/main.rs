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
    let mut numbers: Vec<u64> = vec![];
    for i in input.trim().split('-') {
        if i.len() != 6 {
            err("Codes should have a length of 6!")?;
        }
        numbers.push(i.parse()?);
    }

    let minimum_code = *numbers.get(0).expect("Minimum code not found!");
    let maximum_code = *numbers.get(1).expect("Maximum code not found!");
    let mut current_code = minimum_code;
    let mut result = 0;

    while current_code <= maximum_code {
        let str_code = current_code.to_string();
        let digits_bytes = str_code.as_bytes();
        let mut current_digit = digits_bytes[0];
        let mut found_adjacent_digits = false;
        let mut digits_always_increase = true;

        for next_digit in digits_bytes.iter().skip(1) {
            // Two conditions :
            //   - Two adjacent digits are the same
            //   - Digits always increase or stay the same

            if *next_digit < current_digit {
                digits_always_increase = false;
                break;
            }
            if *next_digit == current_digit {
                found_adjacent_digits = true;
            }

            current_digit = *next_digit;
        }
        if found_adjacent_digits && digits_always_increase {
            result += 1;
        }

        current_code += 1;
    }

    writeln!(io::stdout(), "Part 1 : {}", result)?;

    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut numbers: Vec<u64> = vec![];
    for i in input.trim().split('-') {
        if i.len() != 6 {
            err("Codes should have a length of 6!")?;
        }
        numbers.push(i.parse()?);
    }

    let minimum_code = *numbers.get(0).expect("Minimum code not found!");
    let maximum_code = *numbers.get(1).expect("Maximum code not found!");
    let mut current_code = minimum_code;
    let mut result = 0;

    while current_code <= maximum_code {
        let str_code = current_code.to_string();
        let digits_bytes = str_code.as_bytes();
        let mut current_digit = digits_bytes[0];
        let mut found_adjacent_digits = false;
        let mut digits_always_increase = true;
        let mut current_repeated_digit = 1;

        for next_digit in digits_bytes.iter().skip(1) {
            // Two conditions :
            //   - At least one group of at most two adjacent digits are the same
            //   - Digits always increase or stay the same

            if *next_digit < current_digit {
                digits_always_increase = false;
                break;
            }
            if *next_digit == current_digit {
                current_repeated_digit += 1;
            } else {
                if current_repeated_digit == 2 {
                    found_adjacent_digits = true;
                }
                current_repeated_digit = 1;
            }

            current_digit = *next_digit;
        }
        if current_repeated_digit == 2 {
            found_adjacent_digits = true;
        }

        if found_adjacent_digits && digits_always_increase {
            result += 1;
        }

        current_code += 1;
    }

    writeln!(io::stdout(), "Part 2 : {}", result)?;

    Ok(())
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(s.to_string()))
}
