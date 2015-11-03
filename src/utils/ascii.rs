pub fn from_ascii(data: &String) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(data.len());
    for c in data.chars() {
        bytes.push(c as u8);
    }
    bytes
}

pub fn to_ascii(data: &Vec<u8>) -> String {
    let mut string = String::with_capacity(data.len());
    for byte in data.iter() {
        string.push(*byte as char);
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ascii() {
        assert_eq!(from_ascii(&String::from("An")), vec![65u8, 110u8]);
    }

    #[test]
    fn test_to_ascii() {
        let bytes = vec![65u8, 110u8];
        assert_eq!(to_ascii(&bytes), String::from("An"));
    }
}
