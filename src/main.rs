extern crate advent;
use advent::*;

use std::env;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    match env::args().nth(1) {
        None => println!("Please provide an input file"),
        Some(path) => {
            if path.ends_with("day1") { day1::process_file(&path); }
            else if path.ends_with("day2") { day2::process_file(&path); }
            else if path.ends_with("day3") { day3::process_file(&path); }
            else if path.ends_with("day4") { day4::process_file(&path); }
            else if path.ends_with("day5") { day5::process_file(&path); }
            else if path.ends_with("day6") { day6::process_file(&path); }
            else if path.ends_with("day7") { day7::process_file(&path); }
            else if path.ends_with("day8") { day8::process_file(&path); }
            else if path.ends_with("day9") { day9::process_file(&path); }
            else if path.ends_with("day10") {
                match env::args().nth(2) {
                    None => println!("Please provide an initial input"),
                    Some(arg) => day10::process_arg(&arg)
                }
            }
            else if path.ends_with("day11") {
                match env::args().nth(2) {
                    None => println!("Please provide an initial input"),
                    Some(arg) => day11::process_arg(&arg)
                }
            }
            else if path.ends_with("day12") { day12::process_file(&path); }
            else { println!("Sorry, I don't know about that day"); }
        }
    }
}
