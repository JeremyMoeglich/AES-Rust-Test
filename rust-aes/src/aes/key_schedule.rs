use super::{key::{Key, shift, sub_bytes}, rcon::RCON, aes_size::AesSize};


pub fn key_schedule(cipher: &Key) -> Vec<Key> {
    let rounds = match cipher.size {
        AesSize::S128 => 10,
        AesSize::S192 => 12,
        AesSize::S256 => 14,
    };
    let mut keys = Vec::new();
    let shift1 = shift(-1);
    keys.push(cipher.clone());
    for round_index in 0..rounds {
        let mut key = cipher.clone();
        key.clear();
        let col_amount = key.col_amount();
        let previous_key = &keys[round_index];
        {
            let rcon = RCON.get(round_index);
            let next_col = previous_key.get_col(3);
            let offset4_col = previous_key.get_col(0);
            let shifted_col = shift1(&next_col);
            let mut subsituted_col = sub_bytes(&shifted_col);
            for row_index in 0..subsituted_col.len() {
                subsituted_col[row_index] =
                    subsituted_col[row_index] ^ offset4_col[row_index] ^ rcon[row_index];
            }
            key.set_col(0, subsituted_col);
        }
        for i in 1..col_amount {
            let mut next_col = key.get_col(i as isize - 1);
            let offset_col = previous_key.get_col(i as isize);
            for row_index in 0..next_col.len() {
                next_col[row_index] = next_col[row_index] ^ offset_col[row_index];
            }
            key.set_col(i, next_col);
        }
        keys.push(key);
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_schedule_test() {
        let cipher = Key {
            size: AesSize::S128,
            key: vec![
                [0x2b, 0x7e, 0x15, 0x16],
                [0x28, 0xae, 0xd2, 0xa6],
                [0xab, 0xf7, 0x15, 0x88],
                [0x09, 0xcf, 0x4f, 0x3c],
            ],
        };
        println!("ci\n{}", cipher);
        let keys = key_schedule(&cipher);
        for (i, key) in keys.iter().enumerate() {
            println!("key{}\n{}", i, key);
        }
        assert_eq!(keys.len(), 11);
        assert_eq!(keys[0].key, cipher.key);
        assert_eq!(
            keys[1].key,
            vec![
                [0xa0, 0xfa, 0xfe, 0x17],
                [0x88, 0x54, 0x2c, 0xb1],
                [0x23, 0xa3, 0x39, 0x39],
                [0x2a, 0x6c, 0x76, 0x05],
            ]
        );
        assert_eq!(
            keys[10].key,
            vec![
                [0xd0, 0x14, 0xf9, 0xa8],
                [0xc9, 0xee, 0x25, 0x89],
                [0xe1, 0x3f, 0x0c, 0xc8],
                [0xb6, 0x63, 0x0c, 0xa6]
            ]
        )
    }
}
