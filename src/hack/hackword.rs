use core::fmt;
use std::ops::{Add, BitAnd, BitOr, Neg, Sub};
use std::str::FromStr;
use std::{error::Error, ops::Not};

use crate::common::err;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct HackWord(pub i16);

impl HackWord {
    pub fn bit(&self, b: u8) -> bool {
        let offset = 15 - b;
        (self.0 & (1 << offset)) != 0
    }

    pub fn to_usize(&self) -> usize {
        (self.0 as u16) as usize
    }

    #[inline]
    pub fn one() -> HackWord {
        HackWord(1)
    }

    #[inline]
    pub fn zero() -> HackWord {
        HackWord(0)
    }

    #[inline]
    pub fn minus_one() -> HackWord {
        HackWord(-1)
    }
}

impl fmt::Debug for HackWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

impl Not for HackWord {
    type Output = HackWord;

    fn not(self) -> Self::Output {
        HackWord(!self.0)
    }
}

impl Add for HackWord {
    type Output = HackWord;

    fn add(self, rhs: Self) -> Self::Output {
        HackWord(self.0 + rhs.0)
    }
}

impl Sub for HackWord {
    type Output = HackWord;

    fn sub(self, rhs: Self) -> Self::Output {
        HackWord(self.0 - rhs.0)
    }
}

impl Neg for HackWord {
    type Output = HackWord;

    fn neg(self) -> Self::Output {
        HackWord(-self.0)
    }
}

impl BitAnd for HackWord {
    type Output = HackWord;

    fn bitand(self, rhs: Self) -> Self::Output {
        HackWord(self.0 & rhs.0)
    }
}

impl BitOr for HackWord {
    type Output = HackWord;

    fn bitor(self, rhs: Self) -> Self::Output {
        HackWord(self.0 | rhs.0)
    }
}

impl FromStr for HackWord {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok({
            let u16 = u16::from_str_radix(s, 2).map_err(|_| err("Not a 16-digit binary number"))?;
            HackWord(u16 as i16)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitstring_to_hackword_and_back() {
        for (input, expected) in [
            ("0110000000000000", 0b0110000000000000),
            ("1111110000010000", i16::MIN | 0b0111110000010000),
        ] {
            let word: HackWord = input.parse().expect("Expected parse");

            assert_eq!(word.0, expected);

            let back = format!("{word:?}");

            assert_eq!(back, input);
        }
    }

    #[test]
    fn signed_to_usize() {
        for (input, expected) in [
            (i16::MIN | 0b0111_1111_1111_1111, 0b1111_1111_1111_1111),
            (0, 0),
            (1, 1),
        ] {
            let word = HackWord(input);

            let res = word.to_usize();

            assert_eq!(res, expected)
        }
    }

    #[test]
    fn bits() {
        for (input, bit) in [
            (i16::MIN, 0),
            (0b0100_0000_0000_0000, 1),
            (0b0000_0000_0000_0001, 15),
        ] {
            let word = HackWord(input);

            assert!(word.bit(bit))
        }
    }
}
