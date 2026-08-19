use crate::utils::to_bijbase256;

#[cfg(test)]
use crate::utils::from_bijbase256;

pub type Key = (u128, u128);

const fn rijndael_sbox() -> [u8; 256] {
    let rl = u8::rotate_left;

    let mut arr = [0x63_u8; 256];
    let (mut p, mut q) = (1u8, 1u8);

    loop {
        // multiply p by 3
        p ^= (p << 1) ^ (p >> 7) * 0x1B;

        // divide q by 3
        q ^= q << 1;
        q ^= q << 2;
        q ^= q << 4;
        q ^= (q >> 7) * 0x09;

        // compute affine transformation
        let x = q ^ rl(q, 1) ^ rl(q, 2) ^ rl(q, 3) ^ rl(q, 4);

        arr[p as usize] ^= x;
        if p == 1 {
            break;
        }
    }

    arr
}

const SBOX: [u8; 256] = rijndael_sbox();

fn hash(x: u8, y: u8) -> (u8, u8) {
    let uf = usize::from;
    let s1 = (x & 0x55) | (y & 0xAA);
    let s2 = (y & 0x55) | (x & 0xAA);
    (SBOX[uf(s1)], SBOX[uf(s2)])
}

fn next_state(mut state: u128, mut byte: u8) -> (u128, u8) {
    for block in 0..16 {
        let bit = block * 8;
        let mask = !(0xFF << bit);
        let (a, b) = hash((state >> bit) as u8, byte);
        let a: u128 = a.into();
        state = (state & mask) | (a << bit);
        byte = b;
    }

    (state.rotate_left(8), byte)
}

fn compress(mut x: u128) -> u8 {
    x ^= x >> 8;
    x ^= x >> 16;
    x ^= x >> 32;
    x ^= x >> 64;
    x as u8
}

fn encrypt_single_run(msg: &mut [u8], key: u128, range: impl Iterator<Item = usize>) {
    let mut state = key;
    for i in range {
        let b = msg[i];
        msg[i] ^= compress(state);
        (state, _) = next_state(state, b);
    }
}

#[cfg(test)]
fn decrypt_single_run(msg: &mut [u8], key: u128, range: impl Iterator<Item = usize>) {
    let mut state = key;
    for i in range {
        msg[i] ^= compress(state);
        let b = msg[i];
        (state, _) = next_state(state, b);
    }
}

pub(crate) fn encrypt_internal(bytes: &mut [u8], (key1, key2): Key) {
    encrypt_single_run(bytes, key1, 0..bytes.len());
    encrypt_single_run(bytes, key2, (0..bytes.len()).rev());
}

#[cfg(test)]
pub(crate) fn decrypt_internal(bytes: &mut [u8], (key1, key2): Key) {
    decrypt_single_run(bytes, key2, (0..bytes.len()).rev());
    decrypt_single_run(bytes, key1, 0..bytes.len());
}

pub fn encrypt(num: &mut Vec<u8>, key: Key) {
    to_bijbase256(num);
    encrypt_internal(num, key);
}

#[cfg(test)]
pub fn decrypt(num: &mut Vec<u8>, key: Key) {
    decrypt_internal(num, key);
    from_bijbase256(num);
}
