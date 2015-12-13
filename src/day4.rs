extern crate libc;
extern crate rustc_serialize;

use self::libc::c_long;
use self::rustc_serialize::hex::*;

#[link(name="crypto")]
extern "C" {
    fn MD5(input: *const u8, size: c_long, output: *mut u8) -> *mut u8;
}

fn md5(src: &str) -> String {
    unsafe {
        let mut digest: Vec<u8> = Vec::with_capacity(16);
        MD5(src.as_ptr(), src.len() as c_long, digest.as_mut_ptr());
        digest.set_len(16);
        digest[..].to_hex()
    }
}

pub fn find_hash(prefix: &str, search: &str) -> u64 {
    for i in 1.. {
        let msg = format!("{}{}", prefix, i);
        let digest = md5(&msg);
        if digest.starts_with(search) {
            return i;
        }
    }
    u64::max_value()
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    println!("Part 1: {}", find_hash(&string, "00000"));
    println!("Part 2: {}", find_hash(&string, "000000"));
}

#[cfg(test)]
mod tests {
    use super::find_hash;

    #[test]
    fn test_find_hash() {
        assert_eq!(find_hash("abcdef", "00000"), 609043);
        assert_eq!(find_hash("pqrstuv", "00000"), 1048970);
    }
}
