use crate::code::InvalidCodeError;
use crate::code::{Result as SResult, Serde};
use crate::utils::from_bijbase256;

use rand::distr::StandardUniform;
use rand::prelude::*;

// bijective base 256 unsigned number
pub(crate) type UNum = Vec<u8>;

fn increment(num: &mut UNum) {
    for i in num.iter_mut() {
        *i = i.wrapping_add(1);
        if *i != 1 {
            return;
        }
    }
    num.push(1);
}

fn decrement(num: &mut UNum) {
    for i in num.iter_mut() {
        *i = i.wrapping_sub(1);
        if *i != 0 {
            return;
        }
    }
    num.pop().expect(
        "num being empty is handled by Num's methods, the only place this function is called.",
    );
}

fn interleave(a: u8, b: u8) -> u16 {
    let mut res = 0;
    let a: u16 = a.into();
    let b: u16 = b.into();
    for i in 0..8 {
        res |= ((a & (1 << i)) << i) | ((b & (1 << i)) << (i + 1));
    }
    res
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

impl Distribution<Sign> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sign {
        if rng.random() {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }
}

impl Sign {
    fn to_byte(self) -> u8 {
        if self == Sign::Negative { 0 } else { 1 }
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Sign::Negative),
            1 => Some(Sign::Positive),
            _ => None,
        }
    }
}

// signed number
// value depends on sign:
//     positive ->  |digits|
//     negative -> -|digits| - 1
// where |digits| denotes the number represented by the UNum `digits`

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Num {
    sign: Sign,
    digits: UNum,
}

fn convert<T>(a: Option<T>) -> SResult<T> {
    a.ok_or(InvalidCodeError)
}

impl Serde for Num {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(self.sign.to_byte());

        for &d in self.digits.iter() {
            buf.push(d);
            if d == 0 {
                buf.push(1);
            }
        }

        buf.extend([0, 0]);
    }

    fn from_bytes_prefix(bytes: &[u8]) -> SResult<(Self, &[u8])> {
        let byte = convert(bytes.get(0).copied())?;
        let bytes = convert(bytes.get(1..))?;

        let sign = convert(Sign::from_byte(byte))?;

        let mut digits = Vec::new();
        let mut i = 0;

        loop {
            let byte = convert(bytes.get(i).copied())?;
            i += 1;

            if byte == 0 {
                let nextbyte = convert(bytes.get(i).copied())?;
                i += 1;

                match nextbyte {
                    0 => break,
                    1 => {}
                    2.. => return Err(InvalidCodeError),
                }
            }

            digits.push(byte);
        }

        Ok((Num { sign, digits }, &bytes[i..]))
    }
}

impl Num {
    pub(crate) fn new(sign: Sign, digits: UNum) -> Self {
        Self { sign, digits }
    }

    pub(crate) fn rand_num(rng: &mut (impl Rng + ?Sized), end_prob: f64) -> Self {
        let mut res = Vec::with_capacity(100);

        while rng.random_bool(1.0 - end_prob) {
            res.push(rng.random());
        }

        Num::new(rng.random(), res)
    }

    pub(crate) fn add(&mut self, d: i8) {
        debug_assert!([0, -1, 1].contains(&d));

        if d == -1 {
            self.decrement();
        } else if d == 1 {
            self.increment();
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.digits.len() + 1
    }

    // bits:
    //    extra | self.sign | other.sign | self.bit0 | other.bit0 | self.bit1 | other.bit1 | ...
    // (little endian bit order, and | denotes concatenation)

    pub(crate) fn combine(&self, other: &Self, extra: u8, extra_bitlen: u32) -> Vec<u8> {
        debug_assert!(extra_bitlen <= 8);

        let mut selfdigs = self.digits.clone();
        let mut otherdigs = other.digits.clone();
        from_bijbase256(&mut selfdigs);
        from_bijbase256(&mut otherdigs);

        let maxlen = selfdigs.len().max(otherdigs.len());
        let mut res = Vec::with_capacity(maxlen * 2 + 2);
        let mut rem = (other.sign.to_byte() << 1) | self.sign.to_byte();
        let mut rem_bitlen = 2;
        rem_bitlen += extra_bitlen;

        if rem_bitlen < 8 {
            rem = (rem << extra_bitlen) | extra;
        } else {
            let t: u16 = (u16::from(rem) << extra_bitlen) | u16::from(extra);
            res.push(t as u8);
            rem = (t >> 8) as u8;
            rem_bitlen -= 8;
        }

        for i in 0..maxlen {
            let selfbyte = selfdigs.get(i).copied().unwrap_or(0);
            let otherbyte = otherdigs.get(i).copied().unwrap_or(0);

            let combined = interleave(selfbyte, otherbyte);

            let combined_low = ((combined as u8) << rem_bitlen) | rem;
            let combined_high = (combined >> 8 - rem_bitlen) as u8;

            res.push(combined_low);
            res.push(combined_high);

            rem = (combined >> 16 - rem_bitlen) as u8;
        }

        if rem > 0 {
            res.push(rem);
        }

        res
    }

    pub(crate) fn increment(&mut self) {
        if self.sign == Sign::Negative {
            if self.digits.is_empty() {
                self.sign = Sign::Positive;
                return;
            }
            decrement(&mut self.digits);
        } else {
            increment(&mut self.digits);
        }
    }

    pub(crate) fn decrement(&mut self) {
        if self.sign == Sign::Positive {
            if self.digits.is_empty() {
                self.sign = Sign::Negative;
                return;
            }
            decrement(&mut self.digits);
        } else {
            increment(&mut self.digits);
        }
    }
}
