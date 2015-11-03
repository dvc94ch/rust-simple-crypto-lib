pub fn from_hex(hex: &String) -> Vec<u8> {
    if hex.len() % 2 == 1 {
        panic!("invalid hex string length");
    }
    let mut bytes: Vec<u8> = Vec::with_capacity(hex.len() / 2);
    let mut prev: u8 = 0;
    for (i, c) in hex.chars().enumerate() {
        let digit = c.to_digit(16).unwrap() as u8;
        if i % 2 == 0 {
            prev = digit * 16;
        } else {
            bytes.push(prev + digit);
        }
    }
    bytes
}

pub fn to_hex(bytes: &Vec<u8>) -> String {
    let mut string = String::with_capacity(bytes.len() * 2);
    for byte in bytes.iter() {
        string = string + &format!("{:02x}", byte);
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        let bytes = vec![0x4du8, 0x61u8, 0x6eu8];
        assert_eq!(from_hex(&String::from("4d616e")), bytes);
    }

    #[test]
    fn test_to_hex() {
        let bytes = vec![0x4du8, 0x61u8, 0x6eu8];
        assert_eq!(to_hex(&bytes), String::from("4d616e"));
    }
}
