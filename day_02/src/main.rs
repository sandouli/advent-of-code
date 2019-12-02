use std::io::{self, Read, Write};
use std::error::Error;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {

    let mut steps: Vec<usize> = vec![];

    for line in input.lines() {
        for step in line.split(',') {
            steps.push(step.parse()?);
        }
    }

    // First, update steps 1 and 2 with aforementioned values
    steps[1] = 12;
    steps[2] = 2;

    let result = execute_intcode(steps)?;

    writeln!(io::stdout(), "Part 1 : {}", result)?;

    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut steps: Vec<usize> = vec![];

    for line in input.lines() {
        for step in line.split(',') {
            steps.push(step.parse()?);
        }
    }

    let expected_result = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            steps[1] = noun;
            steps[2] = verb;

            if let Ok(result) = execute_intcode(steps.clone()) {
                if result == expected_result {
                    writeln!(io::stdout(), "Part 2 : {}", 100 * noun + verb)?;
                    return Ok(());
                }
            }
        }
    }

    Err(Box::<dyn Error>::from(format!("IntCode could not find expected value!")))
}


fn execute_intcode(mut steps: Vec<usize>) -> Result<usize> {
    // Execute the program until the value "99" is found or an error occured
    let mut current_step: usize  = 0;

    loop {

        if steps.len() <= current_step {
            return Err(Box::<dyn Error>::from(format!("Current step outside boundaries of input steps!")));
        }
        match steps[current_step] {
            1 => {
                let (first, second, destination) = get_intcode_parameters(&steps, current_step)?;
                steps[destination] = steps[first] + steps[second];
            }
            2 => {
                let (first, second, destination) = get_intcode_parameters(&steps, current_step)?;
                steps[destination] = steps[first] * steps[second];
            }
            99 => {
                return Ok(steps[0]);
            }
            _ => {
                return Err(Box::<dyn Error>::from(format!("Unknown opcode : {:?}", steps[current_step])));
            }
        }

        current_step += 4;
    }
}

fn get_intcode_parameters(steps: &[usize], current_step: usize) -> Result<(usize, usize, usize)> {
    if steps.len() <= current_step + 3 {
        return Err(Box::<dyn Error>::from(format!("Current step + expected parameters outside boundaries of input steps!")));
    }

    let first_step_position = steps[current_step + 1];
    let second_step_position = steps[current_step + 2];
    let destination_step_position = steps[current_step + 3];

    if steps.len() <= first_step_position {
        return Err(Box::<dyn Error>::from(format!("First parameter position outside boundaries of input steps!")));
    }
    if steps.len() <= second_step_position {
        return Err(Box::<dyn Error>::from(format!("Second parameter position outside boundaries of input steps!")));
    }
    if steps.len() <= destination_step_position {
        return Err(Box::<dyn Error>::from(format!("Destination parameter position outside boundaries of input steps!")));
    }

    Ok((first_step_position, second_step_position, destination_step_position))

}