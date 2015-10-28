use analysis;

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

pub fn break_xor_cipher(cipher_text: &Vec<u8>, keysize: usize) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::with_capacity(keysize);
    for ks in 0..keysize {
        let block = cipher_text.iter().enumerate()
            .filter(|&(i, _)| i % keysize == ks)
            .map(|(_, u)| *u)
            .collect::<Vec<u8>>();
        key.push(find_key(&block));
    }
    key
}

fn find_key(cipher_text: &Vec<u8>) -> u8 {
    let mut max_score = 0;
    let mut max_key = 0u8;

    for k in 0..255 {
        let key = key_from_u8(k);
        let plain_text = xor_cipher(&key, &cipher_text);
        let score = analysis::english_score(&plain_text);

        if score > max_score {
            max_score = score;
            max_key = k;
        }
    }

    max_key
}

pub fn find_keysize(bytes: &Vec<u8>) -> usize {
    let mut min_dist = -1.0;
    let mut min_keysize = 0;

    for keysize in 2..40 {
        if keysize * 8 > bytes.len() { break; }
        let dist = analysis::hamming_distance(
            &bytes[0..(keysize * 4)],
            &bytes[(keysize * 4)..(keysize * 8)]
        );
        let norm_dist = dist as f32 / keysize as f32;

        if min_dist < 0.0 || norm_dist < min_dist {
            min_dist = norm_dist;
            min_keysize = keysize;
        }
    }
    min_keysize
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii;
    use base64;
    use hex;

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
    fn test_find_key() {
        let cipher_text_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let cipher_text = hex::from_hex(&String::from(cipher_text_hex));
        let k = super::find_key(&cipher_text);
        let key = key_from_u8(k);
        let plain_text = xor_cipher(&key, &cipher_text);
        let plain_text_ascii = ascii::to_ascii(&plain_text);
        assert_eq!(k as char, 'X');
        assert_eq!(plain_text_ascii, String::from("Cooking MC's like a pound of bacon"));
    }

    #[test]
    fn test_find_keysize() {
        let buffer = "CzY3JyorLmNiLC5paSojaToqPGMkIC1iPWM0PComImMkJydlJyooKy8gQwplLixlKjEkMzplPisgJ2MMaSsgKDFlKGMmMC4nKC8A";
        let bytes = base64::from_base64(&String::from(buffer));
        assert_eq!(super::find_keysize(&bytes), 3);
    }
}
