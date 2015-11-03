pub mod ascii;
pub mod base64;
pub mod blockiter;
pub mod file;
pub mod hex;

pub use self::ascii::{from_ascii, to_ascii};
pub use self::base64::{from_base64, to_base64};
pub use self::hex::{from_hex, to_hex};

pub use self::file::{file_to_buffer};
pub use self::blockiter::{BlockIter};

use rand;

pub fn random_bytes(size: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(rand::random::<u8>());
    }
    vec
}

pub fn escape(string: String, chars: Vec<char>) -> String {
    let mut escaped_string = String::new();
    for c in string.chars() {
        if chars.contains(&c) { continue; }
        escaped_string.push(c);
    }
    escaped_string
}
