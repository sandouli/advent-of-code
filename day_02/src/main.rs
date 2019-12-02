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

    let mut steps: Vec<usize> = vec![];

    for line in input.lines() {
        for step in line.split(',') {
            steps.push(step.parse()?);
        }
    }

    // First, update steps 1 and 2 with aforementioned values
    steps[1] = 12;
    steps[2] = 2;


    // Next, execute the program until the value "99" is found
    let mut current_step: usize  = 0;

    loop {
        if steps.len() <= current_step {
            eprintln!("Current step outside boundaries of input steps!");
            break;
        }
        match steps[current_step] {
            1 => {
                if steps.len() <= current_step + 3 {
                    eprintln!("Current step + expected parameters outside boundaries of input steps!");
                    break;
                }
                let first_step_position = steps[current_step + 1];
                let second_step_position = steps[current_step + 2];
                let destination_step_position = steps[current_step + 3];
                if steps.len() <= first_step_position {
                    eprintln!("First parameter position outside boundaries of input steps!");
                    break;
                }
                if steps.len() <= second_step_position {
                    eprintln!("Second parameter position outside boundaries of input steps!");
                    break;
                }
                steps[destination_step_position] = steps[first_step_position] + steps[second_step_position];
            }
            2 => {
                if steps.len() <= current_step + 3 {
                    eprintln!("Current step + expected parameters outside boundaries of input steps!");
                    break;
                }
                let first_step_position = steps[current_step + 1];
                let second_step_position = steps[current_step + 2];
                let destination_step_position = steps[current_step + 3];
                if steps.len() <= first_step_position {
                    eprintln!("First parameter position outside boundaries of input steps!");
                    break;
                }
                if steps.len() <= second_step_position {
                    eprintln!("Second parameter position outside boundaries of input steps!");
                    break;
                }
                steps[destination_step_position] = steps[first_step_position] * steps[second_step_position];
            }
            99 => {
                writeln!(io::stdout(), "Part 1 : {}", steps[0])?;
                break;
            }
            _ => {
                eprintln!("Unknown opcode : {:?}", steps[current_step]);
                break;
            }
        }

        current_step += 4;
    }

    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut steps: Vec<usize> = vec![];

    for line in input.lines() {
        for step in line.split(',') {
            steps.push(step.parse()?);
        }
    }

    let original_steps = steps.clone(); // Keep original state in memory to test other values of `noun` and `verb`

    let expected_result = 19690720;
    let mut noun = 0;
    let mut verb = 0;

    'outer: loop {
        // First, update steps 1 and 2 with aforementioned values
        steps[1] = noun;
        steps[2] = verb;


        let mut program_finished_without_error = false;

        // Next, execute the program until the value "99" is found
        let mut current_step: usize = 0;

        'inner: loop {
            if steps.len() <= current_step {
                eprintln!("Current step outside boundaries of input steps!");
                break 'inner;
            }
            match steps[current_step] {
                1 => {
                    if steps.len() <= current_step + 3 {
                        eprintln!("Current step + expected parameters outside boundaries of input steps!");
                        break 'inner;
                    }
                    let first_step_position = steps[current_step + 1];
                    let second_step_position = steps[current_step + 2];
                    let destination_step_position = steps[current_step + 3];
                    if steps.len() <= first_step_position {
                        eprintln!("First parameter position outside boundaries of input steps!");
                        break 'inner;
                    }
                    if steps.len() <= second_step_position {
                        eprintln!("Second parameter position outside boundaries of input steps!");
                        break 'inner;
                    }
                    steps[destination_step_position] = steps[first_step_position] + steps[second_step_position];
                }
                2 => {
                    if steps.len() <= current_step + 3 {
                        eprintln!("Current step + expected parameters outside boundaries of input steps!");
                        break 'inner;
                    }
                    let first_step_position = steps[current_step + 1];
                    let second_step_position = steps[current_step + 2];
                    let destination_step_position = steps[current_step + 3];
                    if steps.len() <= first_step_position {
                        eprintln!("First parameter position outside boundaries of input steps!");
                        break 'inner;
                    }
                    if steps.len() <= second_step_position {
                        eprintln!("Second parameter position outside boundaries of input steps!");
                        break 'inner;
                    }
                    steps[destination_step_position] = steps[first_step_position] * steps[second_step_position];
                }
                99 => {
                    program_finished_without_error = true;
                    break 'inner;
                }
                _ => {
                    eprintln!("Unknown opcode : {:?}", steps[current_step]);
                    break 'inner;
                }
            }

            current_step += 4;
        }

        if program_finished_without_error && steps[0] == expected_result {
            writeln!(io::stdout(), "Part 2 : {}", 100 * noun + verb)?;
            break 'outer;
        } else {
            steps = original_steps.clone();
            verb += 1;
            if verb > 99 {
                verb = 0;
                noun += 1;
                if noun > 99 {
                    eprintln!("Noun > 99 !");
                    break 'outer;
                }
            }
        }

    }

    Ok(())
}