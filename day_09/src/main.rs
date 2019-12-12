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
    let mut steps: Vec<i64> = vec![];
    for step in input.trim().split(',') {
        steps.push(step.parse()?);
    }

    let result = execute_intcode(&mut steps.clone(), vec![1 as i64], false, &mut 0, &mut 0)?;

    writeln!(io::stdout(), "Part 1 : {}", result)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut steps: Vec<i64> = vec![];
    for step in input.trim().split(',') {
        steps.push(step.parse()?);
    }

    let result = execute_intcode(&mut steps.clone(), vec![2 as i64], false, &mut 0, &mut 0)?;

    writeln!(io::stdout(), "Part 2 : {}", result)?;
    Ok(())
}

enum AccessMode {
    Position,
    Immediate,
    Relative,
}

impl AccessMode {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '0' => Ok(AccessMode::Position),
            '1' => Ok(AccessMode::Immediate),
            '2' => Ok(AccessMode::Relative),
            _ => Err(Box::<dyn Error>::from(format!("Not a valid bool : {}", c))),
        }
    }
}

fn execute_intcode(
    steps: &mut Vec<i64>,
    inputs: Vec<i64>,
    feedback_loop: bool,
    current_step: &mut usize,
    relative_position: &mut i64,
) -> Result<i64> {
    // Execute the program until the value "99" is found or an error occurred
    let mut last_diagnostic = 0;
    let mut inputs_processed: usize = 0;

    loop {
        if steps.len() <= *current_step {
            err("Current step outside boundaries of input steps!")?;
        }
        let (opcode, access_mode_1, access_mode_2, access_mode_3) =
            parse_opcode(steps[*current_step])?;
        *current_step += match opcode {
            1 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;
                let second_param =
                    get_parameter(steps, *current_step, 2, access_mode_2, *relative_position)?;

                set_parameter(
                    steps,
                    *current_step,
                    3,
                    access_mode_3,
                    *relative_position,
                    first_param + second_param,
                )?;
                4
            }
            2 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;
                let second_param =
                    get_parameter(steps, *current_step, 2, access_mode_2, *relative_position)?;

                set_parameter(
                    steps,
                    *current_step,
                    3,
                    access_mode_3,
                    *relative_position,
                    first_param * second_param,
                )?;
                4
            }
            3 => {
                set_parameter(
                    steps,
                    *current_step,
                    1,
                    access_mode_1,
                    *relative_position,
                    inputs[inputs_processed],
                )?;
                inputs_processed += 1;
                2
            }
            4 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;

                last_diagnostic = first_param;

                if feedback_loop {
                    *current_step += 2;
                    return Ok(last_diagnostic);
                }
                2
            }
            5 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;
                let second_param =
                    get_parameter(steps, *current_step, 2, access_mode_2, *relative_position)?
                        as usize;

                if first_param != 0 {
                    *current_step = second_param;
                    0
                } else {
                    3
                }
            }
            6 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;
                let second_param =
                    get_parameter(steps, *current_step, 2, access_mode_2, *relative_position)?
                        as usize;

                if first_param == 0 {
                    *current_step = second_param;
                    0
                } else {
                    3
                }
            }
            7 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;
                let second_param =
                    get_parameter(steps, *current_step, 2, access_mode_2, *relative_position)?;

                set_parameter(
                    steps,
                    *current_step,
                    3,
                    access_mode_3,
                    *relative_position,
                    if first_param < second_param { 1 } else { 0 },
                )?;
                4
            }
            8 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;
                let second_param =
                    get_parameter(steps, *current_step, 2, access_mode_2, *relative_position)?;

                set_parameter(
                    steps,
                    *current_step,
                    3,
                    access_mode_3,
                    *relative_position,
                    if first_param == second_param { 1 } else { 0 },
                )?;
                4
            }
            9 => {
                let first_param =
                    get_parameter(steps, *current_step, 1, access_mode_1, *relative_position)?;

                *relative_position += first_param;
                2
            }
            99 => {
                return Ok(last_diagnostic);
            }
            _ => {
                err(&format!("Unknown opcode : {:?}", steps[*current_step]))?;
                0
            }
        };
    }
}

