use super::{
    rc4::Rc4,
    result::{CryptResult, Error},
};
use hmac::{Hmac, Mac};
use md5::Md5;

pub struct KerberosEncryption {
    key: Vec<u8>,
    cipher: Rc4,
}

impl KerberosEncryption {
    pub fn new(key: Vec<u8>) -> Self {
        KerberosEncryption {
            cipher: Rc4::new(&key),
            key,
        }
    }

    pub fn encrypt(&mut self, buffer: &[u8]) -> CryptResult<Vec<u8>> {
        let mut encrypted = self.cipher.encrypt(buffer)?;

        let mut mac = Hmac::<Md5>::new_from_slice(&self.key).map_err(|_| Error::InvalidKeySize)?;
        mac.update(&encrypted);

        encrypted.append(&mut mac.finalize().into_bytes().to_vec());
        Ok(encrypted)
    }

    pub fn decrypt(&mut self, buffer: &[u8]) -> CryptResult<Vec<u8>> {
        if self.validate(buffer)? {
            let offset = buffer.len() - 0x10;
            let encrypted = &buffer[..offset];

            self.cipher.decrypt(encrypted)
        } else {
            Err(Error::InvalidChecksum)
        }
    }

    pub fn validate(&self, buffer: &[u8]) -> CryptResult<bool> {
        let offset = buffer.len() - 0x10;
        let data = &buffer[..offset];
        let checksum = &buffer[offset..];

        let mut cipher =
            Hmac::<Md5>::new_from_slice(&self.key).map_err(|_| Error::InvalidKeySize)?;
        cipher.update(data);

        let mac = cipher.finalize().into_bytes().to_vec();
        Ok(mac.as_slice() == checksum)
    }
}

pub fn derive_kerberos_key(principal_id: u32, password: &[u8]) -> [u8; 16] {
    let hash_count = 65000 + (principal_id % 1024);

    // This is a bit awkward, but it allows
    // key to be an array with a known size
    // without dropping temporary values
    let mut key = super::md5::hash(password);
    for _ in 0..(hash_count - 1) {
        key = super::md5::hash(&key);
    }

    key
}

pub struct Ticket {
    pub session_key: Vec<u8>,
    pub server_pid: u32,
    pub ticket_data: Vec<u8>,
}

pub struct TicketData {
    pub ticket_key: Vec<u8>,
    pub ticket_info: Vec<u8>,
}

pub struct TicketInfo {
    pub datetime: u64,
    pub user_pid: u32,
    pub session_key: Vec<u8>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_encrypt_and_decrypt() {
        let data = "kerberos test".as_bytes();
        let key = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let mut kerb = KerberosEncryption::new(key.clone());
        let encrypted = kerb.encrypt(data).expect("Failed to encrypt");
        kerb = KerberosEncryption::new(key);
        let decrypted = kerb
            .decrypt(encrypted.as_slice())
            .expect("Failed to decrypt");
        assert_eq!(data, decrypted);
    }

    #[test]
    fn should_encrypt() {
        let data = "encrypt me".as_bytes();
        let result = vec![
            0xbb, 0x76, 0xea, 0x33, 0xda, 0x47, 0x29, 0x1a, 0xe7, 0x63, 0xef, 0xda, 0xaa, 0xb8,
            0x27, 0xe0, 0x48, 0x4c, 0x68, 0xe0, 0xf7, 0x93, 0x62, 0x27, 0x48, 0x70,
        ];
        let key = vec![0];
        let mut kerb = KerberosEncryption::new(key);
        let encrypted = kerb.encrypt(data).expect("Failed to encrypt");
        assert_eq!(encrypted, result);
    }

    #[test]
    fn should_decrypt() {
        let result = "decrypt me".as_bytes();
        let data = vec![
            0xba, 0x7d, 0xea, 0x33, 0xda, 0x47, 0x29, 0x1a, 0xe7, 0x63, 0x42, 0xc4, 0xe2, 0xdf,
            0x3c, 0x0d, 0x5a, 0xad, 0xea, 0x22, 0xa2, 0x60, 0xd0, 0x2a, 0xb3, 0x50,
        ];
        let key = vec![0];
        let mut kerb = KerberosEncryption::new(key);
        let decrypted = kerb.decrypt(&data).expect("Failed to decrypt");
        assert_eq!(decrypted, result);
    }
}