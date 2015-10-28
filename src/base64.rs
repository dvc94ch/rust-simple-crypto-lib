pub fn to_base64(bytes: &Vec<u8>) -> String {
    let size = bytes.len() as f32 / 3.0 * 4.0;
    let mut base64 = String::with_capacity(size.ceil() as usize);

    let mut iter = bytes.iter();
    let null = 0u8;
    loop {
        let b1 = iter.next();
        if b1.is_none() { break; }

        let (b1, b2, b3, b4) = pack_base64(
            *b1.unwrap(),
            *iter.next().unwrap_or(&null),
            *iter.next().unwrap_or(&null)
        );
        base64.push(lookup_base64(b1));
        base64.push(lookup_base64(b2));
        base64.push(lookup_base64(b3));
        base64.push(lookup_base64(b4));
    }

    if bytes.len() % 3 != 0 {
        base64.truncate(size.floor() as usize + bytes.len() % 3 - 1);
    }
    base64
}

pub fn from_base64(data: &String) -> Vec<u8> {
    let size = data.len() as f32 / 4.0 * 3.0;
    let mut bytes: Vec<u8> = Vec::with_capacity(size.ceil() as usize);

    let mut iter = data.chars().filter(|c| *c != '\n');
    let eq = '=';
    loop {
        let c1 = iter.next();
        if c1.is_none() { break; }

        let (b1, b2, b3) = unpack_base64(
            lookup_u8(c1.unwrap()),
            lookup_u8(iter.next().unwrap_or(eq)),
            lookup_u8(iter.next().unwrap_or(eq)),
            lookup_u8(iter.next().unwrap_or(eq))
        );
        bytes.push(b1);
        bytes.push(b2);
        bytes.push(b3);
    }
    if data.len() % 4 != 0 {
        bytes.truncate(size.floor() as usize);
    }
    bytes
}

fn pack_base64(b1: u8, b2: u8, b3: u8) -> (u8, u8, u8, u8) {
    (
        b1 >> 2,
        ((b1 << 4) & 0b110000u8) + (b2 >> 4),
        ((b2 << 2) & 0b111100u8) + (b3 >> 6),
        b3 & 0b111111u8
    )
}

fn unpack_base64(b1: u8, b2: u8, b3: u8, b4: u8) -> (u8, u8, u8) {
    (
        (b1 << 2) + (b2 >> 4),
        (b2 << 4) + (b3 >> 2),
        (b3 << 6) + b4
    )
}

fn lookup_base64(value: u8) -> char {
    match value {
        0...25 => {
            (value + 'A' as u8) as char
        },
        26...51 => {
            (value + 'a' as u8 - 26) as char
        },
        52...61 => {
            (value + '0' as u8 - 52) as char
        },
        62 => '+',
        63 => '/',
        _ => panic!("invalid value"),
    }
}

fn lookup_u8(value: char) -> u8 {
    match value {
        'A'...'Z' => {
            value as u8 - 'A' as u8
        },
        'a'...'z' => {
            value as u8 - 'a' as u8 + 26u8
        },
        '0'...'9' => {
            value as u8 - '0' as u8 + 52u8
        },
        '+' => 62u8,
        '/' => 63u8,
        '=' => 0u8,
        _ => panic!("invalid value {}", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii;

    #[test]
    fn test_to_base64() {
        let bytes = vec![0x4du8, 0x61u8, 0x6eu8];
        assert_eq!(to_base64(&bytes), String::from("TWFu"));
        let s =
            String::from("Man is distinguished, not only by his reason, but by this singular passion from ") +
            "other animals, which is a lust of the mind, that by a perseverance of delight " +
            "in the continued and indefatigable generation of knowledge, exceeds the short " +
            "vehemence of any carnal pleasure.";
        let expected =
            String::from("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz") +
            "IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg" +
            "dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu" +
            "dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo" +
            "ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4";
        let bytes = ascii::from_ascii(&s);
        assert_eq!(to_base64(&bytes), expected);
    }

    #[test]
    fn test_from_base64() {
        let bytes = vec![0x4du8, 0x61u8, 0x6eu8];
        assert_eq!(from_base64(&String::from("TWFu")), bytes);
        let s =
            String::from("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz") +
            "IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg" +
            "dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu" +
            "dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo" +
            "ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4";
        let expected =
            String::from("Man is distinguished, not only by his reason, but by this singular passion from ") +
            "other animals, which is a lust of the mind, that by a perseverance of delight " +
            "in the continued and indefatigable generation of knowledge, exceeds the short " +
            "vehemence of any carnal pleasure.";
        let bytes = from_base64(&s);
        assert_eq!(ascii::to_ascii(&bytes), expected);
    }

    #[test]
    fn test_lookup_base64() {
        assert_eq!(super::lookup_base64(19u8), 'T');
        assert_eq!(super::lookup_base64(22u8), 'W');
        assert_eq!(super::lookup_base64(5u8), 'F');
        assert_eq!(super::lookup_base64(46u8), 'u');
    }

    #[test]
    fn test_lookup_u8() {
        assert_eq!(super::lookup_u8('T'), 19u8);
        assert_eq!(super::lookup_u8('W'), 22u8);
        assert_eq!(super::lookup_u8('F'), 5u8);
        assert_eq!(super::lookup_u8('u'), 46u8);
    }

    #[test]
    fn test_pack_base64() {
        assert_eq!(super::pack_base64(77u8, 97u8, 110u8), (19u8, 22u8, 5u8, 46u8));
    }

    #[test]
    fn test_unpack_base64() {
        assert_eq!(super::unpack_base64(19u8, 22u8, 5u8, 46u8), (77u8, 97u8, 110u8));
    }
}
