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
    let mut permutations: Vec<Vec<usize>> = vec![];
    let number_of_amplifiers = 5;
    let mut codes = vec![];
    let mut max_result = ::std::i64::MIN;
    let mut steps: Vec<i64> = vec![];

    for i in 0..number_of_amplifiers {
        codes.push(i);
    }
    generate_codes_permutations(&mut codes, number_of_amplifiers, &mut permutations);

    for step in input.trim().split(',') {
        steps.push(step.parse()?);
    }

    for current_permutation in &permutations {
        let mut current_result = 0;
        for i in current_permutation {
            current_result = execute_intcode(&mut steps.clone(), vec![*i as i64, current_result], false, &mut 0)?;
        }
        max_result = std::cmp::max(current_result, max_result);
    }

    writeln!(io::stdout(), "Part 1 : {}", max_result)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut permutations: Vec<Vec<usize>> = vec![];
    let number_of_amplifiers = 5;
    let mut codes = vec![];
    let mut max_result = ::std::i64::MIN;
    let mut steps: Vec<i64> = vec![];

    for i in 5..5+number_of_amplifiers {
        codes.push(i);
    }
    generate_codes_permutations(&mut codes, number_of_amplifiers, &mut permutations);

    for step in input.trim().split(',') {
        steps.push(step.parse()?);
    }

    for current_permutation in &permutations {
        let mut current_result = 0;
        let mut programs: Vec<Vec<i64>> = vec![steps.clone(); number_of_amplifiers];
        let mut current_steps: Vec<usize> = vec![0; number_of_amplifiers];
        'inner: loop {
            for (current_amplifier, i) in current_permutation.iter().enumerate() {
                if current_steps[current_amplifier] > 0 {
                    current_result = execute_intcode(&mut programs[current_amplifier], vec![current_result], true, &mut current_steps[current_amplifier])?;
                } else {
                    current_result = execute_intcode(&mut programs[current_amplifier], vec![*i as i64, current_result], true, &mut current_steps[current_amplifier])?;
                }
            }
            if programs[number_of_amplifiers - 1][current_steps[number_of_amplifiers - 1]] == 99 {
                break 'inner;
            }
        }
        max_result = std::cmp::max(current_result, max_result);
    }

    writeln!(io::stdout(), "Part 2 : {}", max_result)?;
    Ok(())
}

fn generate_codes_permutations(codes: &mut Vec<usize>, n: usize, permutations: &mut Vec<Vec<usize>>) {
    if n == 1 {
        permutations.push(codes.clone());
    } else {
        for i in 0..n-1 {
            generate_codes_permutations(codes, n-1, permutations);
            if n % 2 == 0 {
                codes.swap(n-1, i);
            } else {
                codes.swap(n-1, 0);
            }
        }
        generate_codes_permutations(codes, n-1, permutations);
    }
}

