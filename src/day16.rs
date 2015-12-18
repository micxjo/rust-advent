use std::collections::HashMap;

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let mut analysis: HashMap<String, u32> = HashMap::new();
    analysis.insert("children:".to_owned(), 3);
    analysis.insert("cats:".to_owned(), 7);
    analysis.insert("samoyeds:".to_owned(), 2);
    analysis.insert("pomeranians:".to_owned(), 3);
    analysis.insert("akitas:".to_owned(), 0);
    analysis.insert("vizslas:".to_owned(), 0);
    analysis.insert("goldfish:".to_owned(), 5);
    analysis.insert("trees:".to_owned(), 3);
    analysis.insert("cars:".to_owned(), 2);
    analysis.insert("perfumes:".to_owned(), 1);

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    for line in file.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split(" ").collect();

        let thing = words[2].to_owned();
        let count = words[3].trim_matches(',').parse::<u32>().unwrap();
        match analysis.get(&thing) {
            Some(n) if n == &count => {}
            _ => continue,
        }

        let thing = words[4].to_owned();
        let count = words[5].trim_matches(',').parse::<u32>().unwrap();
        match analysis.get(&thing) {
            Some(n) if n == &count => {}
            _ => continue,
        }

        let thing = words[6].to_owned();
        let count = words[7].trim_matches(',').parse::<u32>().unwrap();
        match analysis.get(&thing) {
            Some(n) if n == &count => {}
            _ => continue,
        }

        println!("Part 1: {}", words[1].trim_matches(':').to_owned());
        break;
    }

}
