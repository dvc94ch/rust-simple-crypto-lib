use aes::AesCipher;
use padder::Pkcs7Padder;
use traits::{BlockCipher, Padder, Mode};
use utils::{BlockIter, random_bytes};
use xor::xor;

pub struct EcbMode<C: BlockCipher, P: Padder> {
    crypter: C,
    padder: P,
}

impl <C, P> EcbMode<C, P> where C: BlockCipher, P: Padder {
    pub fn new(crypter: C, padder: P) -> EcbMode<C, P> {
        EcbMode {
            crypter: crypter,
            padder: padder,
        }
    }
}
pub struct CbcMode<C: BlockCipher, P: Padder> {
    crypter: C,
    padder: P,
}

impl <C, P> CbcMode<C, P> where C: BlockCipher, P: Padder {
    pub fn new(crypter: C, padder: P) -> CbcMode<C, P> {
        CbcMode {
            crypter: crypter,
            padder: padder,
        }
    }
}

pub struct AesEcbMode;

impl AesEcbMode {
    pub fn new(key: Vec<u8>) -> EcbMode<AesCipher, Pkcs7Padder> {
        EcbMode::new(AesCipher::new(key), Pkcs7Padder::new(16))
    }
}

pub struct AesCbcMode;

impl AesCbcMode {
    pub fn new(key: Vec<u8>) -> CbcMode<AesCipher, Pkcs7Padder> {
        CbcMode::new(AesCipher::new(key), Pkcs7Padder::new(16))
    }
}

impl <C, P> Mode for EcbMode<C, P> where C: BlockCipher, P: Padder {
    fn encrypt(&self, plain: &[u8]) -> Vec<u8> {
        let plain = self.padder.pad(plain);
        let mut cipher: Vec<u8> = Vec::with_capacity(plain.len());
        for block in BlockIter::new(plain) {
            cipher.append(&mut self.crypter.encrypt_block(&block[..]))
        }
        cipher
    }

    fn decrypt(&self, cipher: &[u8]) -> Vec<u8> {
        let mut plain: Vec<u8> = Vec::with_capacity(cipher.len());
        for block in BlockIter::new(cipher.to_vec()) {
            plain.append(&mut self.crypter.decrypt_block(&block[..]));
        }
        self.padder.unpad(&plain[..])
    }
}

impl <C, P> Mode for CbcMode<C, P> where C: BlockCipher, P: Padder {
    fn encrypt(&self, plain: &[u8]) -> Vec<u8> {
        let plain = self.padder.pad(plain);
        let mut cipher: Vec<u8> = Vec::with_capacity(plain.len() + 16);

        let mut iv = random_bytes(16);
        cipher.append(&mut iv[..].to_vec());

        for block in BlockIter::new(plain) {
            let block = &xor(&iv[..], &block[..]);
            iv = self.crypter.encrypt_block(&block[..]);
            cipher.append(&mut iv[..].to_vec());
        }
        cipher
    }

    fn decrypt(&self, cipher: &[u8]) -> Vec<u8> {
        let mut plain: Vec<u8> = Vec::with_capacity(cipher.len() - 16);

        let mut iv = cipher[0..16].to_vec();
        for block in BlockIter::new(cipher[16..cipher.len()].to_vec()) {
            let decrypted_block = self.crypter.decrypt_block(&block[..]);
            plain.append(&mut xor(&decrypted_block[..], &iv[..]));
            iv = block[..].to_vec();
        }
        self.padder.unpad(&plain[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils;
    use traits::Mode;

    #[test]
    fn test_aes_ecb_mode() {
        let key = utils::from_ascii(&String::from("YELLOW SUBMARINE"));
        let aes = AesEcbMode::new(key);
        let plain = utils::from_ascii(&String::from("An arbitrary length string"));
        let cipher = aes.encrypt(&plain);
        let new_plain = aes.decrypt(&cipher);
        assert_eq!(utils::to_ascii(&new_plain), "An arbitrary length string");
    }

    #[test]
    fn test_aes_cbc_mode() {
        let key = utils::from_ascii(&String::from("YELLOW SUBMARINE"));
        let aes = AesCbcMode::new(key);
        let plain = utils::from_ascii(&String::from("An arbitrary length string"));
        let cipher = aes.encrypt(&plain);
        let new_plain = aes.decrypt(&cipher);
        assert_eq!(utils::to_ascii(&new_plain), "An arbitrary length string");
    }
}
