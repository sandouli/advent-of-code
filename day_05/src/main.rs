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

    let system_id = 1;
    let result = execute_intcode(steps, system_id)?;

    writeln!(io::stdout(), "Part 1 : {}", result)?;

    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut steps: Vec<i64> = vec![];

    for step in input.trim().split(',') {
        steps.push(step.parse()?);
    }

    let system_id = 5;
    let result = execute_intcode(steps, system_id)?;

    writeln!(io::stdout(), "Part 2 : {}", result)?;

    Ok(())
}

fn execute_intcode(mut steps: Vec<i64>, system_id: i64) -> Result<i64> {
    // Execute the program until the value "99" is found or an error occurred
    let mut current_step: usize = 0;
    let mut last_diagnostic = 0;

    loop {
        if steps.len() <= current_step {
            err("Current step outside boundaries of input steps!")?;
        }
        let (opcode, immediate_mode_1, immediate_mode_2, _immediate_mode_3) =
            parse_opcode(steps[current_step])?;

        current_step += match opcode {
            1 => {
                let first_param = get_parameter(&steps, current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, current_step, 3, true)? as usize;

                steps[destination_param] = first_param + second_param;
                4
            }
            2 => {
                let first_param = get_parameter(&steps, current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, current_step, 3, true)? as usize;

                steps[destination_param] = first_param * second_param;
                4
            }
            3 => {
                let destination_param = get_parameter(&steps, current_step, 1, true)? as usize;

                steps[destination_param] = system_id;
                2
            }
            4 => {
                let first_param = get_parameter(&steps, current_step, 1, immediate_mode_1)?;

                last_diagnostic = first_param;
                2
            }
            5 => {
                let first_param = get_parameter(&steps, current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, current_step, 2, true)? as usize;

                if first_param != 0 {
                    current_step = second_param;
                    0
                } else {
                    3
                }
            }
            6 => {
                let first_param = get_parameter(&steps, current_step, 1, immediate_mode_1)?;
                let second_param =
                    get_parameter(&steps, current_step, 2, immediate_mode_2)? as usize;

                if first_param == 0 {
                    current_step = second_param;
                    0
                } else {
                    3
                }
            }
            7 => {
                let first_param = get_parameter(&steps, current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, current_step, 3, true)? as usize;

                steps[destination_param] = if first_param < second_param { 1 } else { 0 };
                4
            }
            8 => {
                let first_param = get_parameter(&steps, current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, current_step, 3, true)? as usize;

                steps[destination_param] = if first_param == second_param { 1 } else { 0 };
                4
            }
            99 => {
                return Ok(last_diagnostic);
            }
            _ => {
                err(&format!("Unknown opcode : {:?}", steps[current_step]))?;
                0
            }
        };
    }
}

fn parse_opcode(code: i64) -> Result<(i64, bool, bool, bool)> {
    let icode = format!("{:05}", code);
    let vec_code = icode.chars().collect::<Vec<char>>();
    let opcode = format!("{}{}", vec_code[3], vec_code[4]).parse()?;
    let mode_1 = parse_char_to_bool(vec_code[2])?;
    let mode_2 = parse_char_to_bool(vec_code[1])?;
    let mode_3 = parse_char_to_bool(vec_code[0])?;

    Ok((opcode, mode_1, mode_2, mode_3))
}

fn parse_char_to_bool(c: char) -> Result<bool> {
    match c {
        '0' => Ok(false),
        '1' => Ok(true),
        _ => Err(Box::<dyn Error>::from(format!("Not a valid bool : {}", c))),
    }
}

