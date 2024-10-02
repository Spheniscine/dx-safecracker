use std::{fmt::{Debug, Display}, ops::RangeInclusive};

use rand::{rngs::ThreadRng, Rng};


#[derive(Clone)]
pub struct Code(Vec<u8>);

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

impl Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Code {
    pub fn random(digits: usize, range: RangeInclusive<u8>) -> Code {
        let mut rng = ThreadRng::default();
        Code((0..digits).map(|_| rng.gen_range(range.clone())).collect())
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    digits: usize,
    range: RangeInclusive<u8>,
    secret: Code,
    spin: Code
}

impl GameState {
    pub fn new(digits: usize, range: RangeInclusive<u8>) -> Self {
        Self {
            digits,
            range: range.clone(),
            secret: Code::random(digits, range.clone()),
            spin: Code::random(digits, range.clone()),
        }
    }
}