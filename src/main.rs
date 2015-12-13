extern crate advent;
use advent::*;

use std::env;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    match env::args().nth(1) {
        None => println!("Please provide an input file"),
        Some(path) => {
            if path.ends_with("1") { day1::process_file(&path); }
            else if path.ends_with("2") { day2::process_file(&path); }
            else if path.ends_with("3") { day3::process_file(&path); }
            else if path.ends_with("4") { day4::process_file(&path); }
            else if path.ends_with("5") { day5::process_file(&path); }
            else if path.ends_with("6") { day6::process_file(&path); }
            else if path.ends_with("7") { day7::process_file(&path); }
            else if path.ends_with("8") { day8::process_file(&path); }
            else { println!("Sorry, I don't know about that day"); }
        }
    }
}