fn execute_intcode(mut steps: &mut Vec<i64>, inputs: Vec<i64>, feedback_loop: bool, current_step: &mut usize) -> Result<i64> {
    // Execute the program until the value "99" is found or an error occurred
    let mut last_diagnostic = 0;
    let mut inputs_processed: usize = 0;

    loop {
        if steps.len() <= *current_step {
            err("Current step outside boundaries of input steps!")?;
        }
        let (opcode, immediate_mode_1, immediate_mode_2, _immediate_mode_3) =
            parse_opcode(steps[*current_step])?;

        *current_step += match opcode {
            1 => {
                let first_param = get_parameter(&steps, *current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, *current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, *current_step, 3, true)? as usize;

                steps[destination_param] = first_param + second_param;
                4
            }
            2 => {
                let first_param = get_parameter(&steps, *current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, *current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, *current_step, 3, true)? as usize;

                steps[destination_param] = first_param * second_param;
                4
            }
            3 => {
                let destination_param = get_parameter(&steps, *current_step, 1, true)? as usize;

                steps[destination_param] = inputs[inputs_processed];
                inputs_processed += 1;
                2
            }
            4 => {
                let first_param = get_parameter(&steps, *current_step, 1, immediate_mode_1)?;

                last_diagnostic = first_param;
                if feedback_loop {
                    *current_step += 2;
                    return Ok(last_diagnostic);
                }
                2
            }
            5 => {
                let first_param = get_parameter(&steps, *current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, *current_step, 2, immediate_mode_2)? as usize;

                if first_param != 0 {
                    *current_step = second_param;
                    0
                } else {
                    3
                }
            }
            6 => {
                let first_param = get_parameter(&steps, *current_step, 1, immediate_mode_1)?;
                let second_param =
                    get_parameter(&steps, *current_step, 2, immediate_mode_2)? as usize;

                if first_param == 0 {
                    *current_step = second_param;
                    0
                } else {
                    3
                }
            }
            7 => {
                let first_param = get_parameter(&steps, *current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, *current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, *current_step, 3, true)? as usize;

                steps[destination_param] = if first_param < second_param { 1 } else { 0 };
                4
            }
            8 => {
                let first_param = get_parameter(&steps, *current_step, 1, immediate_mode_1)?;
                let second_param = get_parameter(&steps, *current_step, 2, immediate_mode_2)?;
                let destination_param = get_parameter(&steps, *current_step, 3, true)? as usize;

                steps[destination_param] = if first_param == second_param { 1 } else { 0 };
                4
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
    use super::*;

    fn parse_input_to_steps(input: &str) -> Vec<i64> {
        let mut steps = vec![];

        for step in input.trim().split(',') {
            steps.push(step.parse().unwrap());
        }

        steps
    }

    fn init_test_values(input: &str, number_of_amplifiers: usize) -> (Vec<i64>, Vec<Vec<usize>>, i64, Vec<usize>) {
        let mut codes: Vec<usize> = vec![];
        for i in 0..number_of_amplifiers {
            codes.push(i);
        }
        let mut permutations: Vec<Vec<usize>> = vec![];
        generate_codes_permutations(&mut codes, number_of_amplifiers, &mut permutations);

        (parse_input_to_steps(input), permutations, ::std::i64::MIN, vec![])
    }

    fn set_permutations_to_feedback_loop(permutations: &mut Vec<Vec<usize>>) {
        for permutation in permutations.iter_mut() {
            for p in permutation.iter_mut() {
                *p += 5;
            }
        }
    }

    #[test]
    fn test_1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let (steps, permutations, mut max_result, mut max_permutation) = init_test_values(input, 5);

        for current_permutation in &permutations {
            let mut current_result = 0;
            for i in current_permutation {
                current_result = execute_intcode(&mut steps.clone(), vec![*i as i64, current_result], false, &mut 0).unwrap();
            }

            if current_result > max_result {
                max_permutation = current_permutation.clone();
                max_result = current_result;
            }
        }

        assert_eq!((max_result, max_permutation), (43210, vec![4, 3, 2, 1, 0]), "Permutation : 4,3,2,1,0");
    }

    #[test]
    fn test_2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let (steps, permutations, mut max_result, mut max_permutation) = init_test_values(input, 5);

        for current_permutation in &permutations {
            let mut current_result = 0;
            for i in current_permutation {
                current_result = execute_intcode(&mut steps.clone(), vec![*i as i64, current_result], false, &mut 0).unwrap();
            }

            if current_result > max_result {
                max_permutation = current_permutation.clone();
                max_result = current_result;
            }
        }

        assert_eq!((max_result, max_permutation), (54321, vec![0, 1, 2, 3, 4]), "Permutation : 0, 1, 2, 3, 4");
    }

    #[test]
    fn test_3() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let (steps, permutations, mut max_result, mut max_permutation) = init_test_values(input, 5);

        for current_permutation in &permutations {
            let mut current_result = 0;
            for i in current_permutation {
                current_result = execute_intcode(&mut steps.clone(), vec![*i as i64, current_result], false, &mut 0).unwrap();
            }

            if current_result > max_result {
                max_permutation = current_permutation.clone();
                max_result = current_result;
            }
        }

        assert_eq!((max_result, max_permutation), (65210, vec![1, 0, 4, 3, 2]), "Permutation : 1, 0, 4, 3, 2");
    }

    #[test]
    fn test_4() {
        // TODO : This does not work, find out why!
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let number_of_amplifiers = 5;
        let (steps, mut permutations, mut max_result, mut max_permutation) = init_test_values(input, number_of_amplifiers);
        set_permutations_to_feedback_loop(&mut permutations);

        for current_permutation in &permutations {
            let mut current_result = 0;
            let mut programs: Vec<Vec<i64>> = vec![steps.clone(); number_of_amplifiers];
            let mut current_steps: Vec<usize> = vec![0; number_of_amplifiers];
            'inner: loop {
                for (current_amplifier, i) in current_permutation.iter().enumerate() {
                    if current_steps[current_amplifier] > 0 {
                        current_result = execute_intcode(&mut programs[current_amplifier], vec![current_result], true, &mut current_steps[current_amplifier]).unwrap();
                    } else {
                        current_result = execute_intcode(&mut programs[current_amplifier], vec![*i as i64, current_result], true, &mut current_steps[current_amplifier]).unwrap();
                    }
                }
                if programs[number_of_amplifiers - 1][current_steps[number_of_amplifiers - 1]] == 99 {
                    break 'inner;
                }
            }
            if current_result > max_result {
                max_permutation = current_permutation.clone();
                max_result = current_result;
            }

        }

        assert_eq!((max_result, max_permutation), (139629729, vec![9, 8, 7, 6, 5]), "Permutation : 9, 8, 7, 6, 5");
    }
}
