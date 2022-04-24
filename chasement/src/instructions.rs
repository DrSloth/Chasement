mod base;

use std::collections::HashMap;

use crate::{Context, Opcode, ProgramStorage};

pub type Instruction<P> = fn(&mut Context<P>);
pub type InstructionSetInner<P> =
    HashMap<Opcode, Instruction<P>, nohash::BuildNoHashHasher<Opcode>>;

#[derive(Clone)]
pub struct InstructionSet<P: ProgramStorage>(InstructionSetInner<P>);

impl<P: ProgramStorage> InstructionSet<P> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn new_with<F: FnOnce(&mut Self)>(add_instructions: F) -> Self {
        let mut me = Self::new();
        add_instructions(&mut me);
        me
    }

    pub fn inner_mut(&mut self) -> &mut InstructionSetInner<P> {
        &mut self.0
    }

    pub fn insert(&mut self, opcode: u8, instruction: Instruction<P>) {
        self.0.insert(opcode, instruction);
    }

    pub fn get(&self, opcode:& u8) -> Option<Instruction<P>> {
        self.0.get(opcode).copied()
    }

    pub fn with_base_instructions(&mut self) -> &mut Self {
        base::add_base_instructions(self);
        self
    }

    pub fn with_arithmetic_instructions(&mut self) -> &mut Self {
        self
    }
}

pub fn error(err: &str) -> ! {
    eprintln!("ERROR: {}", err);
    std::process::exit(1)
}
