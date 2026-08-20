use crate::code::*;
use crate::crypto::*;
use crate::num::{Num, Sign};
use crate::state::State;
use crate::utils::*;

use rand::prelude::*;

fn rand_bytes(rng: &mut impl Rng, minlen: usize, prob: f64) -> Vec<u8> {
    let mut res = Vec::with_capacity(100);
    res.extend(rng.random_iter::<u8>().take(minlen));

    while rng.random_bool(prob) {
        res.push(rng.random());
    }

    res
}

fn diffusion_score(mut r: Vec<u8>, key: Key) -> (f64, f64) {
    let mut score1 = 0;
    let mut score2 = 0;
    let r_copy = r.clone();
    encrypt_internal(&mut r, key);
    for i in 0..r.len() {
        for j in 0..8 {
            let mut r_copyt = r_copy.clone();
            r_copyt[i] ^= 1 << j;
            encrypt_internal(&mut r_copyt, key);
            for k in 0..r.len() {
                score1 += (r[k] ^ r_copyt[k]).count_zeros();
            }
            let mut val = r.len() * 8;
            for k in 0..r.len() {
                let idx = r.len() - k - 1;
                let diff = r[idx] ^ r_copyt[idx];

                if diff > 0 {
                    val = k * 8 + usize::try_from(diff.leading_zeros()).unwrap();
                    break;
                }
            }
            score2 += val;
        }
    }
    let tot_bits = (r.len() * 8) as f64;

    let score1 = score1 as f64 / (tot_bits * tot_bits);
    let score2 = score2 as f64 / tot_bits;

    (score1, score2)
}

fn remove_leading_zeros(bytes: &mut Vec<u8>) {
    while bytes.last() == Some(&0) {
        bytes.pop();
    }
}

#[test]
fn encryption_then_decryption_equals_id() {
    let mut rng = rand::rng();
    let minlen = 10;
    for _ in 0..500 {
        let key = rng.random();
        let mut bytes = rand_bytes(&mut rng, minlen, 0.9999);
        remove_leading_zeros(&mut bytes);
        let mut bytes_copy = bytes.clone();
        encrypt(&mut bytes_copy, key);
        assert_ne!(bytes, bytes_copy);
        decrypt(&mut bytes_copy, key);
        remove_leading_zeros(&mut bytes_copy);
        assert_eq!(bytes, bytes_copy);
    }
}

#[test]
fn decryption_then_encryption_equals_id() {
    let mut rng = rand::rng();
    let minlen = 10;
    for _ in 0..500 {
        let key = rng.random();
        let bytes = rand_bytes(&mut rng, minlen, 0.9999);
        let mut bytes_copy = bytes.clone();
        decrypt(&mut bytes_copy, key);
        assert_ne!(bytes, bytes_copy);
        encrypt(&mut bytes_copy, key);
        assert_eq!(bytes, bytes_copy);
    }
}

#[test]
fn internal_encryption_then_decryption_equals_id() {
    let mut rng = rand::rng();
    let minlen = 10;
    for _ in 0..500 {
        let key = rng.random();
        let bytes = rand_bytes(&mut rng, minlen, 0.9999);
        let mut bytes_copy = bytes.clone();
        encrypt_internal(&mut bytes_copy, key);
        assert_ne!(bytes, bytes_copy);
        decrypt_internal(&mut bytes_copy, key);
        assert_eq!(bytes, bytes_copy);
    }
}

#[test]
fn internal_decryption_then_encryption_equals_id() {
    let mut rng = rand::rng();
    let minlen = 10;
    for _ in 0..500 {
        let key = rng.random();
        let bytes = rand_bytes(&mut rng, minlen, 0.9999);
        let mut bytes_copy = bytes.clone();
        decrypt_internal(&mut bytes_copy, key);
        assert_ne!(bytes, bytes_copy);
        encrypt_internal(&mut bytes_copy, key);
        assert_eq!(bytes, bytes_copy);
    }
}

#[test]
fn encryption_diffuses() {
    let mut rng = rand::rng();
    let key = rng.random();
    let minlen = 10;
    let num_iter = 500;
    let mut score1 = 0.;
    let mut score2 = 0.;

    for _ in 0..num_iter {
        let bytes = rand_bytes(&mut rng, minlen, 0.99);
        let (sc1, sc2) = diffusion_score(bytes, key);
        score1 += sc1;
        score2 += sc2;
    }

    let d1 = dbg!(score1 / num_iter as f64 - 0.5).abs();
    let d2 = dbg!(score2 / num_iter as f64 - 1.0).abs();

    assert!(d1 < 0.001);
    assert!(d2 < 0.06);
}

#[test]
fn bijbase256_conversion_works() {
    let mut rng = rand::rng();
    let minlen = 10;
    let num_iter = 5000;

    for _ in 0..num_iter {
        let bytes = rand_bytes(&mut rng, minlen, 0.99);
        let mut bytes_copy = bytes.clone();
        from_bijbase256(&mut bytes_copy);
        to_bijbase256(&mut bytes_copy);
        assert_eq!(bytes, bytes_copy);
    }
}

#[test]
fn num_increment_decrement_works() {
    let mut rng = rand::rng();

    for _ in 0..50000 {
        let mut v = vec![0;2];
        rng.fill(&mut v);
        let mut num = Num::new(rng.random(), v);
        let ncpy = num.clone();
        num.increment();
        num.decrement();
        assert_eq!(num, ncpy);
    }

}

#[test]
fn combine_works() {
    let num1 = Num::new(Sign::Negative, vec![0b1110]);
    let num2 = Num::new(Sign::Positive, vec![0b0101]);
    let expected_result = vec![0b01101011, 0b0111];

    assert_eq!(num1.combine(&num2, 0b11, 2), expected_result);
}

#[test]
fn code_byte_conversion_works() {
    let mut rng = rand::rng();
    let minlen = 10;
    for _ in 0..500 {
        let bytes = rand_bytes(&mut rng, minlen, 0.99);
        let string = bytes_to_code(bytes.clone());
        let bytes2 = code_to_bytes(&string).unwrap();
        assert_eq!(bytes, bytes2);
    }
}

#[test]
fn num_serde_works() {
    let mut rng = rand::rng();

    for _ in 0..500 {
        let num = Num::rand_num(&mut rng, 0.999);
        let bytes = num.to_bytes();
        let num2 = Num::from_bytes(&bytes[..]).unwrap();
        assert_eq!(num, num2);
    }
}

#[test]
fn state_serde_works() {
    let mut rng = rand::rng();

    for _ in 0..500 {
        let state = rng.random::<State>();
        let bytes = state.to_bytes();
        let state2 = State::from_bytes(&bytes[..]).unwrap();
        assert_eq!(state, state2);
    }
}
