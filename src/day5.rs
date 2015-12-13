use std::collections::HashMap;

fn is_vowel(c: char) -> bool {
    match c {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    }
}

fn has_good_pair(string: &str) -> bool {
    let chars = string.as_bytes();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn has_bad_pair(string: &str) -> bool {
    string.contains("ab") ||
        string.contains("cd") ||
        string.contains("pq") ||
        string.contains("xy")
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn is_nice(string: &str) -> bool {
    string.chars().filter(|c| is_vowel(*c)).count() >= 3 &&
        has_good_pair(string) &&
        !has_bad_pair(string)
}

fn has_good_trigram(string: &str) -> bool {
    let chars = string.as_bytes();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

fn bigrams(string: &str) -> Vec<(char, char)> {
    string.chars().zip(string.chars().skip(1)).collect()
}

fn has_two_good_pairs(string: &str) -> bool {
    let bs = bigrams(string);
    let mut seen = HashMap::new();
    for i in 0..bs.len() {
        let loc = seen.entry(bs[i]).or_insert(i);
        if *loc + 2 <= i {
            return true;
        }
    }
    false
}

pub fn is_nice_part2(string: &str) -> bool {
    has_good_trigram(string) && has_two_good_pairs(string)
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let (mut part1, mut part2) = (0, 0);
    for line in file.lines() {
        let line = &line.unwrap();
        if is_nice(line) {
            part1 += 1;
        }
        if is_nice_part2(line) {
            part2 += 1;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::{is_nice, is_nice_part2};

    #[test]
    fn test_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_part2() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
