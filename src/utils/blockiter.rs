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

/*pub struct BlockIter {
    block_start: usize,
}

impl BlockIter {
    pub fn new() -> BlockIter {
        BlockIter {
            block_start: 0,
        }
    }
    pub fn next<'a>(&mut self, bytes: &'a Vec<u8>) -> Option<Box<&'a[u8]>> {
        if self.block_start >= bytes.len() { return None; }
        let block_start = self.block_start;
        let block_end = ::std::cmp::min(bytes.len(), self.block_start + 16);
        self.block_start += 16;
        Some(box &bytes[block_start..block_end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockiter() {
        let bytes = vec![0u8; 64];
        let mut iter = BlockIter::new();
        loop {
            if let Some(block) = iter.next(&bytes) {
                assert_eq!(*block, [0u8; 16]);
            } else {
                break;
            }
        }
    }
}*/
