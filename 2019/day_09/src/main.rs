extern crate intcode_vm;

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
    let mut vm = IntCodeVm::new(input)?;
    let mut result = 0;
    let mut vm_input = Some(1);

    loop {
        match vm.run(vm_input.take())? {
            Some(x) => {
                result = x;
            }
            None => match vm.state {
                StateVm::Ended => break,
                _ => unreachable!("Other states should not be reachable during this part!"),
            },
        }
    }

    writeln!(io::stdout(), "Part 1 : {}", result)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut vm = IntCodeVm::new(input)?;
    let mut result = 0;
    let mut vm_input = Some(2);

    loop {
        match vm.run(vm_input.take())? {
            Some(x) => {
                result = x;
            }
            None => match vm.state {
                StateVm::Ended => break,
                _ => unreachable!("Other states should not be reachable during this part!"),
            },
        }
    }

    writeln!(io::stdout(), "Part 2 : {}", result)?;
    Ok(())
}
