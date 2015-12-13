extern crate regex;

use std::cmp;
use std::ops::Add;
use self::regex::Regex;

pub enum Instruction {
    TurnOn((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

use self::Instruction::*;

fn parse_instructions(path: &str) -> Vec<Instruction> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let re = Regex::new(
        r"(turn on |turn off |toggle )(\d+),(\d+) through (\d+),(\d+)").unwrap();

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut instructions = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();
        let x1 = captures.at(2).unwrap().parse::<usize>().unwrap();
        let y1 = captures.at(3).unwrap().parse::<usize>().unwrap();
        let x2 = captures.at(4).unwrap().parse::<usize>().unwrap();
        let y2 = captures.at(5).unwrap().parse::<usize>().unwrap();
        match captures.at(1).unwrap() {
            "turn on " => instructions.push(TurnOn((x1, y1), (x2, y2))),
            "turn off " => instructions.push(TurnOff((x1, y1), (x2, y2))),
            "toggle " => instructions.push(Toggle((x1, y1), (x2, y2))),
            _ => {}
        }
    }

    instructions
}

fn update_grid<T: Copy, F: Fn(T) -> T>(grid: &mut Vec<T>,
                                       start: (usize, usize),
                                       end: (usize, usize),
                                       fun: F) {
    let (x1, y1) = start;
    let (x2, y2) = end;
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            let index = y * 1000 + x;
            grid[index] = fun(grid[index]);
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn run_part1(instructions: &Vec<Instruction>) -> usize {
    let mut grid: Vec<bool> = (0..(1000 * 1000)).map(|_| false).collect();
    for instruction in instructions {
        match *instruction {
            TurnOn(start, end) =>
                update_grid(&mut grid, start, end, |_| true),
            TurnOff(start, end) =>
                update_grid(&mut grid, start, end, |_| false),
            Toggle(start, end) =>
                update_grid(&mut grid, start, end, |b| !b)
        }
    }

    grid.into_iter().filter(|x| *x).count()
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn run_part2(instructions: &Vec<Instruction>) -> i32 {
    let mut grid: Vec<i32> = (0..(1000 * 1000)).map(|_| 0).collect();
    for instruction in instructions {
        match *instruction {
            TurnOn(start, end) =>
                update_grid(&mut grid, start, end, |x| x + 1),
            TurnOff(start, end) =>
                update_grid(&mut grid, start, end, |x| cmp::max(x - 1, 0)),
            Toggle(start, end) =>
                update_grid(&mut grid, start, end, |x| x + 2)
        }
    }

    grid.iter().fold(0, Add::add)
}

pub fn process_file(path: &str) {
    let instructions = parse_instructions(path);
    println!("Part 1: {}", run_part1(&instructions));
    println!("Part 2: {}", run_part2(&instructions));
}
