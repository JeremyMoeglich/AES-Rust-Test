use std::fmt::Display;

use crate::{sbox::SBOX, aes_size::AesSize};

pub struct Key {
    pub size: AesSize,
    pub key: Vec<[u8; 4]>,
}

impl Clone for Key {
    fn clone(&self) -> Self {
        Key {
            size: match self.size {
                AesSize::S128 => AesSize::S128,
                AesSize::S192 => AesSize::S192,
                AesSize::S256 => AesSize::S256,
            },
            key: self.key.clone(),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for row_index in 0..self.col_amount() {
            let row = self.get_row(row_index);
            for byte in row.iter() {
                text.push_str(&format!("{:02x} ", byte));
            }
            text.push_str("\n");
        }
        write!(f, "{}", text)
    }
}

impl Key {
    pub fn col_amount(&self) -> usize {
        self.validate_key();
        match self.size {
            AesSize::S128 => 4,
            AesSize::S192 => 6,
            AesSize::S256 => 8,
        }
    }

    pub fn validate_key(&self) {
        let key_len = self.key.len();
        if key_len
            != (match self.size {
                AesSize::S128 => 4,
                AesSize::S192 => 6,
                AesSize::S256 => 8,
            })
        {
            panic!("Invalid key length");
        }
    }

    pub fn apply_row(&mut self, row_index: usize, func: impl Fn(&[u8]) -> Vec<u8>) {
        let mut row = vec![];
        for v in self.key.iter() {
            row.push(v[row_index]);
        }
        let new_row = func(&row);
        if new_row.len() != self.key.len() {
            panic!("Invalid row length");
        }
        for i in 0..self.key.len() {
            self.key[i][row_index] = new_row[i];
        }
    }

    pub fn apply_col(&mut self, index: usize, func: impl Fn(&[u8; 4]) -> Vec<u8>) {
        let col = self.key[index];
        let new_col = func(&col);
        if new_col.len() != self.key.len() {
            panic!("Invalid col length");
        }
        for i in 0..self.key.len() {
            self.key[i][index] = new_col[i];
        }
    }

    pub fn get_row(&self, row_index: usize) -> Vec<u8> {
        let mut row = vec![];
        for v in self.key.iter() {
            row.push(v[row_index]);
        }
        row
    }

    pub fn get_col(&self, index: isize) -> Vec<u8> {
        let u_index = (if index < 0 {
            index + self.key.len() as isize
        } else {
            index
        }) as usize;
        let col = self.key[u_index];
        let mut col_vec = vec![];
        for v in col.iter() {
            col_vec.push(*v);
        }
        col_vec
    }

    pub fn set_row(&mut self, row_index: isize, row: Vec<u8>) {
        let u_row_index = (if row_index < 0 {
            row_index + self.col_amount() as isize
        } else {
            row_index
        }) as usize;
        if row.len() != self.key.len() {
            panic!("Invalid row length");
        }
        for i in 0..self.key.len() {
            self.key[i][u_row_index] = row[i];
        }
    }

    pub fn set_col(&mut self, index: usize, col: Vec<u8>) {
        if col.len() != self.key.len() {
            panic!("Invalid col length");
        }
        for i in 0..4 {
            self.key[index][i] = col[i];
        }
    }

    pub fn apply_all(&mut self, func: fn(&[u8]) -> Vec<u8>) {
        for i in 0..self.key.len() {
            self.apply_row(i, func);
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.key.len() {
            for j in 0..self.key[i].len() {
                self.key[i][j] = 0;
            }
        }
    }
}

pub fn shift(mut amount: i64) -> impl Fn(&[u8]) -> Vec<u8> {
    let swap = if amount < 0 { true } else { false };
    if swap {
        amount = -amount;
    }
    move |row| {
        let mut new_row = vec![];
        for v in row {
            new_row.push(*v);
        }
        if swap {
            new_row.reverse();
        }
        for _ in 0..amount {
            let v = new_row.pop().unwrap();
            new_row.insert(0, v);
        }
        if swap {
            new_row.reverse();
        }
        new_row
    }
}

pub fn sub_bytes(row: &[u8]) -> Vec<u8> {
    let mut new_row = vec![];
    for v in row {
        new_row.push(SBOX.get(*v));
    }
    new_row
}