use openssl::crypto::symm::{Crypter, Mode, Type};
use traits::Cipher;

pub struct AesCipher {
    key: Vec<u8>,
    crypter: Crypter,
}

impl AesCipher {
    pub fn new(key: Vec<u8>) -> AesCipher {
        let openssl_type = match key.len() {
            16 => Type::AES_128_ECB,
            32 => Type::AES_256_ECB,
            _ => panic!("Keysize must be 128 or 256 bits"),
        };
        AesCipher {
            key: key,
            crypter: Crypter::new(openssl_type),
        }
    }
    fn aes(&self, mode: Mode, block: &[u8]) -> Vec<u8> {
        if block.len() != 16 {
            panic!("Blocksize must be 16 bytes");
        }
        self.crypter.init(mode, &self.key, &[0u8; 16]);
        self.crypter.pad(false);
        let block = self.crypter.update(&block);
        self.crypter.finalize();
        block
    }
}

impl Cipher for AesCipher {
    fn encrypt_block(&self, block: &[u8]) -> Vec<u8> {
        self.aes(Mode::Encrypt, &block)
    }
    fn decrypt_block(&self, block: &[u8]) -> Vec<u8> {
        self.aes(Mode::Decrypt, &block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii;
    use traits::Cipher;

    #[test]
    fn test_aes_cipher() {
        let key = ascii::from_ascii(&String::from("YELLOW SUBMARINE"));
        let aes = AesCipher::new(key);
        let bytes = ascii::from_ascii(&String::from("YELLOW SUBMARINE"));

        let encrypted_blocks = aes.encrypt_block(&bytes[..]);
        let decrypted_blocks = aes.decrypt_block(&encrypted_blocks[..]);
        assert_eq!(ascii::to_ascii(&decrypted_blocks), "YELLOW SUBMARINE");
    }
}
