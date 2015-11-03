pub fn key_from_u8(k: u8) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::with_capacity(1);
    key.push(k);
    key
}
pub fn xor_cipher(key: &Vec<u8>, plain_text: &Vec<u8>) -> Vec<u8> {
    let mut cipher_text: Vec<u8> = Vec::with_capacity(plain_text.len());
    for (i, byte) in plain_text.iter().enumerate() {
        cipher_text.push(byte ^ key[i % key.len()]);
    }
    cipher_text
}

pub fn xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    if bytes1.len() != bytes2.len() {
        panic!("must be same length");
    }

    let mut res: Vec<u8> = Vec::with_capacity(bytes1.len());
    for (b1, b2) in bytes1.iter().zip(bytes2.iter()) {
        res.push(b1 ^ b2);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sb_xor_cipher() {
        let plain_text = vec![0x1b, 0x37, 0x78];
        let key = key_from_u8(0x05);
        assert_eq!(xor_cipher(&key, &plain_text), vec![0x1e, 0x32, 0x7d]);
    }

    #[test]
    fn test_mb_xor_cipher() {
        let plain_text = vec![0x1b, 0x37, 0x78];
        let key = vec![0x05, 0x04, 0x03];
        assert_eq!(xor_cipher(&key, &plain_text), vec![0x1e, 0x33, 0x7b]);
    }

    #[test]
    fn test_xor() {
        let bytes1 = vec![0x1b, 0x37, 0x78];
        let bytes2 = vec![0x05, 0x04, 0x03];
        assert_eq!(xor(&bytes1[..], &bytes2[..]), vec![0x1e, 0x33, 0x7b]);
    }
}
