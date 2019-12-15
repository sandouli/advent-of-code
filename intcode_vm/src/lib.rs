use std::error::Error;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

pub struct IntCodeVm {
    pub state: StateVm,
    pub ram: Vec<isize>,
    pub current_position: usize,
    pub relative_position: isize,
}

impl IntCodeVm {
    pub fn new(input: &str) -> Result<Self> {
        let mut steps: Vec<isize> = vec![];
        for step in input.trim().split(',') {
            steps.push(step.parse()?);
        }

        Ok(Self {
            state: StateVm::Initial,
            ram: steps,
            current_position: 0,
            relative_position: 0,
        })
    }

    pub fn run(&mut self, mut input: Option<isize>) -> Result<Option<isize>> {
        self.state = StateVm::Initial;

        loop {
            if self.ram.len() <= self.current_position {
                err!("Current step outside boundaries of input steps!");
            }
            let (opcode, access_mode_1, access_mode_2, access_mode_3) = self.parse_instruction()?;

            self.current_position += match opcode {
                OpCode::Addition => {
                    let first_param = self.get_parameter(1, access_mode_1)?;
                    let second_param = self.get_parameter(2, access_mode_2)?;

                    self.set_parameter(3, access_mode_3, first_param + second_param)?;
                    4
                }
                OpCode::Multiplication => {
                    let first_param = self.get_parameter(1, access_mode_1)?;
                    let second_param = self.get_parameter(2, access_mode_2)?;

                    self.set_parameter(3, access_mode_3, first_param * second_param)?;
                    4
                }
                OpCode::Input => {
                    match input.take() {
                        Some(input) => {
                            self.set_parameter(1, access_mode_1, input)?;
                        }
                        None => {
                            self.state = StateVm::WaitingInstruction;
                            return Ok(None);
                        }
                    }
                    2
                }
                OpCode::Output => {
                    let first_param = self.get_parameter(1, access_mode_1)?;
                    self.current_position += 2;
                    self.state = StateVm::Output;
                    return Ok(Some(first_param));
                }
                OpCode::JumpIfTrue => {
                    let first_param = self.get_parameter(1, access_mode_1)?;
                    let second_param = self.get_parameter(2, access_mode_2)?;

                    if first_param != 0 {
                        self.current_position = second_param as usize;
                        0
                    } else {
                        3
                    }
                }
                OpCode::JumpIfFalse => {
                    let first_param = self.get_parameter(1, access_mode_1)?;
                    let second_param = self.get_parameter(2, access_mode_2)?;

                    if first_param == 0 {
                        self.current_position = second_param as usize;
                        0
                    } else {
                        3
                    }
                }
                OpCode::LessThan => {
                    let first_param = self.get_parameter(1, access_mode_1)?;
                    let second_param = self.get_parameter(2, access_mode_2)?;

                    self.set_parameter(
                        3,
                        access_mode_3,
                        if first_param < second_param { 1 } else { 0 },
                    )?;
                    4
                }
                OpCode::Equals => {
                    let first_param = self.get_parameter(1, access_mode_1)?;
                    let second_param = self.get_parameter(2, access_mode_2)?;

                    self.set_parameter(
                        3,
                        access_mode_3,
                        if first_param == second_param { 1 } else { 0 },
                    )?;
                    4
                }
                OpCode::AdjustsRelativeBase => {
                    let first_param = self.get_parameter(1, access_mode_1)?;

                    self.relative_position += first_param;
                    2
                }
                OpCode::EndsProgram => {
                    self.state = StateVm::Ended;
                    return Ok(None);
                }
            };
        }
    }

    fn get_parameter(&mut self, position: usize, access_mode: AccessMode) -> Result<isize> {
        self.check_memory((self.current_position + position) as isize)?;

        let param: isize;
        let current_val = self.ram[self.current_position + position];

        match access_mode {
            AccessMode::Position => {
                self.check_memory(current_val)?;
                param = self.ram[current_val as usize];
            }
            AccessMode::Immediate => {
                param = current_val;
            }
            AccessMode::Relative => {
                self.check_memory(current_val + self.relative_position)?;
                param = self.ram[(current_val + self.relative_position) as usize];
            }
        }

        Ok(param)
    }

