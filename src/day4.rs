extern crate libc;

use self::libc::c_long;

#[link(name="crypto")]
extern "C" {
    fn MD5(input: *const u8, size: c_long, output: *mut u8) -> *mut u8;
}

fn md5(src: &str) -> Vec<u8> {
    unsafe {
        let mut digest: Vec<u8> = Vec::with_capacity(16);
        MD5(src.as_ptr(), src.len() as c_long, digest.as_mut_ptr());
        digest.set_len(16);
        digest
    }
}

pub fn find_hash(prefix: &str, digits: u8) -> u64 {
    let shift = if digits == 5 { 4 } else { 0 };
    for i in 1.. {
        let msg = format!("{}{}", prefix, i);
        let digest = md5(&msg);
        if digest[0] == 0 && digest[1] == 0 && (digest[2] >> shift) == 0 {
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
    println!("Part 1: {}", find_hash(&string, 5));
    println!("Part 2: {}", find_hash(&string, 6));
}

#[cfg(test)]
mod tests {
    use super::find_hash;

    #[test]
    fn test_find_hash() {
        assert_eq!(find_hash("abcdef", 5), 609043);
        assert_eq!(find_hash("pqrstuv", 5), 1048970);
    }
}
