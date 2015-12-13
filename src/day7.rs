use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Source {
    Wire(String),
    Value(u32),
}

use self::Source::*;

#[derive(Clone, Debug)]
enum Connection {
    Signal(Source, String),
    Not(Source, String),
    And(Source, Source, String),
    Or(Source, Source, String),
    LShift(Source, Source, String),
    RShift(Source, Source, String),
}

use self::Connection::*;

fn parse_source(string: &str) -> Source {
    match string.parse::<u32>() {
        Ok(n) => Value(n),
        Err(_) => Wire(string.to_owned()),
    }
}

fn parse_circuit(path: &str) -> Vec<Connection> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut circuit = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split(" ").collect();
        let connection = match words.len() {
            3 => Signal(parse_source(words[0]), words[2].to_owned()),
            4 => Not(parse_source(words[1]), words[3].to_owned()),
            5 => {
                match words[1] {
                    "AND" => {
                        And(parse_source(words[0]),
                            parse_source(words[2]),
                            words[4].to_owned())
                    }
                    "OR" => {
                        Or(parse_source(words[0]),
                           parse_source(words[2]),
                           words[4].to_owned())
                    }
                    "LSHIFT" => {
                        LShift(parse_source(words[0]),
                               parse_source(words[2]),
                               words[4].to_owned())
                    }
                    "RSHIFT" => {
                        RShift(parse_source(words[0]),
                               parse_source(words[2]),
                               words[4].to_owned())
                    }
                    _ => Signal(Value(0), "invalid".to_owned()),
                }
            }
            _ => Signal(Value(0), "invalid".to_owned()),
        };

        circuit.push(connection);
    }

    circuit
}

fn get_source(values: &HashMap<String, u32>, source: Source) -> Option<u32> {
    match source {
        Value(n) => Some(n),
        Wire(s) => values.get(&s).cloned(),
    }
}

fn get_sources(values: &HashMap<String, u32>,
               s1: Source,
               s2: Source)
               -> Option<(u32, u32)> {
    match get_source(values, s1) {
        Some(s1) => {
            match get_source(values, s2) {
                Some(s2) => Some((s1, s2)),
                None => None,
            }
        }
        None => None,
    }
}

fn run_circuit(circuit: &Vec<Connection>, init_b: Option<u16>) -> u16 {
    let mut circuit: Vec<Connection> = circuit.iter().cloned().collect();
    let mut values: HashMap<String, u32> = HashMap::new();
    let mut a = None;

    match init_b {
        Some(n) => {
            values.insert("b".to_owned(), n as u32);
        }
        None => {}
    }

    while a == None {
        let connection = circuit.remove(0);
        let mut used = true;

        match connection.clone() {
            Signal(source, dest) => {
                match get_source(&values, source) {
                    Some(n) => {
                        if !values.contains_key(&dest) {
                            values.insert(dest, n);
                        }
                    }
                    None => {
                        used = false;
                    }
                }
            }
            Not(source, dest) => {
                match get_source(&values, source) {
                    Some(n) => {
                        values.insert(dest, !n);
                    }
                    None => {
                        used = false;
                    }
                }
            }
            And(s1, s2, dest) => {
                match get_sources(&values, s1, s2) {
                    Some((s1, s2)) => {
                        values.insert(dest, (s1 & s2) % 65535);
                    }
                    None => {
                        used = false;
                    }
                }
            }
            Or(s1, s2, dest) => {
                match get_sources(&values, s1, s2) {
                    Some((s1, s2)) => {
                        values.insert(dest, (s1 | s2) % 65535);
                    }
                    None => {
                        used = false;
                    }
                }
            }
            LShift(s1, s2, dest) => {
                match get_sources(&values, s1, s2) {
                    Some((s1, s2)) => {
                        values.insert(dest, (s1 << s2) % 65535);
                    }
                    None => {
                        used = false;
                    }
                }
            }
            RShift(s1, s2, dest) => {
                match get_sources(&values, s1, s2) {
                    Some((s1, s2)) => {
                        values.insert(dest, (s1 >> s2) % 65535);
                    }
                    None => {
                        used = false;
                    }
                }
            }
        }

        if !used {
            circuit.push(connection);
        }

        a = values.get("a").cloned();
    }

    a.unwrap() as u16
}

pub fn process_file(path: &str) {
    let circuit = parse_circuit(path);
    let part1 = run_circuit(&circuit, None);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", run_circuit(&circuit, Some(part1)));
}