    fn set_parameter(
        &mut self,
        position: usize,
        access_mode: AccessMode,
        value: isize,
    ) -> Result<()> {
        self.check_memory((self.current_position + position) as isize)?;

        let current_val = self.ram[self.current_position + position];

        match access_mode {
            AccessMode::Position => {
                self.check_memory(current_val)?;
                self.ram[current_val as usize] = value;
            }
            AccessMode::Immediate => {
                err!("Setting parameter in immediate mode is not allowed!");
            }
            AccessMode::Relative => {
                self.check_memory(current_val + self.relative_position)?;
                self.ram[(current_val + self.relative_position) as usize] = value;
            }
        }

        Ok(())
    }

    fn check_memory(&mut self, position: isize) -> Result<()> {
        if position < 0 {
            err!("Positional parameter should not be less than zero!");
        }
        if self.ram.len() <= position as usize {
            for _ in 0..=(position as usize - self.ram.len()) {
                self.ram.push(0);
            }
        }

        Ok(())
    }

    fn parse_instruction(&self) -> Result<(OpCode, AccessMode, AccessMode, AccessMode)> {
        let instruction = format!("{:05}", self.ram[self.current_position]);
        let vec_code = instruction.chars().collect::<Vec<char>>();
        let opcode = OpCode::from_int(self.ram[self.current_position] % 100)?;
        let mode_1 = AccessMode::from_char(vec_code[2])?;
        let mode_2 = AccessMode::from_char(vec_code[1])?;
        let mode_3 = AccessMode::from_char(vec_code[0])?;

        Ok((opcode, mode_1, mode_2, mode_3))
    }
}

pub enum StateVm {
    Initial,
    WaitingInstruction,
    Output,
    Ended,
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
            _ => err!("Not a valid access mode character : {}", c),
        }
    }
}

enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustsRelativeBase,
    EndsProgram,
}

impl OpCode {
    fn from_int(n: isize) -> Result<OpCode> {
        match n {
            1 => Ok(OpCode::Addition),
            2 => Ok(OpCode::Multiplication),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output),
            5 => Ok(OpCode::JumpIfTrue),
            6 => Ok(OpCode::JumpIfFalse),
            7 => Ok(OpCode::LessThan),
            8 => Ok(OpCode::Equals),
            9 => Ok(OpCode::AdjustsRelativeBase),
            99 => Ok(OpCode::EndsProgram),
            _ => err!("Not a valid opcode : {}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input_to_steps(input: &str) -> Vec<isize> {
        let mut steps = vec![];

        for step in input.trim().split(',') {
            steps.push(step.parse().unwrap());
        }

        steps
    }

    #[test]
    fn test_1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut intcode_vm = IntCodeVm::new(input).unwrap();
        let mut result: Vec<isize> = vec![];

        let expected_result = parse_input_to_steps(input);

        loop {
            match intcode_vm.run(None).unwrap() {
                Some(x) => {
                    result.push(x);
                }
                None => match intcode_vm.state {
                    StateVm::Ended => break,
                    _ => unreachable!("Other states should not be reachable during this test!"),
                },
            }
        }

        assert_eq!(result, expected_result, "Copy of itself");
    }

    #[test]
    fn test_2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut intcode_vm = IntCodeVm::new(input).unwrap();
        let mut result: Vec<isize> = vec![];

        loop {
            match intcode_vm.run(None).unwrap() {
                Some(x) => {
                    result.push(x);
                }
                None => match intcode_vm.state {
                    StateVm::Ended => break,
                    _ => unreachable!("Other states should not be reachable during this test!"),
                },
            }
        }

        assert_eq!(result, vec![34915192 * 34915192], "16 digit number");
    }

    #[test]
    fn test_3() {
        let input = "104,1125899906842624,99";
        let mut intcode_vm = IntCodeVm::new(input).unwrap();
        let mut result: Vec<isize> = vec![];

        loop {
            match intcode_vm.run(None).unwrap() {
                Some(x) => {
                    result.push(x);
                }
                None => match intcode_vm.state {
                    StateVm::Ended => break,
                    _ => unreachable!("Other states should not be reachable during this test!"),
                },
            }
        }

        assert_eq!(result, vec![1125899906842624], "Large number");
    }
}
