use traits::Padder;

pub struct PkcsPadder;

impl PkcsPadder {
    pub fn new() -> PkcsPadder {
        PkcsPadder {}
    }
}

impl Padder for PkcsPadder {
    fn pad(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let padding_size = 16 - bytes.len() % 16;
        let padded_length = bytes.len() + padding_size;
        let mut padded_bytes: Vec<u8> = Vec::with_capacity(padded_length);
        let padding = vec![4u8; padding_size];
        unsafe {
            padded_bytes.set_len(padded_length);
            ::std::slice::bytes::copy_memory(&bytes[..], &mut padded_bytes[..]);
            ::std::slice::bytes::copy_memory(&padding[..], &mut padded_bytes[bytes.len()..padded_length]);
        }
        padded_bytes
    }

    fn unpad(&self, mut bytes: Vec<u8>) -> Vec<u8> {
        loop {
            if bytes.len() == 0 || bytes[bytes.len() - 1] != 4 { break; }
            bytes.pop();
        }
        bytes
    }
}
