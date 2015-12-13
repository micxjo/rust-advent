extern crate advent;
use advent::{day1, day2};

use std::env;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    match env::args().nth(1) {
        None => println!("Please provide an input file"),
        Some(path) => {
            if path.ends_with("1") { day1::process_file(&path); }
            else if path.ends_with("2") { day2::process_file(&path); }
            else { println!("Sorry, I don't know about that day"); }
        }
    }
}
