use xor::xor;
use traits::{Cipher, Padder};
use aes::AesCipher;
use padder::PkcsPadder;

pub trait Mode {
    fn encrypt(&self, bytes: &Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, bytes: &Vec<u8>) -> Vec<u8>;
}

pub struct EcbMode<C: Cipher, P: Padder> {
    cipher: C,
    padder: P,
}

pub struct CbcMode<C: Cipher, P: Padder> {
    cipher: C,
    padder: P,
    iv: [u8; 16],
}

pub struct AesEcbMode;

impl AesEcbMode {
    pub fn new(key: Vec<u8>) -> EcbMode<AesCipher, PkcsPadder> {
        let cipher = AesCipher::new(key);
        let padder = PkcsPadder {};
        EcbMode {
            cipher: cipher,
            padder: padder,
        }
    }
}

pub struct AesCbcMode;

impl AesCbcMode {
    pub fn new(key: Vec<u8>, iv: [u8; 16]) -> CbcMode<AesCipher, PkcsPadder> {
        let cipher = AesCipher::new(key);
        let padder = PkcsPadder {};
        CbcMode {
            cipher: cipher,
            padder: padder,
            iv: iv,
        }
    }
}

impl Mode for EcbMode<AesCipher, PkcsPadder> {
    fn encrypt(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let bytes = self.padder.pad(&bytes);
        let mut cipher_text: Vec<u8> = Vec::with_capacity(bytes.len());
        unsafe {
            cipher_text.set_len(bytes.len());
            for offset in (0..bytes.len()).step_by(16) {
                let encrypted_block = self.cipher.encrypt_block(&bytes[offset..(offset + 16)]);
                ::std::slice::bytes::copy_memory(&encrypted_block[..], &mut cipher_text[offset..(offset + 16)]);
            }
        }
        cipher_text
    }

    fn decrypt(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut plain_text: Vec<u8> = Vec::with_capacity(bytes.len());
        unsafe {
            plain_text.set_len(bytes.len());
            for offset in (0..bytes.len()).step_by(16) {
                let decrypted_block = self.cipher.decrypt_block(&bytes[offset..(offset + 16)]);
                ::std::slice::bytes::copy_memory(&decrypted_block[..], &mut plain_text[offset..(offset + 16)]);
            }
        }
        self.padder.unpad(plain_text)
    }
}

impl Mode for CbcMode<AesCipher, PkcsPadder> {
    fn encrypt(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let bytes = self.padder.pad(&bytes);
        let mut cipher_text: Vec<u8> = Vec::with_capacity(bytes.len());
        unsafe {
            cipher_text.set_len(bytes.len());
            let mut encrypted_block = self.iv.to_vec();
            for offset in (0..bytes.len()).step_by(16) {
                let xored_block = xor(&encrypted_block[..], &bytes[offset..(offset + 16)]);
                encrypted_block = self.cipher.encrypt_block(&xored_block[..]);
                ::std::slice::bytes::copy_memory(&encrypted_block[..], &mut cipher_text[offset..(offset + 16)]);
            }
        }
        cipher_text
    }

    fn decrypt(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut plain_text: Vec<u8> = Vec::with_capacity(bytes.len());
        unsafe {
            plain_text.set_len(bytes.len());
            let mut encrypted_block = self.iv.to_vec();
            for offset in (0..bytes.len()).step_by(16) {
                let xored_block = self.cipher.decrypt_block(&bytes[offset..(offset + 16)]);
                let decrypted_block = xor(&encrypted_block[..], &xored_block[..]);
                ::std::slice::bytes::copy_memory(&decrypted_block[..], &mut plain_text[offset..(offset + 16)]);
                encrypted_block = (&bytes[offset..(offset + 16)]).to_vec();
            }
        }
        self.padder.unpad(plain_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii;

    #[test]
    fn test_aes_ecb_mode() {
        let key = ascii::from_ascii(&String::from("YELLOW SUBMARINE"));
        let aes = AesEcbMode::new(key);
        let plain_text = ascii::from_ascii(&String::from("An arbitrary length string"));
        let cipher_text = aes.encrypt(&plain_text);
        let decrypted_blocks = aes.decrypt(&cipher_text);
        assert_eq!(ascii::to_ascii(&decrypted_blocks), "An arbitrary length string");
    }

    #[test]
    fn test_aes_cbc_mode() {
        let key = ascii::from_ascii(&String::from("YELLOW SUBMARINE"));
        let iv = [0u8; 16];
        let aes = AesCbcMode::new(key, iv);
        let plain_text = ascii::from_ascii(&String::from("An arbitrary length string"));
        let cipher_text = aes.encrypt(&plain_text);
        let decrypted_blocks = aes.decrypt(&cipher_text);
        assert_eq!(ascii::to_ascii(&decrypted_blocks), "An arbitrary length string");
    }
}
