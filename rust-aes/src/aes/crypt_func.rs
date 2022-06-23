use super::key::Key;

pub fn g_mul(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;
    for _ in 0..8 {
        if b & 1 == 1 {
            result ^= a;
        }

        let set = (a & 0x80) != 0;
        a <<= 1;
        if set {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    result
}

pub fn add_round_key(key: &mut Key, round_key: &Key) {
    let mut new_cols = Vec::new();
    for col_index in 0..key.col_amount() {
        let col_index = col_index as isize;
        let col = key.get_col(col_index);
        let round_col = round_key.get_col(col_index);
        let mut new_col = [0 as u8; 4];
        for row_index in 0..4 {
            let v1 = col[row_index];
            let v2 = round_col[row_index];
            new_col[row_index] = v1 ^ v2;
        }
        new_cols.push(new_col);
    }
    key.key = new_cols;
}
