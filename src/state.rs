use crate::crypto::encrypt;
use crate::crypto::Key;
use crate::num::Num;
use crate::utils::{textbox, self};

use ciborium::{from_reader_with_buffer, into_writer};
use rand::{Rng, RngExt};
use rand::distr::{Distribution, StandardUniform};
use serde::{Serialize, Deserialize};

use std::io::{Cursor, Error};

pub struct HitWallError;

#[derive(Debug)]
pub struct InvalidCodeError;

// probability of the Nums in State ending at each step of the generation process
// so that the probability of the number being L (base 256) digits long is 
//       (1-END_PROB)^L

const END_PROB: f64 = 0.001;

// directions:
//        0
//    7*      1*
// 6             2
//    5*      3*
//        4
//
// * means there's a wall of text there, instead of a door

// (x,y) deltas for direction given by <index>*2
const DELTAS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const CHARMAP: [char; 256] = include!("chars.txt");

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct State {
    positionx: Num,
    positiony: Num,
    key: Key,
    direction: u8,
}

impl Distribution<State> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        State {
            positionx: Num::rand_num(rng, END_PROB),
            positiony: Num::rand_num(rng, END_PROB),
            key: rng.random(),
            direction: rng.random::<u8>()&7,
        }
    }
}

impl State {
    pub fn from_code(code: &str) -> Result<Self, InvalidCodeError> {
        let bytes = utils::from_code(code)
                          .map_err(|_| InvalidCodeError)?;
        from_reader_with_buffer(&bytes[..], &mut vec![0; 65536])
            .map_err(|_| InvalidCodeError)
    }

    pub fn to_code(&self) -> String {
        let mut bytes = Vec::new();
        into_writer(self, Cursor::new(&mut bytes));
        utils::to_code(bytes)
    }
}

impl State {
    pub fn describe(&self) -> String {
        if self.door_in_front() {
            self.describe_door()
        } else {
            self.describe_wall()
        }
    }

    fn describe_door(&self) -> String {
        String::from("A dark, rusty door stands before you.")
    }

    fn describe_wall(&self) -> String {
        debug_assert!(!self.door_in_front());
        let mut bytes = self.positionx.combine(&self.positiony, self.direction/2, 2);

        encrypt(&mut bytes, self.key);

        let mut res = String::new();

        for byte in bytes {
            res.push(CHARMAP[usize::from(byte)]);
        }


        format!("A wall of text stands before you. It reads:\n{}",
                textbox(res))
    }

    pub fn move_forward(&mut self) -> Result<(), HitWallError> {
        if !self.door_in_front() {
            return Err(HitWallError);
        }
        let (dx, dy) = DELTAS[self.direction as usize];
        self.positionx.add(dx);
        self.positiony.add(dy);
        Ok(())
    }

    pub fn door_in_front(&self) -> bool {
        self.direction & 1 == 0
    }

    pub fn turn_left(&mut self) {
        self.direction += 7;
        self.direction &= 7;
    }

    pub fn turn_right(&mut self) {
        self.direction += 1;
        self.direction &= 7;
    }
}

