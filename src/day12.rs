extern crate serde_json;

use std::ops::Add;
use self::serde_json::value::Value;

pub fn json_sum(value: &Value) -> i64 {
    match *value {
        Value::I64(n) => n,
        Value::U64(n) => n as i64,
        Value::Array(ref vec) => vec.iter().map(json_sum).fold(0, Add::add),
        Value::Object(ref o) => o.values().map(json_sum).fold(0, Add::add),
        _ => 0,
    }
}

pub fn json_sum_str(json: &str) -> i64 {
    match serde_json::from_str(json) {
        Ok(val) => json_sum(&val),
        Err(_) => 0,
    }
}

pub fn json_sum_unred(value: &Value) -> i64 {
    match *value {
        Value::I64(n) => n,
        Value::U64(n) => n as i64,
        Value::Array(ref vec) => {
            vec.iter()
               .map(json_sum_unred)
               .fold(0, Add::add)
        }
        Value::Object(ref o) => {
            if o.values().any(|v| v == &Value::String("red".to_owned())) {
                0
            } else {
                o.values().map(json_sum_unred).fold(0, Add::add)
            }
        }
        _ => 0,
    }
}

pub fn json_sum_unred_str(json: &str) -> i64 {
    match serde_json::from_str(json) {
        Ok(val) => json_sum_unred(&val),
        Err(_) => 0,
    }
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    let json = serde_json::from_str(&string).unwrap();
    println!("Part 1: {}", json_sum(&json));
    println!("Part 2: {}", json_sum_unred(&json));
}

#[cfg(test)]
mod tests {
    use super::{json_sum_str, json_sum_unred_str};

    #[test]
    fn test_json_sum() {
        assert_eq!(json_sum_str("[1,2,3]"), 6);
        assert_eq!(json_sum_str("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(json_sum_str("[[[3]]]"), 3);
        assert_eq!(json_sum_str("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(json_sum_str("{\"a\":[-1,1]}"), 0);
        assert_eq!(json_sum_str("[-1,{\"a\":1}]"), 0);
        assert_eq!(json_sum_str("[]"), 0);
        assert_eq!(json_sum_str("[[]]"), 0);
        assert_eq!(json_sum_str("{}"), 0);
    }

    #[test]
    fn test_json_sum_unred() {
        assert_eq!(json_sum_unred_str("[1,2,3]"), 6);
        assert_eq!(json_sum_unred_str("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
        assert_eq!(json_sum_unred_str("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":\
                                       5}"),
                   0);
        assert_eq!(json_sum_unred_str("[1,\"red\",5]"), 6);
    }
}
