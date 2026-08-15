use crate::crypto::{encrypt, decrypt};
use crate::crypto::Key;
use crate::num::Num;
use crate::utils::textbox;

use rand::{Rng, RngExt};
use rand::distr::{Distribution, StandardUniform};


pub struct HitWallError;

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

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn from_code(code: String) -> Self {
        todo!()
    }

    pub fn to_code(&self) -> String {
        todo!()
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

