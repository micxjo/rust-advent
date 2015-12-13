fn char_count(string: &str, search: char) -> i64 {
    string.chars().filter(|c| *c == search).count() as i64
}

pub fn santa_floor(string: &str) -> i64 {
    char_count(string, '(') - char_count(string, ')')
}

pub fn first_basement(string: &str) -> Option<usize> {
    for len in 0..string.len() {
        let (init, _) = string.split_at(len + 1);
        let floor = santa_floor(init);
        if floor < 0 {
            return Some(len + 1);
        }
    }
    None
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    println!("Part 1: {}", santa_floor(&string));
    println!("Part 2: {}", first_basement(&string).unwrap_or(0));
}

#[cfg(test)]
mod tests {
    use super::{santa_floor, first_basement};

    #[test]
    fn test_santa_floor() {
        assert_eq!(santa_floor("(())"), 0);
        assert_eq!(santa_floor("()()"), 0);
        assert_eq!(santa_floor("((("), 3);
        assert_eq!(santa_floor("(()(()("), 3);
        assert_eq!(santa_floor("))((((("), 3);
        assert_eq!(santa_floor("())"), -1);
        assert_eq!(santa_floor("))("), -1);
        assert_eq!(santa_floor(")))"), -3);
        assert_eq!(santa_floor(")())())"), -3);
    }

    #[test]
    fn test_first_basement() {
        assert_eq!(first_basement("((())"), None);
        assert_eq!(first_basement(")"), Some(1));
        assert_eq!(first_basement("()())"), Some(5));
    }
}
