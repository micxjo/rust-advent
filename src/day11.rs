fn good_password(pass: &str) -> bool {
    let bytes = pass.as_bytes();
    let mut straight = false;
    let mut pairs = 0;
    let mut last_pair = false;
    for i in 0..bytes.len() - 2 {
        let (a, b, c) = (bytes[i], bytes[i + 1], bytes[i + 2]);
        if a == b - 1 && b == c - 1 {
            straight = true;
        }

        if a == 'i' as u8 || a == 'o' as u8 || a == 'l' as u8 {
            return false;
        }

        if a == b && !last_pair {
            pairs += 1;
            last_pair = true;
        } else {
            last_pair = false;
        }
    }

    if bytes[bytes.len() - 2] == bytes[bytes.len() - 1] && !last_pair {
        pairs += 1;
    }

    straight && pairs > 1
}

fn next_char(c: char) -> char {
    if c == 'z' {
        'a'
    } else {
        ((c as u8) + 1) as char
    }
}

fn change_password(pass: &str) -> String {
    let mut next = String::new();
    let mut changing = true;
    for c in pass.chars().rev() {
        if changing {
            let n = next_char(c);
            next.insert(0, n);
            if n != 'a' {
                changing = false;
            }
        } else {
            next.insert(0, c);
        }
    }
    next
}

pub fn next_password(pass: &str) -> String {
    let mut next = change_password(pass);
    while !good_password(&next) {
        next = change_password(&next);
    }
    next
}

pub fn process_arg(arg: &str) {
    let part1 = next_password(arg);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", next_password(&part1));
}

#[cfg(test)]
mod tests {
    use super::next_password;

    #[test]
    fn test_next_password() {
        assert_eq!(next_password("abcdefhg"), "abcdffaa");
        assert_eq!(next_password("ghijklmn"), "ghjaabcc");
    }
}
