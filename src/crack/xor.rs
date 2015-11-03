use xor;
use crack::analysis;

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
        let key = xor::key_from_u8(k);
        let plain_text = xor::xor_cipher(&key, &cipher_text);
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
    use utils;
    use xor;

    #[test]
    fn test_find_key() {
        let cipher_text_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let cipher_text = utils::from_hex(&String::from(cipher_text_hex));
        let k = super::find_key(&cipher_text);
        let key = xor::key_from_u8(k);
        let plain_text = xor::xor_cipher(&key, &cipher_text);
        let plain_text_ascii = utils::to_ascii(&plain_text);
        assert_eq!(k as char, 'X');
        assert_eq!(plain_text_ascii, String::from("Cooking MC's like a pound of bacon"));
    }

    #[test]
    fn test_find_keysize() {
        let buffer = "CzY3JyorLmNiLC5paSojaToqPGMkIC1iPWM0PComImMkJydlJyooKy8gQwplLixlKjEkMzplPisgJ2MMaSsgKDFlKGMmMC4nKC8A";
        let bytes = utils::from_base64(&String::from(buffer));
        assert_eq!(super::find_keysize(&bytes), 3);
    }
}
