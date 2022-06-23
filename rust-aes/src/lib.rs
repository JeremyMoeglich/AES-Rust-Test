extern crate cfg_if;
extern crate wasm_bindgen;

pub mod aes;

use aes::AesSize;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn encrypt(password: &str, plaintext: &str, size_string: &str) -> Vec<u8> {
    let size = AesSize::parse(&size_string).expect("Invalid AES size");
    let cipher = aes::Key::from_password(password, size);
    let ciphertext = aes::encrypt(&cipher, plaintext).expect("Failed to encrypt");
    ciphertext
}

#[wasm_bindgen]
pub fn decrypt(password: &str, ciphertext: &[u8], size_string: &str) -> String {
    let size = AesSize::parse(&size_string).expect("Invalid AES size");
    let cipher = aes::Key::from_password(password, size);
    let plaintext = aes::decrypt(&cipher, ciphertext)
        .map_err(|e| e.to_string())
        .expect("Failed to decrypt");
    plaintext
}

#[wasm_bindgen]
pub fn get_cipher(password: &str, size_string: &str) -> String {
    let size = AesSize::parse(&size_string).expect("Invalid AES size");
    let cipher = aes::Key::from_password(password, size);
    cipher.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let password = "passwrd";
        let plaintext = "plaintext";
        let size_string = "128";
        let ciphertext = encrypt(password, plaintext, size_string);
        let decrypted = decrypt(password, &ciphertext, size_string);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn empty() {
        let password = "";
        let plaintext = "";
        let size_string = "128";
        let ciphertext = encrypt(password, plaintext, size_string);
        let decrypted = decrypt(password, &ciphertext, size_string);
        assert_eq!(plaintext, decrypted);
    }
}
