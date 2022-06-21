use bitvec::vec;

use crate::{sbox::get_sbox, AesSize};

pub struct Key {
    size: AesSize,
    key: Vec<[u8; 4]>,
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

impl Key {
    pub fn row_amount(&self) -> usize {
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

    pub fn apply_row(&mut self, row_index: usize, func: fn (&[u8]) -> Vec<u8> ) {
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

    pub fn apply_col(&mut self, index: usize, func: fn (&[u8; 4]) -> Vec<u8> ) {
        let col = self.key[index];
        let new_col = func(&col);
        if new_col.len() != self.key.len() {
            panic!("Invalid col length");
        }
        for i in 0..self.key.len() {
            self.key[i][index] = new_col[i];
        }
    }

    pub fn apply_all(&mut self, func: fn (&[u8]) -> Vec<u8> ) {
        for i in 0..self.key.len() {
            self.apply_row(i, func);
        }
    }
}

fn shift(amount: i64) -> impl Fn(&[u8]) -> Vec<u8> {
    move |row| {
        let mut new_row = vec![];
        for v in row {
            new_row.push(*v);
        }
        for _ in 0..amount {
            let v = new_row.pop().unwrap();
            new_row.insert(0, v);
        }
        new_row
    }
}

fn sub_bytes() -> impl Fn(&[u8]) -> Vec<u8> {
    let sbox = get_sbox();
    move |row| {
        let mut new_row = vec![];
        for v in row {
            new_row.push(sbox.get(*v));
        }
        new_row
    }
}

pub fn key_schedule(cipher: &Key) -> Vec<Key> {
    let rounds = match cipher.size {
        AesSize::S128 => 10,
        AesSize::S192 => 12,
        AesSize::S256 => 14,
    };
    let mut keys = Vec::new();
    keys.push(cipher.clone());
    for round in 1..rounds {
        let mut new_cols = vec![];
        for i in 0..cipher.row_amount() {
            let new_col;
            if i == 0 {
                new_col = keys[round - 1].key[3].clone();
            } else {
                new_col = keys[round - 1].key[i - 1].clone();
            }
            new_cols.push(new_col);
        }
        let new_key = Key {
            size: cipher.size.clone(),
            key: new_cols,
        };
        keys.push(new_key);
    }
    keys
}
