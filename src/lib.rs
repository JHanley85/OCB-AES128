extern crate ocbaes128_sys;

pub struct CryptState(ocbaes128_sys::CryptState);

impl CryptState {

    pub fn new() -> Self {
        unsafe {
            CryptState(ocbaes128_sys::CryptState::new())
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

    // pub fn set_key(&mut self, rkey: &[u8; 16], eiv: &[u8; 16], div: &[u8; 16]) {
    //     unsafe {
    //         self.0.setKey(rkey.as_ptr(), eiv.as_ptr(), div.as_ptr());
    //     }
    // }

    pub fn encrypt(&mut self, msg: &[u8]) -> std::vec::Vec<u8> {
        let mut enc = vec![0u8; msg.len() + 4];
        unsafe {
            self.0.encrypt(msg.as_ptr(), enc[..].as_mut_ptr(), msg.len() as u32);
        }
        enc
    }

}

// pub fn encrypt(key: &[u8; 16], nonce: &[u8; 16], msg: &[u8], out: &mut [u8]) -> Vec<u8> {
//     unsafe {
//         let mut cs = ocbaes128_sys::CryptState::new();
//         cs.setKey(msg.as_ptr(), dec.as_ptr(), enc.as_ptr());

//         // let mut buf = [0u8; 19];
//         cs.obs_encrypt(msg.as_ptr(), out.as_mut_ptr(), 15);
//     }
//     Vec::<u8>::new()
// }


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

