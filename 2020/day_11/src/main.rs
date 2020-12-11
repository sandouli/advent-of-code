#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:     101,493 ns/iter (+/- 15,975)
    test bench::bench_part_1      ... bench:  79,907,602 ns/iter (+/- 3,840,854)
    test bench::bench_part_2      ... bench: 134,061,672 ns/iter (+/- 4,306,232)

*/

use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Clone, PartialEq, Eq)]
enum CellState {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

#[derive(Clone)]
struct Seats {
    cells: Vec<Vec<CellState>>,
    line_length: usize,
}

impl Seats {
    fn execute_rounds(&mut self, tolerance: usize, only_see_adjacent_seats: bool) {
        loop {
            let mut new_cells = self.cells.clone();
            let mut placement_has_changed = false;
            for (i, cell_line) in self.cells.iter().enumerate() {
                for (j, cell) in cell_line.iter().enumerate() {
                    let occupied_seats = if only_see_adjacent_seats {
                        self.get_adjacent_occupied_seats(j, i)
                    } else {
                        self.get_visible_occupied_seats(j, i)
                    };
                    if *cell == CellState::EmptySeat && occupied_seats == 0 {
                        new_cells[i][j] = CellState::OccupiedSeat;
                        placement_has_changed = true;
                    } else if *cell == CellState::OccupiedSeat && occupied_seats >= tolerance {
                        new_cells[i][j] = CellState::EmptySeat;
                        placement_has_changed = true;
                    }
                }
            }

            self.cells = new_cells;

            if !placement_has_changed {
                break;
            }
        }
    }

    fn get_adjacent_occupied_seats(&self, x: usize, y: usize) -> usize {
        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut adjacent_occupied_seats = 0;

        for direction in directions {
            let current_x = x as isize + direction.0;
            let current_y = y as isize + direction.1;

            if current_x >= 0
                && self.line_length > current_x as usize
                && current_y >= 0
                && self.cells.len() > current_y as usize
                && self.cells[current_y as usize][current_x as usize] == CellState::OccupiedSeat
            {
                adjacent_occupied_seats += 1;
            }
        }

        adjacent_occupied_seats
    }

    fn get_visible_occupied_seats(&self, x: usize, y: usize) -> usize {
        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut visible_occupied_seats = 0;
        'outer: for direction in directions {
            let mut current_x = x as isize + direction.0;
            let mut current_y = y as isize + direction.1;

            while current_x >= 0
                && self.line_length > current_x as usize
                && current_y >= 0
                && self.cells.len() > current_y as usize
            {
                match self.cells[current_y as usize][current_x as usize] {
                    CellState::OccupiedSeat => {
                        visible_occupied_seats += 1;
                        continue 'outer;
                    }
                    CellState::EmptySeat => continue 'outer,
                    _ => {}
                }

                current_x += direction.0;
                current_y += direction.1;
            }
        }
        visible_occupied_seats
    }
}

impl TryFrom<&str> for Seats {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self> {
        let mut cells = vec![];
        let mut line_length = 0;

        for (i, line) in value.lines().enumerate() {
            if i == 0 {
                line_length = line.len();
            } else if line.len() != line_length {
                err!("Invalid input : every line should have the same length")
            }
            let mut cells_line = vec![];

            for cell in line.chars() {
                match cell {
                    '.' => cells_line.push(CellState::Floor),
                    'L' => cells_line.push(CellState::EmptySeat),
                    '#' => cells_line.push(CellState::OccupiedSeat),
                    other_char => err!("Invalid characted found : {}", other_char),
                }
            }

            cells.push(cells_line);
        }

        Ok(Seats { cells, line_length })
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let seats = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(seats.clone()))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(seats))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Seats> {
    Seats::try_from(input)
}

fn part_1(mut seats: Seats) -> usize {
    seats.execute_rounds(4, true);
    seats
        .cells
        .iter()
        .flatten()
        .filter(|&v| *v == CellState::OccupiedSeat)
        .count()
}

fn part_2(mut seats: Seats) -> usize {
    seats.execute_rounds(5, false);
    seats
        .cells
        .iter()
        .flatten()
        .filter(|&v| *v == CellState::OccupiedSeat)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn read_test_file() -> Result<String> {
        let mut input = String::new();
        File::open("input/test.txt")?.read_to_string(&mut input)?;
        Ok(input)
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let seats = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(seats), 37);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let seats = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(seats), 26);
        Ok(())
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use std::fs::File;
    use test::Bencher;

    fn read_input_file() -> Result<String> {
        let mut input = String::new();
        File::open("input/input.txt")?.read_to_string(&mut input)?;
        Ok(input)
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) -> Result<()> {
        let input = read_input_file()?;
        b.iter(|| test::black_box(parse_input(&input)));
        Ok(())
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) -> Result<()> {
        let seats = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(seats.clone())));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let seats = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(seats.clone())));
        Ok(())
    }
}
