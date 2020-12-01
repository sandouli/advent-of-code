extern crate intcode_vm;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

use intcode_vm::*;

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

    let painted_positions = get_painted_positions(steps, 0)?;

    writeln!(io::stdout(), "Part 1 : {}", painted_positions.len())?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut steps: Vec<i64> = vec![];
    for step in input.trim().split(',') {
        steps.push(step.parse()?);
    }

    let painted_positions = get_painted_positions(steps, 1)?;

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);

    // Update min/max coordinates of painted positions
    for current_position in painted_positions.keys() {
        min_x = std::cmp::min(min_x, current_position.1);
        max_x = std::cmp::max(max_x, current_position.1);
        min_y = std::cmp::min(min_y, current_position.0);
        max_y = std::cmp::max(max_y, current_position.0);
    }

    let width = (min_x.abs() + max_x + 1) as usize;
    let height = (min_y.abs() + max_y + 1) as usize;
    let mut pixels: Vec<u8> = vec![0; width * height];

    for (position, color) in &painted_positions {
        pixels[((position.0 + min_y.abs()) * (max_x + 1) + (position.1 + min_x.abs())) as usize] =
            match color {
                0 => 0,
                1 => 255,
                _ => unreachable!(
                    "Painted positions should not have any value other than 0/1 at this point"
                ),
            }
    }

    use image::png::PNGEncoder;
    use image::ColorType;
    use std::fs::File;

    let image_file_path = format!("{}/part_2.png", env!("CARGO_MANIFEST_DIR"));
    let output = File::create(image_file_path.clone())?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixels, width as u32, height as u32, ColorType::Gray(8))?;

    writeln!(
        io::stdout(),
        "Part 2 : To get result, open following image : \"{}\"",
        image_file_path
    )?;

    Ok(())
}

fn get_painted_positions(
    mut steps: Vec<i64>,
    initial_color: usize,
) -> Result<HashMap<(isize, isize), usize>> {
    let mut positions_painted: HashMap<(isize, isize), usize> = HashMap::new();
    let mut current_position = (0, 0);

    let mut current_intcode_step = 0;
    let mut relative_position = 0;
    let mut current_direction = Direction::Up;

    let mut current_color = initial_color;

    loop {
        let color_to_paint = execute_intcode(
            &mut steps,
            vec![current_color as i64],
            true,
            &mut current_intcode_step,
            &mut relative_position,
        )?;
        if steps[current_intcode_step] == 99 {
            break;
        } else {
            match color_to_paint {
                0 | 1 => match positions_painted.get_mut(&current_position) {
                    Some(n) => {
                        *n = color_to_paint as usize;
                    }
                    None => {
                        positions_painted.insert(current_position, color_to_paint as usize);
                    }
                },
                _ => {
                    err(&format!("Invalid color to paint : {}!", color_to_paint))?;
                }
            }
        }

        let turn_direction = execute_intcode(
            &mut steps,
            vec![current_color as i64],
            true,
            &mut current_intcode_step,
            &mut relative_position,
        )?;
        if steps[current_intcode_step] == 99 {
            break;
        } else {
            current_direction = current_direction.get_next_direction(turn_direction)?;
            match current_direction {
                Direction::Up => current_position.1 -= 1,
                Direction::Down => current_position.1 += 1,
                Direction::Left => current_position.0 -= 1,
                Direction::Right => current_position.0 += 1,
            }
        }

        current_color = *positions_painted.get(&current_position).unwrap_or(&0);
    }

    Ok(positions_painted)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_next_direction(self, c: i64) -> Result<Self> {
        match c {
            0 => match self {
                Direction::Up => Ok(Direction::Left),
                Direction::Left => Ok(Direction::Down),
                Direction::Down => Ok(Direction::Right),
                Direction::Right => Ok(Direction::Up),
            },
            1 => match self {
                Direction::Up => Ok(Direction::Right),
                Direction::Left => Ok(Direction::Up),
                Direction::Down => Ok(Direction::Left),
                Direction::Right => Ok(Direction::Down),
            },
            _ => Err(Box::<dyn Error>::from(format!(
                "Not a valid access mode character : {}",
                c
            ))),
        }
    }
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
            _ => Err(Box::<dyn Error>::from(format!(
                "Not a valid access mode character : {}",
                c
            ))),
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
