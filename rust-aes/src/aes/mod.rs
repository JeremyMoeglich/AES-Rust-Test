use std::string::FromUtf8Error;

pub use self::aes_size::AesSize;
pub use self::key::Key;

mod aes_size;
mod crypt_func;
mod decrypt_func;
mod encrypt_func;
mod key;
mod key_schedule;
mod rcon;
mod sbox;

pub fn encrypt(cipher: &Key, plaintext: &str) -> Result<Vec<u8>, String> {
    if plaintext.find("\0").is_some() {
        return Err("Plaintext contains null byte".into());
    }
    let round_keys = key_schedule::key_schedule(&cipher);
    let blocks = plaintext.as_bytes().chunks(16);
    let mut ciphertext = Vec::new();
    for block in blocks {
        let mut byte_block = block.to_vec();
        for _ in block.len()..16 {
            byte_block.push(0);
        }
        let block = Key::from_bytes(&byte_block);
        let encrypted_block = encrypt_func::encrypt_block(&block, &round_keys);
        ciphertext.extend_from_slice(&encrypted_block.to_bytes());
    }
    Ok(ciphertext)
}

pub fn decrypt(cipher: &Key, ciphertext: &[u8]) -> Result<String, FromUtf8Error> {
    let round_keys = key_schedule::key_schedule(&cipher);
    let blocks = ciphertext.chunks(16);
    let mut plaintext = Vec::new();
    for block in blocks {
        let byte_block = block.to_vec();
        let block = Key::from_bytes(&byte_block);
        let decrypted_block = decrypt_func::decrypt_block(&block, &round_keys);
        plaintext.extend_from_slice(&decrypted_block.to_bytes());
    }
    let string = String::from_utf8(plaintext)?;
    let null_index = match string.find("\0") {
        Some(index) => index,
        None => string.len(),
    };
    Ok(string[..null_index].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypt_test() {
        let cipher = Key::from_vec(vec![
            [0x2b, 0x7e, 0x15, 0x16],
            [0x28, 0xae, 0xd2, 0xa6],
            [0xab, 0xf7, 0x15, 0x88],
            [0x09, 0xcf, 0x4f, 0x3c],
        ]);
        let block = Key::from_vec(vec![
            [0x32, 0x43, 0xf6, 0xa8],
            [0x88, 0x5a, 0x30, 0x8d],
            [0x31, 0x31, 0x98, 0xa2],
            [0xe0, 0x37, 0x07, 0x34],
        ]);
        let round_keys = key_schedule::key_schedule(&cipher);
        let encrypted_block = encrypt_func::encrypt_block(&block, &round_keys);
        println!("key\n{}", encrypted_block);
        assert_eq!(
            encrypted_block.key,
            vec![
                [0x39, 0x25, 0x84, 0x1d],
                [0x02, 0xdc, 0x09, 0xfb],
                [0xdc, 0x11, 0x85, 0x97],
                [0x19, 0x6a, 0x0b, 0x32],
            ]
        );
        let decrypted_block = decrypt_func::decrypt_block(&encrypted_block, &round_keys);
        println!("key\n{}", decrypted_block);
        assert_eq!(decrypted_block.key, block.key);
    }

    #[test]
    fn password_test() {
        let password = "Password1234";
        let cipher = Key::from_password(password, AesSize::S128);
        let plaintext = "Hello, World!";
        let ciphertext = encrypt(&cipher, plaintext).expect("encryption failed");
        println!("ciphertext: {:?}", ciphertext);
        let decrypted_plaintext = decrypt(&cipher, &ciphertext);
        assert_eq!(plaintext, decrypted_plaintext.unwrap());
    }

    #[test]
    fn size_test() {
        let password = "ℵΓ∅Γℍ∂ΔΣΔℍ∅∨³Ψ⁴ωω∅γιß₉τ₉τ⊂ßε₅ßψωψßωψßωψßωßψ₈⁺τ";
        let cipher = Key::from_password(password, AesSize::S256);
        let plaintext = "Hello, World!";
        let ciphertext = encrypt(&cipher, plaintext).expect("encryption failed");
        let decrypted_plaintext = decrypt(&cipher, &ciphertext);
        assert_eq!(plaintext, decrypted_plaintext.unwrap());
    }
}