fn parse_opcode(code: i64) -> Result<(i64, AccessMode, AccessMode, AccessMode)> {
    let icode = format!("{:05}", code);
    let vec_code = icode.chars().collect::<Vec<char>>();
    let opcode = format!("{}{}", vec_code[3], vec_code[4]).parse()?;
    let mode_1 = AccessMode::from_char(vec_code[2])?;
    let mode_2 = AccessMode::from_char(vec_code[1])?;
    let mode_3 = AccessMode::from_char(vec_code[0])?;

    Ok((opcode, mode_1, mode_2, mode_3))
}

fn get_parameter(
    steps: &mut Vec<i64>,
    current_step: usize,
    position: usize,
    access_mode: AccessMode,
    relative_position: i64,
) -> Result<i64> {
    if steps.len() <= current_step + position {
        extend_available_memory(steps, current_step + position - steps.len() + 1);
    }

    let param: i64;
    let current_val = steps[current_step + position];

    match access_mode {
        AccessMode::Position => {
            if steps.len() <= current_val as usize {
                extend_available_memory(steps, current_val as usize - steps.len() + 1);
            }
            if current_val < 0 {
                err("Parameter in position mode should not be less than zero!")?;
            }
            param = steps[current_val as usize];
        }
        AccessMode::Immediate => {
            param = current_val;
        }
        AccessMode::Relative => {
            if steps.len() <= (current_val + relative_position) as usize {
                extend_available_memory(
                    steps,
                    (current_val + relative_position) as usize - steps.len() + 1,
                );
            }
            if current_val + relative_position < 0 {
                err("Parameter in relative mode should not be less than zero!")?;
            }
            param = steps[(current_val + relative_position) as usize];
        }
    }

    Ok(param)
}

fn set_parameter(
    steps: &mut Vec<i64>,
    current_step: usize,
    position: usize,
    access_mode: AccessMode,
    relative_position: i64,
    value: i64,
) -> Result<()> {
    if steps.len() <= current_step + position {
        extend_available_memory(steps, current_step + position - steps.len() + 1);
    }

    let current_val = steps[current_step + position];

    match access_mode {
        AccessMode::Position => {
            if steps.len() <= current_val as usize {
                extend_available_memory(steps, current_val as usize - steps.len() + 1);
            }
            if current_val < 0 {
                err("Parameter in position mode should not be less than zero!")?;
            }
            steps[current_val as usize] = value;
        }
        AccessMode::Immediate => {
            err("Setting parameter in immediate mode is not allowed!")?;
        }
        AccessMode::Relative => {
            if steps.len() <= (current_val + relative_position) as usize {
                extend_available_memory(
                    steps,
                    (current_val + relative_position) as usize - steps.len() + 1,
                );
            }
            if current_val + relative_position < 0 {
                err("Parameter in relative mode should not be less than zero!")?;
            }
            steps[(current_val + relative_position) as usize] = value;
        }
    }

    Ok(())
}

fn extend_available_memory(steps: &mut Vec<i64>, n: usize) {
    for _ in 0..n {
        steps.push(0);
    }
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input_to_steps(input: &str) -> Vec<i64> {
        let mut steps = vec![];

        for step in input.trim().split(',') {
            steps.push(step.parse().unwrap());
        }

        steps
    }

    #[test]
    fn test_1() {
        let mut steps =
            parse_input_to_steps("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");

        let expected_result = steps.clone();

        let mut result: Vec<i64> = vec![];

        let mut current_step = 0;
        let mut relative_position = 0;

        loop {
            let x = execute_intcode(
                &mut steps,
                vec![],
                true,
                &mut current_step,
                &mut relative_position,
            )
            .unwrap();
            if steps[current_step] != 99 {
                result.push(x);
            } else {
                break;
            }
        }

        assert_eq!(result, expected_result, "Copy of itself");
    }

    #[test]
    fn test_2() {
        let mut steps = parse_input_to_steps("1102,34915192,34915192,7,4,7,99,0");

        assert_eq!(
            execute_intcode(&mut steps, vec![], false, &mut 0, &mut 0).unwrap(),
            34915192 * 34915192,
            "16 digit number"
        );
    }

    #[test]
    fn test_3() {
        let mut steps = parse_input_to_steps("104,1125899906842624,99");

        assert_eq!(
            execute_intcode(&mut steps, vec![], false, &mut 0, &mut 0).unwrap(),
            1125899906842624,
            "Large number"
        );
    }
}
