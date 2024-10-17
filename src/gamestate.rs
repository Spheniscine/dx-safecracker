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
    pub digits: usize,
    pub range: RangeInclusive<u8>,
    pub secret: Code,
    pub spin: Code,
    pub spins: i32,
    pub history: Vec<Code>,
    pub state_kind: StateKind,
}

impl GameState {
    pub fn new(digits: usize, range: RangeInclusive<u8>) -> Self {
        let secret = Code::random(digits, range.clone());
        let mut r = Self {
            digits,
            range: range.clone(),
            secret,
            spin: Code(vec![]),
            spins: 0,
            history: vec![],
            state_kind: StateKind::GuessValue
        };
        r.spin();
        r
    }
    pub fn spin(&mut self) {
        self.spin = Code::random(self.digits, self.range.clone());
        if self.current_value() == 0 {
            self.history.push(self.spin.clone());
            self.state_kind = StateKind::GuessCode;
        } else {
            self.state_kind = StateKind::GuessValue;
        }
    }
    pub fn code_value(&self, code: &Code) -> i32 {
        code.0.iter().zip(&self.secret.0).map(|(&a, &b)| if a == b {a as i32} else {0}).sum()
    }
    pub fn current_value(&self) -> i32 {
        self.code_value(&self.spin)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateKind {
    GuessValue, GuessCode, Won
}