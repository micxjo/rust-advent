use std::collections::HashSet;

fn next_house(house: (i64, i64), direction: char) -> (i64, i64) {
    let (x, y) = house;
    match direction {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        _ => (x, y),
    }
}

fn house_path(path: &str) -> Vec<(i64, i64)> {
    let mut house = (0, 0);
    let mut houses = vec![(0, 0)];
    for c in path.chars() {
        house = next_house(house, c);
        houses.push(house);
    }
    houses
}

pub fn unique_houses(path: &str) -> usize {
    house_path(path).into_iter().collect::<HashSet<_>>().len()
}

fn split_path(path: &str) -> (String, String) {
    let (mut santa, mut robo_santa) = ("".to_owned(), "".to_owned());
    let mut i = -1;
    for direction in path.chars() {
        if i < 0 {
            santa.push(direction);
        } else {
            robo_santa.push(direction);
        }
        i *= -1;
    }
    (santa, robo_santa)
}

pub fn unique_houses_with_robo_santa(path: &str) -> usize {
    let (santa_path, robo_path) = split_path(path);
    let santa_houses = house_path(&santa_path);
    let robo_houses = house_path(&robo_path);
    let mut houses = HashSet::new();
    houses.extend(santa_houses.into_iter());
    houses.extend(robo_houses.into_iter());
    houses.len()
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    println!("Part 1: {}", unique_houses(&string));
    println!("Part 2: {}", unique_houses_with_robo_santa(&string));
}

#[cfg(test)]
mod tests {
    use super::{unique_houses, unique_houses_with_robo_santa};

    #[test]
    fn test_unique_houses() {
        assert_eq!(unique_houses(""), 1);
        assert_eq!(unique_houses(">"), 2);
        assert_eq!(unique_houses("^>v<"), 4);
        assert_eq!(unique_houses("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_unique_houses_with_robo_santa() {
        assert_eq!(unique_houses_with_robo_santa(""), 1);
        assert_eq!(unique_houses_with_robo_santa(">"), 2);
        assert_eq!(unique_houses_with_robo_santa("^v"), 3);
        assert_eq!(unique_houses_with_robo_santa("^>v<"), 3);
        assert_eq!(unique_houses_with_robo_santa("^v^v^v^v^v"), 11);
    }
}
