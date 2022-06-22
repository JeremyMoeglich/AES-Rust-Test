use crate::{
    aes_size::AesSize,
    key::{shift, sub_bytes as sub_bytes_key, Key},
};
use std::num::Wrapping;

fn sub_bytes(key: &mut Key) {
    key.apply_all(sub_bytes_key);
}

fn shift_rows(key: &mut Key) {
    key.apply_row(1, shift(1));
    key.apply_row(2, shift(2));
    key.apply_row(3, shift(3));
}

fn mix_columns(key: &mut Key) {
    fn g_mul(mut a: u8, mut b: u8) -> u8 {
        let mut result = 0;
        for _ in 0..8 {
            if b & 1 == 1 {
                result ^= a;
            }
            a = a.rotate_left(1);
            b >>= 1;
        }
        result
    }

    for col_index in 0..key.col_amount() {
        let col = key.get_col(col_index as isize);

        key.set_col(
            col_index,
            [
                g_mul(col[0], 0x02) ^ col[1] ^ col[2] ^ g_mul(col[3], 0x03),
                g_mul(col[0], 0x03) ^ g_mul(col[1], 0x02) ^ col[2] ^ col[3],
                col[0] ^ g_mul(col[1], 0x03) ^ g_mul(col[2], 0x02) ^ col[3],
                col[0] ^ col[1] ^ g_mul(col[2], 0x03) ^ g_mul(col[3], 0x02),
            ]
            .to_vec(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_columns_test() {
        let mut key = Key {
            size: AesSize::S128,
            key: vec![
                [0xd4, 0xbf, 0x5d, 0x30],
                [0xe0, 0xb4, 0x52, 0xae],
                [0xb8, 0x41, 0x11, 0xf1],
                [0x1e, 0x27, 0x98, 0xe5],
            ],
        };
        mix_columns(&mut key);
        println!("{}", key);
        assert_eq!(
            key.key,
            vec![
                [0x04, 0x66, 0x81, 0xe5],
                [0xe0, 0xcb, 0x19, 0x9a],
                [0x48, 0xf8, 0xd3, 0xd3],
                [0x28, 0x06, 0x26, 0x4c]
            ]
        );
    }
}