fn get_parameter(
    steps: &[i64],
    current_step: usize,
    position: usize,
    immediate_mode: bool,
) -> Result<i64> {
    if steps.len() <= current_step + position {
        err("Parameter position outside boundaries of input steps!")?;
    }

    let param = if immediate_mode {
        steps[current_step + position]
    } else {
        if steps.len() <= steps[current_step + position] as usize {
            err("Parameter position outside boundaries of input steps!")?;
        }
        steps[steps[current_step + position] as usize]
    };

    Ok(param)
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(s.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn parse_input_to_steps(input: &str) -> Vec<i64> {
        let mut steps = vec![];

        for step in input.trim().split(',') {
            steps.push(step.parse().unwrap());
        }

        steps
    }

    #[test]
    fn test_1() {
        // If system_id == 8
        //    output 1
        // Else
        //    output 0
        let steps = parse_input_to_steps("3,9,8,9,10,9,4,9,99,-1,8");

        assert_eq!(execute_intcode(steps.clone(), 7).unwrap(), 0, "ID = 7");
        assert_eq!(execute_intcode(steps.clone(), 8).unwrap(), 1, "ID = 8");
        assert_eq!(execute_intcode(steps.clone(), 9).unwrap(), 0, "ID = 9");
    }

    #[test]
    fn test_2() {
        // If system_id < 8
        //    output 1
        // Else
        //    output 0
        let steps = parse_input_to_steps("3,9,7,9,10,9,4,9,99,-1,8");

        assert_eq!(execute_intcode(steps.clone(), 7).unwrap(), 1, "ID = 7");
        assert_eq!(execute_intcode(steps.clone(), 8).unwrap(), 0, "ID = 8");
        assert_eq!(execute_intcode(steps.clone(), 9).unwrap(), 0, "ID = 9");
    }

    #[test]
    fn test_3() {
        // If system_id == 8
        //    output 1
        // Else
        //    output 0
        let steps = parse_input_to_steps("3,3,1108,-1,8,3,4,3,99");

        assert_eq!(execute_intcode(steps.clone(), 7).unwrap(), 0, "ID = 7");
        assert_eq!(execute_intcode(steps.clone(), 8).unwrap(), 1, "ID = 8");
        assert_eq!(execute_intcode(steps.clone(), 9).unwrap(), 0, "ID = 9");
    }

    #[test]
    fn test_4() {
        // If system_id < 8
        //    output 1
        // Else
        //    output 0
        let steps = parse_input_to_steps("3,3,1107,-1,8,3,4,3,99");

        assert_eq!(execute_intcode(steps.clone(), 7).unwrap(), 1, "ID = 7");
        assert_eq!(execute_intcode(steps.clone(), 8).unwrap(), 0, "ID = 8");
        assert_eq!(execute_intcode(steps.clone(), 9).unwrap(), 0, "ID = 9");
    }

    #[test]
    fn test_5() {
        // If system_id == 0
        //    output 0
        // Else
        //    output 1
        let steps = parse_input_to_steps("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");

        assert_eq!(execute_intcode(steps.clone(), 0).unwrap(), 0, "ID = 0");
        assert_eq!(execute_intcode(steps.clone(), 1).unwrap(), 1, "ID = 1");
        assert_eq!(execute_intcode(steps.clone(), 2).unwrap(), 1, "ID = 2");
    }

    #[test]
    fn test_6() {
        // If system_id == 0
        //    output 0
        // Else
        //    output 1
        let steps = parse_input_to_steps("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");

        assert_eq!(execute_intcode(steps.clone(), 0).unwrap(), 0, "ID = 0");
        assert_eq!(execute_intcode(steps.clone(), 1).unwrap(), 1, "ID = 1");
        assert_eq!(execute_intcode(steps.clone(), 2).unwrap(), 1, "ID = 2");
    }

    #[test]
    fn test_7() {
        // If system_id < 8
        //    output 999
        // Else if system_id == 8
        //    output 1000
        // Else
        //    output 1001
        let steps = parse_input_to_steps("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");

        assert_eq!(execute_intcode(steps.clone(), 7).unwrap(), 999, "ID = 7");
        assert_eq!(execute_intcode(steps.clone(), 8).unwrap(), 1000, "ID = 8");
        assert_eq!(execute_intcode(steps.clone(), 9).unwrap(), 1001, "ID = 9");
    }
}
