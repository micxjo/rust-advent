use std::collections::{HashMap, HashSet};
use std::cmp::max;
use day9::permutations;

fn parse_preferences(path: &str) -> HashMap<(String, String), i32> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut map = HashMap::new();
    for line in file.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split(" ").collect();
        let (a, b) = (words[0], words[10]);
        let b = b.trim_matches('.');
        let diff = words[3].parse::<i32>().unwrap();
        let diff = match words[2] {
            "lose" => -diff,
            _ => diff,
        };
        map.insert((a.to_owned(), b.to_owned()), diff);
    }
    map
}

fn pair_happiness(prefs: &HashMap<(String, String), i32>,
                  first: &str,
                  second: &str)
                  -> i32 {
    let mut diff = prefs.get(&(first.to_owned(), second.to_owned()))
                        .cloned()
                        .unwrap_or(0);
    diff += prefs.get(&(second.to_owned(), first.to_owned()))
                 .cloned()
                 .unwrap_or(0);
    diff
}

fn happiness(prefs: &HashMap<(String, String), i32>,
             seats: &Vec<String>)
             -> i32 {
    let mut sum = 0;
    for i in 0..seats.len() - 1 {
        sum += pair_happiness(prefs, &seats[i], &seats[i + 1]);
    }

    sum + pair_happiness(prefs, &seats[0], &seats[seats.len() - 1])
}

fn all_guests(prefs: &HashMap<(String, String), i32>) -> Vec<String> {
    let mut guests = HashSet::new();
    for key in prefs.keys().cloned() {
        let (a, b) = key;
        guests.insert(a);
        guests.insert(b);
    }

    guests.into_iter().collect()
}

pub fn process_file(path: &str) {
    let prefs = parse_preferences(path);
    let guests = all_guests(&prefs);
    let perms = permutations(&guests);
    let mut part1 = i32::min_value();
    for perm in perms {
        part1 = max(part1, happiness(&prefs, &perm));
    }
    println!("Part 1: {}", part1);

    let mut new_guests: Vec<String> = guests.into_iter().collect();
    new_guests.push("Me".to_owned());
    let new_perms = permutations(&new_guests);
    let mut part2 = i32::min_value();
    for perm in new_perms {
        part2 = max(part2, happiness(&prefs, &perm));
    }
    println!("Part 2: {}", part2);
}
