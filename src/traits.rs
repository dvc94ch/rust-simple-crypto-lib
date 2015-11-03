pub trait BlockCipher {
    fn encrypt_block(&self, block: &[u8]) -> Vec<u8>;
    fn decrypt_block(&self, block: &[u8]) -> Vec<u8>;
}

pub trait Mode {
    fn encrypt(&self, bytes: &[u8]) -> Vec<u8>;
    fn decrypt(&self, bytes: &[u8]) -> Vec<u8>;
}

pub trait Padder {
    fn pad(&self, bytes: &[u8]) -> Vec<u8> {
        bytes.to_vec()
    }
    fn unpad(&self, bytes: &[u8]) -> Vec<u8> {
        bytes.to_vec()
    }
}
