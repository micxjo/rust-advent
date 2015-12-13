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

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut part1 = 0;
    for line in file.lines() {
        let line = line.unwrap();
        part1 += char_count(&line);
    }
    println!("Part 1: {}", part1);
}


#[cfg(test)]
mod tests {
    use super::char_count;

    #[test]
    fn test_char_count() {
        assert_eq!(char_count("\"\""), 2);
        assert_eq!(char_count("\"abc\""), 2);
        assert_eq!(char_count("\"aaa\\\"aaa\""), 3);
        assert_eq!(char_count("\"\\x27\""), 5);
    }
}
