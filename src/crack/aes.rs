use traits::Mode;
use utils;

pub fn find_blocksize(cryptor: Box<Mode>) -> usize {
    let bytes = vec![0u8];
    let encrypted_bytes = cryptor.encrypt(&bytes);
    encrypted_bytes.len()
}

pub fn is_ecb_mode(cryptor: Box<Mode>) -> bool {
    let plain_text = vec![0u8; 64];
    let cipher_text = cryptor.encrypt(&plain_text);
    contains_duplicate_blocks(cipher_text)
}

pub fn contains_duplicate_blocks(bytes: Vec<u8>) -> bool {
    let mut block_vec: Vec<[u8; 16]> = Vec::new();
    for new_block in utils::BlockIter::new(bytes) {
        for block in &block_vec {
            if block == &new_block {
                return true;
            }
        }
        block_vec.push(new_block);
    }
    false
}

pub fn find_duplicate_block(bytes: Vec<u8>) -> isize {
    let mut block_vec: Vec<[u8; 16]> = Vec::new();
    for new_block in utils::BlockIter::new(bytes) {
        for (i, block) in block_vec.iter().enumerate() {
            if block == &new_block {
                return i as isize;
            }
        }
        block_vec.push(new_block);
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use symm::AesEcbMode;
    use utils;
    use traits::Mode;

    #[test]
    fn test_find_blocksize() {
        let cryptor = box AesEcbMode::new(utils::random_bytes(16)) as Box<Mode>;
        assert_eq!(find_blocksize(cryptor), 16);
    }
}
