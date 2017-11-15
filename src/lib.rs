extern crate ocbaes128_sys;

pub struct CryptState(ocbaes128_sys::CryptState);

impl CryptState {

    pub fn new() -> Self {
        unsafe {
            CryptState(ocbaes128_sys::CryptState::new())
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            self.0.isValid()
        }
    }

    pub fn set_key(&mut self, rkey: &[u8], eiv: &[u8], div: &[u8]) {
        unsafe {
            self.0.setKey(rkey.as_ptr(), eiv.as_ptr(), div.as_ptr());
        }
    }

    pub fn get_key(&self) -> [u8; 16] {
        self.0.raw_key
    }

    pub fn get_encrypt_iv(&self) -> [u8; 16] {
        self.0.encrypt_iv
    }

    pub fn get_decrypt_iv(&self) -> [u8; 16] {
        self.0.decrypt_iv
    }

    pub fn encrypt(&mut self, msg: &[u8], enc: &mut [u8]) {
        unsafe {
            self.0.encrypt(msg.as_ptr(), enc.as_mut_ptr(), msg.len() as u32);
        }
    }

    pub fn decrypt(&mut self, enc: &[u8], dec: &mut [u8]) -> bool {
        unsafe {
            self.0.decrypt(enc.as_ptr(), dec.as_mut_ptr(), enc.len() as u32)
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

