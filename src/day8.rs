pub fn char_count(string: &str) -> usize {
    let bytes = string.as_bytes();
    let mut sum = bytes.len();
    let mut i = 0;
    while i < bytes.len() - 1 {
        sum -= 1;
        match (bytes[i] as char, bytes[i + 1] as char) {
            ('\\', 'x') => i += 4,
            ('\\', '\\') => i += 2,
            ('\\', '\"') => i += 2,
            (_, _) => i += 1,
        }
    }

    sum + 1
}

pub fn encoded_count(string: &str) -> usize {
    let bytes = string.as_bytes();
    let mut sum = 0;
    let mut i = 0;
    while i < bytes.len() - 1 {
        match (bytes[i] as char, bytes[i + 1] as char) {
            ('\\', '\"') => {
                sum += 2;
                i += 2;
            }
            ('\\', '\\') => {
                sum += 2;
                i += 2;
            }
            ('\\', 'x') => {
                sum += 1;
                i += 4;
            }
            _ => {
                i += 1;
            }
        }
    }

    sum + 4
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let (mut part1, mut part2) = (0, 0);
    for line in file.lines() {
        let line = line.unwrap();
        part1 += char_count(&line);
        part2 += encoded_count(&line);
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}


#[cfg(test)]
mod tests {
    use super::{char_count, encoded_count};

    #[test]
    fn test_char_count() {
        assert_eq!(char_count("\"\""), 2);
        assert_eq!(char_count("\"abc\""), 2);
        assert_eq!(char_count("\"aaa\\\"aaa\""), 3);
        assert_eq!(char_count("\"\\x27\""), 5);
    }

    #[test]
    fn test_encoded_count() {
        assert_eq!(encoded_count("\"\""), 4);
        assert_eq!(encoded_count("\"abc\""), 4);
        assert_eq!(encoded_count("\"aaa\\\"aaa\""), 6);
        assert_eq!(encoded_count("\"\\x27\""), 5);
    }
}
