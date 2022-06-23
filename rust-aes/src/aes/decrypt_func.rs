use super::{key::{shift, inv_sub_bytes as inv_sub_bytes_key, Key}, crypt_func::{g_mul, add_round_key}};

pub fn inv_sub_bytes(key: &mut Key) {
    key.apply_all(inv_sub_bytes_key);
}

pub fn inv_shift_rows(key: &mut Key) {
    key.apply_row(1, shift(1));
    key.apply_row(2, shift(2));
    key.apply_row(3, shift(3));
}


pub fn inv_mix_columns(key: &mut Key) {
    for col_index in 0..key.col_amount() {
        let col = key.get_col(col_index as isize);
        let new_col = [
            g_mul(col[0], 0x0e) ^ g_mul(col[1], 0x0b) ^ g_mul(col[2], 0x0d) ^ g_mul(col[3], 0x09),
            g_mul(col[0], 0x09) ^ g_mul(col[1], 0x0e) ^ g_mul(col[2], 0x0b) ^ g_mul(col[3], 0x0d),
            g_mul(col[0], 0x0d) ^ g_mul(col[1], 0x09) ^ g_mul(col[2], 0x0e) ^ g_mul(col[3], 0x0b),
            g_mul(col[0], 0x0b) ^ g_mul(col[1], 0x0d) ^ g_mul(col[2], 0x09) ^ g_mul(col[3], 0x0e),
        ];
        key.set_col(col_index, new_col.to_vec());
    }
}

pub fn decrypt_block(block: &Key, round_keys: &Vec<Key>) -> Key {
    let mut block = block.clone();
    add_round_key(&mut block, &round_keys[round_keys.len() - 1]);
    inv_shift_rows(&mut block);
    inv_sub_bytes(&mut block);
    for round_key in round_keys.iter().skip(1).rev().skip(1) {
        add_round_key(&mut block, round_key);
        inv_mix_columns(&mut block);
        inv_shift_rows(&mut block);
        inv_sub_bytes(&mut block);
    }
    add_round_key(&mut block, &round_keys[0]);
    block
}


