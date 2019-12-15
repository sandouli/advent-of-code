extern crate intcode_vm;

use intcode_vm::*;
use std::error::Error;
use std::io::{self, Read, Write};

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
    let mut vm = IntCodeVm::new(input)?;
    let mut vm_input = None;
    let mut block_count = 0;
    let mut current_block: Vec<isize> = vec![];

    loop {
        match vm.run(vm_input.take())? {
            Some(x) => {
                current_block.push(x);
                if current_block.len() == 3 {
                    if let Block::Block(_, _) = parse_block(&current_block)? {
                        block_count += 1;
                    }
                    current_block.clear();
                }
            }
            None => match vm.state {
                StateVm::Ended => break,
                _ => unreachable!("Other states should not be reachable during this part!"),
            },
        }
    }

    writeln!(io::stdout(), "Part 1 : {}", block_count)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut vm = IntCodeVm::new(input)?;
    let mut vm_input = None;
    let mut current_block: Vec<isize> = vec![];

    let mut current_ball_x = 0;
    let mut current_paddle_x = 0;
    let mut current_score = 0;

    vm.set_ram(0, 2);

    loop {
        match vm.run(vm_input.take())? {
            Some(x) => {
                current_block.push(x);
                if current_block.len() == 3 {
                    match parse_block(&current_block)? {
                        Block::Ball(x, _) => current_ball_x = x,
                        Block::Paddle(x, _) => current_paddle_x = x,
                        Block::Score(score) => current_score = score,
                        _ => {}
                    }
                    current_block.clear();
                }
            }
            None => match vm.state {
                StateVm::WaitingInstruction => {
                    vm_input = if current_ball_x > current_paddle_x {
                        Some(1)
                    } else if current_ball_x < current_paddle_x {
                        Some(-1)
                    } else {
                        Some(0)
                    };
                }
                StateVm::Ended => break,
                _ => unreachable!("Other states should not be reachable during this part!"),
            },
        }
    }

    writeln!(io::stdout(), "Part 2 : {}", current_score)?;
    Ok(())
}

enum Block {
    Empty(isize, isize),
    Wall(isize, isize),
    Block(isize, isize),
    Paddle(isize, isize),
    Ball(isize, isize),
    Score(isize),
}

fn parse_block(block: &[isize]) -> Result<Block> {
    assert_eq!(
        block.len(),
        3,
        "Block description should have a length of 3!"
    );

    if block[0] == -1 && block[1] == 0 {
        Ok(Block::Score(block[2]))
    } else {
        match block[2] {
            0 => Ok(Block::Empty(block[0], block[1])),
            1 => Ok(Block::Wall(block[0], block[1])),
            2 => Ok(Block::Block(block[0], block[1])),
            3 => Ok(Block::Paddle(block[0], block[1])),
            4 => Ok(Block::Ball(block[0], block[1])),
            _ => err!("Unknown block type : {}", block[2]),
        }
    }
}
