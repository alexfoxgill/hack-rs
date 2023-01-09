use std::{path::Path, str::FromStr, collections::HashMap};

use crate::{
    common::{read_lines, Error, Res},
    hack::{
        hackword::HackWord,
        instruction::{Comp, Dest, Jump, Instruction}, io::{SCREEN_MEM_START, KB_MEM_SLOT},
    },
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum MemoryLocation {
    Numeric(u16),
    Variable(String),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Asm {
    LoadAddress(MemoryLocation),
    Label(String),
    Compute { dest: Dest, should_deref: bool, comp: Comp, jump: Jump },
    EmptyLine,
}

pub fn compile_file(file: impl AsRef<Path>) -> Res<Vec<HackWord>> {
    compile(read_lines(file)?)
}

impl FromStr for Asm {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.split_once("//").map(|(a, _)| a).unwrap_or(line).trim();

        Ok(if line.is_empty() {
            Asm::EmptyLine
        } else if let Some(addr) = line.strip_prefix('@') {
            Asm::LoadAddress(if let Ok(num) = addr.trim().parse() {
                MemoryLocation::Numeric(num)
            } else {
                MemoryLocation::Variable(addr.trim().into())
            })
        } else if let Some(label) = line.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            Asm::Label(label.trim().into())
        } else {
            let eq = line.chars().position(|c| c == '=');
            let semi = line.chars().position(|c| c == ';');

            let (dest_str, comp, jump) = match (eq, semi) {
                (None, None) => (None, line, None),
                (None, Some(semi)) => (None, &line[..semi], Some(&line[semi+1..])),
                (Some(eq), None) => (Some(&line[..eq]), &line[eq+1..], None),
                (Some(eq), Some(semi)) => (Some(&line[..eq]), &line[eq+1..semi], Some(&line[semi+1..])),
            };

            let mut dest = Dest::default();
            for c in dest_str.iter().flat_map(|d| d.chars()) {
                match c {
                    'A' => dest.a = true,
                    'D' => dest.d = true,
                    'M' => dest.m = true,
                    _ => return Err(format!("Unrecognised destination '{c}'").into())
                }
            }

            let mut should_deref = false;
            let comp = match comp {
                "0" => Comp::Zero,
                "1" => Comp::One,
                "-1" => Comp::MinusOne,
                "D" => Comp::D,
                "A" => Comp::A,
                "M" => {
                    should_deref = true;
                    Comp::A
                },
                "!D" => Comp::NotD,
                "!A" => Comp::NotA,
                "!M" => {
                    should_deref = true;
                    Comp::NotA
                },
                "-D" => Comp::MinusD,
                "-A" => Comp::MinusA,
                "-M" => {
                    should_deref = true;
                    Comp::MinusA
                },
                "D+1" => Comp::DPlus1,
                "A+1" => Comp::APlus1,
                "M+1" => {
                    should_deref = true;
                    Comp::APlus1
                }
                "D-1" => Comp::DMinus1,
                "A-1" => Comp::AMinus1,
                "M-1" => {
                    should_deref = true;
                    Comp::AMinus1
                },
                "D+A" => Comp::DPlusA,
                "D+M" => {
                    should_deref = true;
                    Comp::DPlusA
                }
                "D-A" => Comp::DMinusA,
                "D-M" => {
                    should_deref = true;
                    Comp::DMinusA
                }
                "A-D" => Comp::AMinusD,
                "M-D" => {
                    should_deref = true;
                    Comp::AMinusD
                }
                "D&A" => Comp::DAndA,
                "D&M" => {
                    should_deref = true;
                    Comp::DAndA
                }
                "D|A" => Comp::DOrA,
                "D|M" => {
                    should_deref = true;
                    Comp::DOrA
                }
                _ => return Err(format!("Unrecognised comp '{comp}'").into())
            };

            let jump = match jump {
                None => Jump::Null,
                Some("JGT") => Jump::JGT,
                Some("JEQ") => Jump::JEQ,
                Some("JGE") => Jump::JGE,
                Some("JLT") => Jump::JLT,
                Some("JNE") => Jump::JNE,
                Some("JLE") => Jump::JLE,
                Some("JMP") => Jump::JMP,
                Some(x) => return Err(format!("Unrecognised jump '{x}'").into())
            };

            Asm::Compute { dest, should_deref, comp, jump }
        })
    }
}

pub fn compile<'a>(asm_lines: Vec<String>) -> Res<Vec<HackWord>> {
    let mut memory = HashMap::from([
        ("R0".into(), 0),
        ("R1".into(), 1),
        ("R2".into(), 2),
        ("R3".into(), 3),
        ("R4".into(), 4),
        ("R5".into(), 5),
        ("R6".into(), 6),
        ("R7".into(), 7),
        ("R8".into(), 8),
        ("R9".into(), 9),
        ("R10".into(), 10),
        ("R11".into(), 11),
        ("R12".into(), 12),
        ("R13".into(), 13),
        ("R14".into(), 14),
        ("R15".into(), 15),
        ("SP".into(), 0),
        ("LCL".into(), 1),
        ("ARG".into(), 2),
        ("THIS".into(), 3),
        ("THAT".into(), 4),
        ("SCREEN".into(), SCREEN_MEM_START),
        ("KBD".into(), KB_MEM_SLOT)
    ]);

    let mut hashwords = Vec::new();

    let mut ram = 16;
    for line in asm_lines {
        let asm: Asm = line.parse()?;
        match asm {
            Asm::LoadAddress(MemoryLocation::Numeric(n)) => {
                hashwords.push(Instruction::A(n).into());
            },
            Asm::LoadAddress(MemoryLocation::Variable(v)) => {
                let &mut n = memory.entry(v).or_insert_with(|| {
                    let r = ram;
                    ram += 1;
                    r
                });
                hashwords.push(Instruction::A(n).into());
            }
            Asm::Label(l) => {
                memory.insert(l, hashwords.len() as u16);
            },
            Asm::Compute { dest, should_deref, comp, jump } => {
                hashwords.push(Instruction::C {
                    dest,
                    should_deref,
                    comp,
                    jump
                }.into())
            },
            Asm::EmptyLine => (),
        }
    }

    Ok(hashwords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_asm() {
        for (input, expected) in [
            ("@ test", Asm::LoadAddress(MemoryLocation::Variable("test".into()))),
            ("@ 98", Asm::LoadAddress(MemoryLocation::Numeric(98))),
            ("// a comment", Asm::EmptyLine),
            ("( LOOP )", Asm::Label("LOOP".into())),
            ("M=1", Asm::Compute { dest: Dest { a: false, d: false, m: true }, should_deref: false, comp: Comp::One, jump: Jump::Null }),
            ("D;JGT", Asm::Compute { dest: Dest::default(), should_deref: false, comp: Comp::D, jump: Jump::JGT})
        ] {
            let res: Asm = input.parse().unwrap();

            assert_eq!(res, expected);
        }
    }
}
