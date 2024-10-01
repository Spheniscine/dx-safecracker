use std::ops::RangeInclusive;

pub const MIN_DIGITS: usize = 5;
pub const MAX_DIGITS: usize = 10;

pub const RANGES: &[(&str, RangeInclusive<u8>)] = [
    ("1-6", 1..=6),
    ("1-4", 1..=4),
    ("1-9", 1..=9),
].as_slice();