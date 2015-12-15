use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::cmp::{min, max};
use std::u32;

#[derive(Clone, Debug)]
struct Link {
    start: String,
    end: String,
    distance: u32,
}

fn parse_links(path: &str) -> Vec<Link> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut links = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split(" ").collect();
        links.push(Link {
            start: words[0].to_owned(),
            end: words[2].to_owned(),
            distance: words[4].parse::<u32>().unwrap(),
        });
    }

    links
}

fn distance(distances: &HashMap<(String, String), u32>,
            path: &Vec<String>)
            -> u32 {
    let mut sum: u32 = 0;
    for i in 0..path.len() - 1 {
        let start = path[i].to_owned();
        let end = path[i + 1].to_owned();
        let dist = distances.get(&(start, end))
                            .cloned()
                            .unwrap_or(u32::max_value());
        sum += dist;
    }
    sum
}

pub fn permutations<T: Clone + Hash + Eq>(vec: &Vec<T>) -> HashSet<Vec<T>> {
    let mut vec: Vec<_> = vec.iter().cloned().collect();
    let mut set = HashSet::new();
    let n = vec.len();
    heaps_perms(&mut set, &mut vec, n);
    set
}

// Generate permutations using Heap's algorithm
fn heaps_perms<T: Clone + Hash + Eq>(set: &mut HashSet<Vec<T>>,
                                     vec: &mut Vec<T>,
                                     n: usize) {
    if n == 1 {
        set.insert(vec.clone());
    } else {
        for i in 0..n - 1 {
            heaps_perms(set, vec, n - 1);
            if n % 2 == 0 {
                vec.swap(i, n - 1);
            } else {
                vec.swap(0, n - 1);
            }
        }
        heaps_perms(set, vec, n - 1);
    }
}

fn all_cities(links: &Vec<Link>) -> Vec<String> {
    let mut cities = HashSet::new();
    for link in links.iter().cloned() {
        cities.insert(link.start);
        cities.insert(link.end);
    }

    cities.into_iter().collect()
}

fn all_distances(links: &Vec<Link>) -> HashMap<(String, String), u32> {
    let mut map = HashMap::new();
    for link in links.iter().cloned() {
        map.insert((link.start.to_owned(), link.end.to_owned()), link.distance);
        map.insert((link.end, link.start), link.distance);
    }

    map
}

pub fn process_file(path: &str) {
    let links = parse_links(path);
    let distances = all_distances(&links);
    let cities = all_cities(&links);
    let perms = permutations(&cities);
    let (mut min_dist, mut max_dist) = (u32::max_value(), 0);
    for perm in perms {
        let dist = distance(&distances, &perm);
        min_dist = min(min_dist, dist);
        max_dist = max(max_dist, dist);
    }
    println!("Part 1: {}", min_dist);
    println!("Part 2: {}", max_dist);
}
