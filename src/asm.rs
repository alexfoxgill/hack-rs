use std::{collections::HashMap, path::Path, str::FromStr};

use crate::{
    common::{read_lines, Error, Res},
    hack::{
        hackword::HackWord,
        instruction::{Comp, Dest, Instruction, Jump},
        io::{KB_MEM_SLOT, SCREEN_MEM_START},
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
    Compute {
        dest: Dest,
        should_deref: bool,
        comp: Comp,
        jump: Jump,
    },
    EmptyLine,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct AsmLine {
    instruction: Asm,
    comment: Option<String>,
}

pub fn compile_file(file: impl AsRef<Path>, debug: bool) -> Res<(Vec<HackWord>, Option<AsmDebug>)> {
    compile(read_lines(file)?, debug)
}

impl FromStr for AsmLine {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (line, comment) = line.split_once("//").unwrap_or((line, ""));
        let line = line.trim();

        Ok(AsmLine {
            comment: if comment == "" {
                None
            } else {
                Some(comment.into())
            },
            instruction: if line.is_empty() {
                Asm::EmptyLine
            } else if let Some(addr) = line.strip_prefix('@') {
                Asm::LoadAddress(if let Ok(num) = addr.trim().parse() {
                    if num > 32767 {
                        return Err(format!(
                            "Constant number '{num}' cannot be greater than 32767"
                        )
                        .into());
                    }
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
                    (None, Some(semi)) => (None, &line[..semi], Some(&line[semi + 1..])),
                    (Some(eq), None) => (Some(&line[..eq]), &line[eq + 1..], None),
                    (Some(eq), Some(semi)) => (
                        Some(&line[..eq]),
                        &line[eq + 1..semi],
                        Some(&line[semi + 1..]),
                    ),
                };

                let mut dest = Dest::default();
                for c in dest_str.iter().flat_map(|d| d.chars()) {
                    match c {
                        'A' => dest.a = true,
                        'D' => dest.d = true,
                        'M' => dest.m = true,
                        _ => return Err(format!("Unrecognised destination '{c}'").into()),
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
                    }
                    "!D" => Comp::NotD,
                    "!A" => Comp::NotA,
                    "!M" => {
                        should_deref = true;
                        Comp::NotA
                    }
                    "-D" => Comp::MinusD,
                    "-A" => Comp::MinusA,
                    "-M" => {
                        should_deref = true;
                        Comp::MinusA
                    }
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
                    }
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
                    _ => return Err(format!("Unrecognised comp '{comp}'").into()),
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
                    Some(x) => return Err(format!("Unrecognised jump '{x}'").into()),
                };

                Asm::Compute {
                    dest,
                    should_deref,
                    comp,
                    jump,
                }
            },
        })
    }
}

pub fn compile_lines(str: &str) -> Res<Vec<HackWord>> {
    Ok(compile(str.lines().map(|x| x.into()).collect(), false)?.0)
}

pub fn compile(asm_lines: Vec<String>, debug: bool) -> Res<(Vec<HackWord>, Option<AsmDebug>)> {
    let mut symbols = HashMap::from([
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
        ("KBD".into(), KB_MEM_SLOT),
    ]);

    let mut hashwords = Vec::new();
    let mut line_mappings = HashMap::new();

    let parsed = {
        let parsed: Res<Vec<AsmLine>> = asm_lines.iter().map(|line| line.parse()).collect();
        parsed?
    };

    // first pass: load labels into memory
    let mut i = 0;
    for asm in &parsed {
        match &asm.instruction {
            Asm::Label(l) => {
                symbols.insert(l.into(), i);
            }
            Asm::EmptyLine => (),
            _ => {
                i += 1;
            }
        }
    }

    let mut ram = 16;
    for (i, (line, asm)) in asm_lines.into_iter().zip(parsed).enumerate() {
        match asm.instruction {
            Asm::LoadAddress(MemoryLocation::Numeric(n)) => {
                if debug {
                    line_mappings.insert(hashwords.len(), (i + 1, line));
                }
                hashwords.push(Instruction::A(n).into());
            }
            Asm::LoadAddress(MemoryLocation::Variable(v)) => {
                if debug {
                    line_mappings.insert(hashwords.len(), (i + 1, line));
                }
                let &mut n = symbols.entry(v).or_insert_with(|| {
                    let r = ram;
                    ram += 1;
                    r
                });
                hashwords.push(Instruction::A(n).into());
            }
            Asm::Compute {
                dest,
                should_deref,
                comp,
                jump,
            } => {
                if debug {
                    line_mappings.insert(hashwords.len(), (i + 1, line));
                }
                hashwords.push(
                    Instruction::C {
                        dest,
                        should_deref,
                        comp,
                        jump,
                    }
                    .into(),
                )
            }
            Asm::Label(_) | Asm::EmptyLine => (),
        }
    }

    let debug_info = if debug {
        Some(AsmDebug {
            symbols,
            line_mappings,
        })
    } else {
        None
    };

    Ok((hashwords, debug_info))
}

pub struct AsmDebug {
    pub symbols: HashMap<String, u16>,
    pub line_mappings: HashMap<usize, (usize, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_asm() {
        for (input, expected) in [
            (
                "@ test",
                Asm::LoadAddress(MemoryLocation::Variable("test".into())),
            ),
            ("@ 98", Asm::LoadAddress(MemoryLocation::Numeric(98))),
            ("// a comment", Asm::EmptyLine),
            ("( LOOP )", Asm::Label("LOOP".into())),
            (
                "M=1",
                Asm::Compute {
                    dest: Dest {
                        a: false,
                        d: false,
                        m: true,
                    },
                    should_deref: false,
                    comp: Comp::One,
                    jump: Jump::Null,
                },
            ),
            (
                "D;JGT",
                Asm::Compute {
                    dest: Dest::default(),
                    should_deref: false,
                    comp: Comp::D,
                    jump: Jump::JGT,
                },
            ),
        ] {
            let res: AsmLine = input.parse().unwrap();

            assert_eq!(res.instruction, expected);
        }
    }

    use crate::hack::{hackword::HackWord, machine::Machine};

    #[test]
    fn test_mult() {
        for (a, b) in [(1, 1), (1, 2), (2, 1), (2, 2), (0, 2), (2, 0)] {
            let mut machine = Machine::new();
            machine.memory[0] = HackWord(a);
            machine.memory[1] = HackWord(b);

            let (instructions, _) = compile_file("resources/mult.asm", false).unwrap();
            machine.load_instructions(instructions);

            machine.run().unwrap();

            assert_eq!(machine.memory[2], HackWord(a * b))
        }
    }

    #[test]
    fn example_compilation() {
        let asm = r#"
            // Adds 1 + ... + 100
                @i
                M=1    // i=1
                @sum
                M=0    // sum=0
            (LOOP)
                @i
                D=M    // D=i
                @100
                D=D-A  // D=i-100
                @END
                D;JGT  // if (i-100)>0 goto END
                @i
                D=M    // D=i
                @sum
                M=D+M  // sum=sum+i
                @i
                M=M+1  // i=i+1
                @LOOP
                0;JMP  // goto LOOP
            (END)
                @END
                0;JMP  // infinite loop
        "#;

        let expected: Vec<HackWord> = vec![
            "0000000000010000",
            "1110111111001000",
            "0000000000010001",
            "1110101010001000",
            "0000000000010000",
            "1111110000010000",
            "0000000001100100",
            "1110010011010000",
            "0000000000010010",
            "1110001100000001",
            "0000000000010000",
            "1111110000010000",
            "0000000000010001",
            "1111000010001000",
            "0000000000010000",
            "1111110111001000",
            "0000000000000100",
            "1110101010000111",
            "0000000000010010",
            "1110101010000111"
        ].iter().map(|x| x.parse().unwrap()).collect();

        let res = compile_lines(asm).unwrap();

        assert_eq!(res, expected);
    }
}
