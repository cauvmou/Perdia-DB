use aes::{cipher::{generic_array::GenericArray, typenum::{UInt, UTerm}, consts::{B1, B0}, KeyInit, BlockEncrypt, BlockDecrypt}, Aes128};
use crypto::{sha3::Sha3, digest::Digest};

pub struct Key {
    key: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>>,
    cipher: Aes128,
}

impl Key {
    pub fn new(key: Vec<u8>) -> Self {
        let mut hasher = Sha3::shake_128();
        hasher.input(&key);
        let mut key_array = [0u8; 16];
        hasher.result(&mut key_array);
        let key = GenericArray::from(key_array);
        let cipher = Aes128::new(&key.into());
        Self { key, cipher }
    }

    // TODO: Improve performance
    pub fn encrypt(&self, mut bytes: Vec<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::with_capacity(bytes.len() % 16);
        'encrypt_loop: 
        loop {
            if bytes.len() <= 16 {
                for _ in bytes.len()..16 {
                    bytes.push(0);
                }
                let mut block: [u8; 16] = [0u8; 16];
                block.copy_from_slice(bytes.as_slice());
                self.cipher.encrypt_block(&mut GenericArray::from(block));
                output.extend(&block);
                break 'encrypt_loop;
            } else {
                let (slice, b) = bytes.split_at(16);
                let mut block: [u8; 16] = [0u8; 16];
                block.copy_from_slice(slice);
                self.cipher.encrypt_block(&mut GenericArray::from(block));
                output.extend(&block);
                bytes = b.to_vec();
            }

        }
        output
    }

    // TODO: Improve performance
    pub fn decrypt(&self, mut bytes: Vec<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::with_capacity(bytes.len() % 16);
        'encrypt_loop: 
        loop {
            if bytes.len() <= 16 {
                for _ in bytes.len()..16 {
                    bytes.push(0);
                }
                let mut block: [u8; 16] = [0u8; 16];
                block.copy_from_slice(bytes.as_slice());
                self.cipher.decrypt_block(&mut GenericArray::from(block));
                output.extend(&block);
                break 'encrypt_loop;
            } else {
                let (slice, b) = bytes.split_at(16);
                let mut block: [u8; 16] = [0u8; 16];
                block.copy_from_slice(slice);
                self.cipher.decrypt_block(&mut GenericArray::from(block));
                output.extend(&block);
                bytes = b.to_vec();
            }

        }
        output
    }

}

impl From<Vec<u8>> for Key {
    fn from(key: Vec<u8>) -> Self {
        Self::new(key)
    }
}

impl From<String> for Key {
    fn from(key: String) -> Self {
        let key = key.into_bytes();
        Self::new(key)
    }
}

impl Into<GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>>> for Key {
    fn into(self) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>> {
        self.key
    }
}