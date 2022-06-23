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

impl AesSize {
    pub fn parse(size: &str) -> Option<AesSize> {
        match size {
            "128" => Some(AesSize::S128),
            "192" => Some(AesSize::S192),
            "256" => Some(AesSize::S256),
            _ => None,
        }
    }
}
