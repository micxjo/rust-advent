pub fn look_and_say(string: &str) -> String {
    // Wishing I had .group_by()....
    let mut current_char = None;
    let mut current_count = 0;
    let mut result = String::new();
    for c in string.chars() {
        match current_char {
            Some(current) if current == c => {
                current_count += 1;
            }
            Some(current) => {
                result.push_str(&current_count.to_string());
                result.push(current);
                current_count = 1;
                current_char = Some(c);
            }
            None => {
                current_count = 1;
                current_char = Some(c);
            }
        }
    }

    match current_char {
        Some(current) => {
            result.push_str(&current_count.to_string());
            result.push(current);
        }
        None => {}
    }

    result
}

pub fn iterated_look_and_say(string: &str, count: u32) -> String {
    let mut string = string.to_string();
    for _ in 0..count {
        string = look_and_say(&string);
    }

    string
}

pub fn process_arg(arg: &str) {
    let part1 = iterated_look_and_say(arg, 40);
    let part2 = iterated_look_and_say(&part1, 10);
    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2.len());
}

#[cfg(test)]
mod tests {
    use super::{look_and_say, iterated_look_and_say};

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say(""), "");
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    #[test]
    fn test_iterated_look_and_say() {
        assert_eq!(iterated_look_and_say("1", 1), "11");
        assert_eq!(iterated_look_and_say("1", 2), "21");
        assert_eq!(iterated_look_and_say("1", 3), "1211");
        assert_eq!(iterated_look_and_say("1", 4), "111221");
        assert_eq!(iterated_look_and_say("1", 5), "312211");
    }
}
