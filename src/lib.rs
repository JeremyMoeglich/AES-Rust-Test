mod key_schedule;
mod sbox;
pub enum AesSize {
    S128,
    S192,
    S256,
}

impl Clone for AesSize {
    fn clone(&self) -> Self {
        match self {
            AesSize::S128 => AesSize::S128,
            AesSize::S192 => AesSize::S192,
            AesSize::S256 => AesSize::S256,
        }
    }
}