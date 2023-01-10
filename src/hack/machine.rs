use crate::common::*;
use crate::hackword::*;
use crate::instruction::*;

pub const MEMORY_SIZE: usize = 32768;

pub struct Machine {
    instructions: Vec<HackWord>,
    current_instruction: HackWord,
    pub memory: [HackWord; MEMORY_SIZE],
    register_a: HackWord,
    register_d: HackWord,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            current_instruction: HackWord::default(),
            memory: [HackWord::default(); MEMORY_SIZE],
            register_a: HackWord::default(),
            register_d: HackWord::default(),
        }
    }

    fn set_instruction(&mut self, instruction: HackWord) {
        self.current_instruction = instruction;
    }

    fn m(&self) -> HackWord {
        self.memory[self.register_a.to_usize()]
    }

    fn m_mut(&mut self) -> &mut HackWord {
        &mut self.memory[self.register_a.to_usize()]
    }

    fn ins(&self) -> Res<Option<Instruction>> {
        Ok({
            let i = self.current_instruction.to_usize();
            if i < self.instructions.len() {
                Some(self.instructions[i].try_into()?)
            } else {
                None
            }
        })
    }

    fn step_ins(&mut self, current: Instruction) {
        match current {
            Instruction::A(dest) => {
                self.register_a = HackWord(dest as i16);
                self.set_instruction(self.current_instruction + HackWord::one());
            }
            Instruction::C {
                should_deref,
                comp,
                dest,
                jump,
            } => {
                let value = {
                    let d = self.register_d;
                    let a = if should_deref {
                        self.m()
                    } else {
                        self.register_a
                    };
                    match comp {
                        Comp::Zero => HackWord::zero(),
                        Comp::One => HackWord::one(),
                        Comp::MinusOne => HackWord::minus_one(),
                        Comp::D => d,
                        Comp::A => a,
                        Comp::NotD => !d,
                        Comp::NotA => !a,
                        Comp::MinusD => -d,
                        Comp::MinusA => -a,
                        Comp::DPlus1 => d + HackWord::one(),
                        Comp::APlus1 => a + HackWord::one(),
                        Comp::DMinus1 => d - HackWord::one(),
                        Comp::AMinus1 => a - HackWord::one(),
                        Comp::DPlusA => d + a,
                        Comp::DMinusA => d - a,
                        Comp::AMinusD => a - d,
                        Comp::DAndA => d & a,
                        Comp::DOrA => d | a,
                    }
                };

                if dest.a {
                    self.register_a = value;
                }
                if dest.d {
                    self.register_d = value;
                }
                if dest.m {
                    *self.m_mut() = value;
                }

                self.set_instruction(if jump.should_jump(value) {
                    self.register_a
                } else {
                    self.current_instruction + HackWord::one()
                })
            }
        }
    }

    pub fn step(&mut self) -> Res<bool> {
        Ok(if let Some(current) = self.ins()? {
            self.step_ins(current);
            true
        } else {
            false
        })
    }

    pub fn run(&mut self) -> Res {
        while self.step()? {}
        Ok(())
    }

    pub fn load_instructions(&mut self, instructions: Vec<HackWord>) {
        self.instructions = instructions;
    }
}
