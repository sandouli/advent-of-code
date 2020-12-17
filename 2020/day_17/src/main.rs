#![cfg_attr(feature = "unstable", feature(test))]

// Launch program : cargo run --release < input/input.txt
// Launch benchmark : cargo +nightly bench --features "unstable"

/*
Benchmark results:

    running 5 tests
    test tests::test_part_1 ... ignored
    test tests::test_part_2 ... ignored
    test bench::bench_parse_input ... bench:       4,944 ns/iter (+/- 469)
    test bench::bench_part_1      ... bench:  67,192,333 ns/iter (+/- 2,347,512)
    test bench::bench_part_2      ... bench: 908,377,293 ns/iter (+/- 156,213,295)

*/

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { return Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

static DIRECTIONS: [(isize, isize, isize, isize); 80] = [
    // (x, y, z, w)
    (-1, -1, -1, -1),
    (0, -1, -1, -1),
    (1, -1, -1, -1),
    (-1, 0, -1, -1),
    (0, 0, -1, -1),
    (1, 0, -1, -1),
    (-1, 1, -1, -1),
    (0, 1, -1, -1),
    (1, 1, -1, -1),
    (-1, -1, 0, -1),
    (0, -1, 0, -1),
    (1, -1, 0, -1),
    (-1, 0, 0, -1),
    (1, 0, 0, -1),
    (-1, 1, 0, -1),
    (0, 1, 0, -1),
    (1, 1, 0, -1),
    (-1, -1, 1, -1),
    (0, -1, 1, -1),
    (1, -1, 1, -1),
    (-1, 0, 1, -1),
    (0, 0, 1, -1),
    (1, 0, 1, -1),
    (-1, 1, 1, -1),
    (0, 1, 1, -1),
    (1, 1, 1, -1),
    (-1, -1, -1, 0),
    (0, -1, -1, 0),
    (1, -1, -1, 0),
    (-1, 0, -1, 0),
    (0, 0, -1, 0),
    (1, 0, -1, 0),
    (-1, 1, -1, 0),
    (0, 1, -1, 0),
    (1, 1, -1, 0),
    (-1, -1, 0, 0),
    (0, -1, 0, 0),
    (1, -1, 0, 0),
    (-1, 0, 0, 0),
    (1, 0, 0, 0),
    (-1, 1, 0, 0),
    (0, 1, 0, 0),
    (1, 1, 0, 0),
    (-1, -1, 1, 0),
    (0, -1, 1, 0),
    (1, -1, 1, 0),
    (-1, 0, 1, 0),
    (0, 0, 1, 0),
    (1, 0, 1, 0),
    (-1, 1, 1, 0),
    (0, 1, 1, 0),
    (1, 1, 1, 0),
    (-1, -1, -1, 1),
    (0, -1, -1, 1),
    (1, -1, -1, 1),
    (-1, 0, -1, 1),
    (0, 0, -1, 1),
    (1, 0, -1, 1),
    (-1, 1, -1, 1),
    (0, 1, -1, 1),
    (1, 1, -1, 1),
    (-1, -1, 0, 1),
    (0, -1, 0, 1),
    (1, -1, 0, 1),
    (-1, 0, 0, 1),
    (1, 0, 0, 1),
    (-1, 1, 0, 1),
    (0, 1, 0, 1),
    (1, 1, 0, 1),
    (-1, -1, 1, 1),
    (0, -1, 1, 1),
    (1, -1, 1, 1),
    (-1, 0, 1, 1),
    (0, 0, 1, 1),
    (1, 0, 1, 1),
    (-1, 1, 1, 1),
    (0, 1, 1, 1),
    (1, 1, 1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
];

#[derive(Clone)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
    active: bool,
}

#[derive(Clone)]
struct Map {
    cubes: HashMap<(isize, isize, isize, isize), bool>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
    min_w: isize,
    max_w: isize,
}

impl Map {
    fn execute_turns(&mut self, fourth_dimensional: bool) {
        for _ in 0..6 {
            let mut new_cubes = self.cubes.clone();
            for x in (self.min_x - 1)..=(self.max_x + 1) {
                for y in (self.min_y - 1)..=(self.max_y + 1) {
                    for z in (self.min_z - 1)..=(self.max_z + 1) {
                        if fourth_dimensional {
                            for w in (self.min_w - 1)..=(self.max_w + 1) {
                                self.update_map(&mut new_cubes, x, y, z, w);
                            }
                        } else {
                            self.update_map(&mut new_cubes, x, y, z, 0);
                        }
                    }
                }
            }
            self.cubes = new_cubes;
        }
    }

    fn update_map(
        &mut self,
        new_cubes: &mut HashMap<(isize, isize, isize, isize), bool>,
        x: isize,
        y: isize,
        z: isize,
        w: isize,
    ) {
        // let current_cube_active = match self.cubes.get(&(x, y, z, w)) {
        //     Some(true) => true,
        //     _ => false,
        // };
        let current_cube_active = matches!(self.cubes.get(&(x, y, z, w)), Some(true));
        let active_neighbors_count = self.count_active_neighbors(x, y, z, w);

        match (current_cube_active, active_neighbors_count) {
            (true, 2) | (true, 3) => {}
            (true, _) => {
                new_cubes.insert((x, y, z, w), false);
            }
            (false, 3) => {
                new_cubes.insert((x, y, z, w), true);
                self.update_min_max_coordinates(x, y, z, w);
            }
            (false, _) => {}
        }
    }

    fn count_active_neighbors(
        &self,
        cube_x: isize,
        cube_y: isize,
        cube_z: isize,
        cube_w: isize,
    ) -> usize {
        DIRECTIONS
            .iter()
            .map(|(direction_x, direction_y, direction_z, direction_w)| {
                match self.cubes.get(&(
                    cube_x + direction_x,
                    cube_y + direction_y,
                    cube_z + direction_z,
                    cube_w + direction_w,
                )) {
                    Some(true) => 1,
                    _ => 0,
                }
            })
            .sum()
    }

    fn update_min_max_coordinates(&mut self, x: isize, y: isize, z: isize, w: isize) {
        self.min_x = std::cmp::min(x, self.min_x);
        self.max_x = std::cmp::max(x, self.max_x);
        self.min_y = std::cmp::min(y, self.min_y);
        self.max_y = std::cmp::max(y, self.max_y);
        self.min_z = std::cmp::min(z, self.min_z);
        self.max_z = std::cmp::max(z, self.max_z);
        self.min_w = std::cmp::min(w, self.min_w);
        self.max_w = std::cmp::max(w, self.max_w);
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(&input)?;

    writeln!(io::stdout(), "Part 1 : {}", part_1(map.clone()))?;
    writeln!(io::stdout(), "Part 2 : {}", part_2(map))?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Map> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut cubes: HashMap<(isize, isize, isize, isize), bool> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        max_y += 1;
        max_x = std::cmp::max(max_x, line.len());
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '.' => {}
                '#' => {
                    cubes.insert((x as isize, y as isize, 0, 0), true);
                }
                _ => err!("Invalid input, could not determine cell state : {}", cell),
            };
        }
    }

    Ok(Map {
        cubes,
        min_x: 0,
        max_x: max_x as isize,
        min_y: 0,
        max_y: max_y as isize,
        min_z: 0,
        max_z: 0,
        min_w: 0,
        max_w: 0,
    })
}

fn part_1(mut map: Map) -> usize {
    map.execute_turns(false);
    map.cubes.values().filter(|&v| *v).count()
}

fn part_2(mut map: Map) -> usize {
    map.execute_turns(true);
    map.cubes.values().filter(|&v| *v).count()
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
        let map = parse_input(&read_test_file()?)?;
        assert_eq!(part_1(map), 112);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let map = parse_input(&read_test_file()?)?;
        assert_eq!(part_2(map), 848);
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
        let processor = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_1(processor.clone())));
        Ok(())
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) -> Result<()> {
        let processor = parse_input(&read_input_file()?)?;
        b.iter(|| test::black_box(part_2(processor.clone())));
        Ok(())
    }
}
