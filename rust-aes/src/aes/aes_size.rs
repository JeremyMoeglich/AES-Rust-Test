extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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
    pub fn parse(size: &str) -> Result<AesSize, String> {
        match size {
            "128" => Ok(AesSize::S128),
            "192" => Ok(AesSize::S192),
            "256" => Ok(AesSize::S256),
            _ => Err(format!("Invalid AES size: {}", size)),
        }
    }
}
