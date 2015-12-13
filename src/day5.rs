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

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut nice_count = 0;
    for line in file.lines() {
        if is_nice(&line.unwrap()) {
            nice_count += 1;
        }
    }
    println!("Part 1: {}", nice_count);
}

#[cfg(test)]
mod tests {
    use super::is_nice;

    #[test]
    fn test_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
