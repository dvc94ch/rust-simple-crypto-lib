pub trait Cipher {
    fn encrypt_block(&self, block: &[u8]) -> Vec<u8>;
    fn decrypt_block(&self, block: &[u8]) -> Vec<u8>;
}

pub trait Padder {
    fn pad(&self, bytes: &Vec<u8>) -> Vec<u8>;
    fn unpad(&self, mut bytes: Vec<u8>) -> Vec<u8>;
}
