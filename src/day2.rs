use std::ops::Add;

pub fn wrapping_paper(l: u64, w: u64, h: u64) -> u64 {
    let sides = [2 * l * w, 2 * w * h, 2 * h * l];
    let min_side = sides.iter().cloned().min().unwrap_or(0);
    sides.iter().fold(0, Add::add) + min_side / 2
}

pub fn ribbon(l: u64, w: u64, h: u64) -> u64 {
    let mut lengths = [l, w, h];
    lengths.sort();
    2 * lengths[0] + 2 * lengths[1] + l * w * h
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let (mut wp_sum, mut ribbon_sum) = (0, 0);
    for line in file.lines() {
        let line = line.unwrap();
        let dims: Vec<&str> = line.split('x').collect();
        let (l, w, h) = (dims[0].parse::<u64>().unwrap(),
                         dims[1].parse::<u64>().unwrap(),
                         dims[2].parse::<u64>().unwrap());
        wp_sum += wrapping_paper(l, w, h);
        ribbon_sum += ribbon(l, w, h);
    }
    println!("Part 1: {}", wp_sum);
    println!("Part 2: {}", ribbon_sum);
}

#[cfg(test)]
mod tests {
    use super::{wrapping_paper, ribbon};

    #[test]
    fn test_wrapping_paper() {
        assert_eq!(wrapping_paper(0, 0, 0), 0);
        assert_eq!(wrapping_paper(2, 3, 4), 58);
        assert_eq!(wrapping_paper(1, 1, 10), 43);
    }

    #[test]
    fn test_ribbon() {
        assert_eq!(ribbon(0, 0, 0), 0);
        assert_eq!(ribbon(2, 3, 4), 34);
        assert_eq!(ribbon(1, 1, 10), 14);
    }
}
