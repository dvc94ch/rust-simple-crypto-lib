use traits::Padder;

pub struct Pkcs7Padder {
    blocksize: usize,
}

impl Pkcs7Padder {
    pub fn new(blocksize: usize) -> Pkcs7Padder {
        Pkcs7Padder {
            blocksize: blocksize,
        }
    }
}

impl Padder for Pkcs7Padder {
    fn pad(&self, bytes: &[u8]) -> Vec<u8> {
        let mut bytes = bytes.to_vec();
        let mut padding = self.blocksize - bytes.len() % self.blocksize;
        if padding == 0 { padding = 16; }
        bytes.append(&mut vec![padding as u8; padding]);
        bytes
    }

    fn unpad(&self, bytes: &[u8]) -> Vec<u8> {
        let padding = bytes[bytes.len() - 1] as usize;
        if padding > self.blocksize { return bytes.to_vec(); }
        bytes[0..(bytes.len() - padding)].to_vec()
    }
}
