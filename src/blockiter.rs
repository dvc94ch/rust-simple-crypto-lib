pub struct BlockIter {
    bytes: Vec<u8>,
    block_start: usize,
}

impl BlockIter {
    pub fn new(bytes: Vec<u8>) -> BlockIter {
        BlockIter {
            bytes: bytes,
            block_start: 0,
        }
    }
}

impl Iterator for BlockIter {
    type Item = [u8; 16];
    fn next(&mut self) -> Option<[u8; 16]> {
        if self.block_start >= self.bytes.len() { return None; }
        let mut block = [4u8; 16];
        let block_end = ::std::cmp::min(self.bytes.len(), self.block_start + 16);
        ::std::slice::bytes::copy_memory(&self.bytes[self.block_start..block_end], &mut block);
        self.block_start += 16;
        Some(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockiter() {
        let iter = BlockIter::new(vec![0u8; 64]);
        for block in iter {
            assert_eq!(block, [0u8; 16]);
        }
    }
}
