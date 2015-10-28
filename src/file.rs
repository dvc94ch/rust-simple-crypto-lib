use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub fn file_to_buffer(filename: &'static str) -> String {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).unwrap();

    buffer
}
