use std::collections::HashMap;

pub fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> i32 {
    let mut distance = 0;
    for (b1, b2) in bytes1.iter().zip(bytes2.iter()) {
        let mut b1 = *b1;
        let mut b2 = *b2;
        for _ in 0..8 {
            if b1 & 1 != b2 & 1 {
                distance += 1;
            }
            b1 = b1 >> 1;
            b2 = b2 >> 1;
        }
    }
    distance
}

pub fn byte_freq(data: &Vec<u8>) -> HashMap<u8, u8> {
    let mut byte_freq: HashMap<u8, u8> = HashMap::with_capacity(30);
    let null = 0u8;

    for byte in data.iter() {
        let count = byte_freq.get(byte).unwrap_or(&null) + 1;
        byte_freq.insert(*byte, count);
    }

    byte_freq
}

pub fn english_score(data: &Vec<u8>) -> i32 {
    let byte_count = byte_freq(data);

    let mut score = 0;
    let null = 0u8;
    let index = String::from("ETAOIN SHRDLU");
    let weight = vec![12, 9, 8, 7, 7, 3, 6, 6, 6, 5, 4, 4, 2];

    for (c, weight) in index.chars().zip(weight.iter()) {
        let i_upper = c as u8;
        let i_lower = match c {
            'A'...'Z' => i_upper + 32,
            _ => i_upper,
        };
        let upper_count = byte_count.get(&i_upper).unwrap_or(&null);
        let lower_count = byte_count.get(&i_lower).unwrap_or(&null);
        score += (upper_count + lower_count) as i32 * weight;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii;

    #[test]
    fn test_hamming_distance() {
        let bytes1 = ascii::from_ascii(&String::from("this is a test"));
        let bytes2 = ascii::from_ascii(&String::from("wokka wokka!!!"));
        let dist = hamming_distance(&bytes1[..], &bytes2[..]);
        assert_eq!(dist, 37);
    }
}
