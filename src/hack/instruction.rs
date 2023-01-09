use std::error::Error;

use crate::common::{err, Res};
use crate::hackword::HackWord;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlus1,
    APlus1,
    DMinus1,
    AMinus1,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct Dest {
    pub a: bool,
    pub d: bool,
    pub m: bool,
}


#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum Jump {
    #[default] Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl Jump {
    pub fn should_jump(&self, value: HackWord) -> bool {
        match self {
            Jump::Null => false,
            Jump::JGT => value.0 > 0,
            Jump::JEQ => value.0 == 0,
            Jump::JGE => value.0 >= 0,
            Jump::JLT => value.0 < 0,
            Jump::JNE => value.0 != 0,
            Jump::JLE => value.0 <= 0,
            Jump::JMP => true,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Instruction {
    A(u16),
    C {
        comp: Comp,
        should_deref: bool,
        dest: Dest,
        jump: Jump,
    },
}

impl TryFrom<HackWord> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(word: HackWord) -> Res<Self> {
        Ok(if !word.bit(0) {
            Instruction::A(word.0 as u16)
        } else {
            let should_deref = word.bit(3);
            let comp = match (word.0 & (0b111111 << 6)) >> 6 {
                0b101010 => Comp::Zero,
                0b111111 => Comp::One,
                0b111010 => Comp::MinusOne,
                0b001100 => Comp::D,
                0b110000 => Comp::A,
                0b001101 => Comp::NotD,
                0b110001 => Comp::NotA,
                0b001111 => Comp::MinusD,
                0b110011 => Comp::MinusA,
                0b011111 => Comp::DPlus1,
                0b110111 => Comp::APlus1,
                0b001110 => Comp::DMinus1,
                0b110010 => Comp::AMinus1,
                0b000010 => Comp::DPlusA,
                0b010011 => Comp::DMinusA,
                0b000111 => Comp::AMinusD,
                0b000000 => Comp::DAndA,
                0b010101 => Comp::DOrA,
                _ => return Err(err("Unrecognised comp instruction")),
            };
            let dest = Dest {
                a: word.bit(10),
                d: word.bit(11),
                m: word.bit(12),
            };
            let jump = match word.0 & 0b111 {
                0b000 => Jump::Null,
                0b001 => Jump::JGT,
                0b010 => Jump::JEQ,
                0b011 => Jump::JGE,
                0b100 => Jump::JLT,
                0b101 => Jump::JNE,
                0b110 => Jump::JLE,
                0b111 => Jump::JMP,
                _ => panic!("Impossible jump"),
            };
            Instruction::C {
                should_deref,
                comp,
                dest,
                jump,
            }
        })
    }
}

impl From<Instruction> for HackWord {
    fn from(i: Instruction) -> Self {
        match i {
            Instruction::A(x) => HackWord(x as i16),
            Instruction::C {
                comp,
                should_deref,
                dest,
                jump,
            } => {
                let mut word = i16::MIN | 0b0110_0000_0000_0000;
                if should_deref {
                    word |= 0b0001_0000_0000_0000;
                }
                word |= match comp {
                    Comp::Zero => 0b101010,
                    Comp::One => 0b111111,
                    Comp::MinusOne => 0b111010,
                    Comp::D => 0b001100,
                    Comp::A => 0b110000,
                    Comp::NotD => 0b001101,
                    Comp::NotA => 0b110001,
                    Comp::MinusD => 0b001111,
                    Comp::MinusA => 0b110011,
                    Comp::DPlus1 => 0b011111,
                    Comp::APlus1 => 0b110111,
                    Comp::DMinus1 => 0b001110,
                    Comp::AMinus1 => 0b110010,
                    Comp::DPlusA => 0b000010,
                    Comp::DMinusA => 0b010011,
                    Comp::AMinusD => 0b000111,
                    Comp::DAndA => 0b000000,
                    Comp::DOrA => 0b010101,
                } << 6;
                if dest.a {
                    word |= 0b0000_0000_0010_0000;
                }
                if dest.d {
                    word |= 0b0000_0000_0001_0000;
                }
                if dest.m {
                    word |= 0b0000_0000_0000_1000;
                }
                word |= match jump {
                    Jump::Null => 0b000,
                    Jump::JGT => 0b001,
                    Jump::JEQ => 0b010,
                    Jump::JGE => 0b011,
                    Jump::JLT => 0b100,
                    Jump::JNE => 0b101,
                    Jump::JLE => 0b110,
                    Jump::JMP => 0b111,
                };
                HackWord(word)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hackword_to_instruction_and_back() {
        for (input, expected) in [
            (
                i16::MIN | 0b0111_1111_1111_1111,
                Instruction::C {
                    should_deref: true,
                    comp: Comp::One,
                    dest: Dest {
                        a: true,
                        d: true,
                        m: true,
                    },
                    jump: Jump::JMP,
                },
            ),
            (
                i16::MIN | 0b0110_0000_0000_0000,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: false,
                        d: false,
                        m: false,
                    },
                    jump: Jump::Null,
                },
            ),
            (
                i16::MIN | 0b0111_0000_0000_0000,
                Instruction::C {
                    should_deref: true,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: false,
                        d: false,
                        m: false,
                    },
                    jump: Jump::Null,
                },
            ),
            (
                i16::MIN | 0b0110_1100_0100_0000,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::NotA,
                    dest: Dest {
                        a: false,
                        d: false,
                        m: false,
                    },
                    jump: Jump::Null,
                },
            ),
            (
                i16::MIN | 0b0110_0000_0010_0000,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: true,
                        d: false,
                        m: false,
                    },
                    jump: Jump::Null,
                },
            ),
            (
                i16::MIN | 0b0110_0000_0001_0000,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: false,
                        d: true,
                        m: false,
                    },
                    jump: Jump::Null,
                },
            ),
            (
                i16::MIN | 0b0110_0000_0000_1000,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: false,
                        d: false,
                        m: true,
                    },
                    jump: Jump::Null,
                },
            ),
            (
                i16::MIN | 0b0110_0000_0000_0100,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: false,
                        d: false,
                        m: false,
                    },
                    jump: Jump::JLT,
                },
            ),
            (
                i16::MIN | 0b0110_0000_0000_0010,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: false,
                        d: false,
                        m: false,
                    },
                    jump: Jump::JEQ,
                },
            ),
            (
                i16::MIN | 0b0110_0000_0000_0001,
                Instruction::C {
                    should_deref: false,
                    comp: Comp::DAndA,
                    dest: Dest {
                        a: false,
                        d: false,
                        m: false,
                    },
                    jump: Jump::JGT,
                },
            ),
        ] {
            let word = HackWord(input);

            let ins: Instruction = word.try_into().expect("Expect conversion");

            assert_eq!(ins, expected);

            let back: HackWord = ins.into();

            assert_eq!(back, word)
        }
    }
}
