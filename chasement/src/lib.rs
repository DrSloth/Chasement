pub mod instructions;

pub use instructions::InstructionSet;

use std::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
};

use instructions::Instruction;

pub type Opcode = u8;

#[derive(Clone)]
pub struct Vm<'a, P: ProgramStorage> {
    /// All available instructions, indexed by the ascii value of its responding char.
    /// Will be changed to a const array later.
    instructions: InstructionSet<P>,
    ctx: Context<'a, P>,
}

impl<'a, P: ProgramStorage> Vm<'a, P> {
    pub fn new(instructions: InstructionSet<P>, data: P) -> Self {
        Self {
            instructions,
            ctx: Context::new(data),
        }
    }

    pub fn with_program(self, program: P) -> Self {
        Vm {
            instructions: self.instructions,
            ctx: self.ctx.with_program(program),
        }
    }

    pub fn run(&mut self) {
        while let Some(opcode) = self.ctx.program.opcode_at(self.ctx.pc) {
            self.run_op(&opcode)
        }
    }

    pub fn run_op(&mut self, opcode: &u8) {
        let instruction = self.instructions.get(opcode).unwrap_or_else(|| {
            panic!(
                "No instruction for {:?} at {}",
                *opcode as char, self.ctx.pc
            )
        });
        self.run_instruction(instruction);
        //Use wrapping_add here because of jumps semantics
        self.ctx.pc = self.ctx.pc.wrapping_add(1);
    }

    #[inline(always)]
    pub fn run_instruction(&mut self, instruction: Instruction<P>) {
        instruction(&mut self.ctx);
    }

    pub fn get_context(&self) -> &Context<P> {
        &self.ctx
    }

    pub fn get_context_mut(&mut self) -> &mut Context<'a, P> {
        &mut self.ctx
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Int(i64),
    Bool(bool),
    Char(char),
    Str(String),
    //Add float support later (. is occupied for that)
    Float(f64),
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Char(c) => write!(f, "{}", c),
            Self::Str(s) => write!(f, "{}", s),
            Self::Float(fl) => write!(f, "{}", fl),
        }
    }
}

/// A mutable Context for a program
#[derive(Clone, Debug)]
pub struct Context<'a, P: ProgramStorage + 'a> {
    /// Value Stack
    stack: Vec<Data>,
    /// Auxiliary stack
    auxiliary_stack: Vec<Data>,
    /// Program counter (current instruction)
    pc: usize,
    program: P,
    phantom: PhantomData<&'a mut P>,
}

impl<'a, P: ProgramStorage> Context<'a, P> {
    /// Create a Context with a program
    pub fn new(program: P) -> Self {
        Context {
            program,
            stack: Default::default(),
            auxiliary_stack: Default::default(),
            pc: 0,
            phantom: Default::default(),
        }
    }

    pub fn with_program<'b, P2: ProgramStorage>(self, program: P2) -> Context<'b, P2> {
        Context {
            program,
            stack: self.stack,
            auxiliary_stack: self.auxiliary_stack,
            pc: self.pc,
            phantom: Default::default(),
        }
    }

    /// Pop a value of the data stack
    pub fn pop(&mut self) -> Option<Data> {
        self.stack.pop()
    }

    /// Get a reference to the top value of the Data
    pub fn top(&self) -> Option<&Data> {
        self.stack.last()
    }

    /// Push a value to the data stack
    pub fn push(&mut self, data: Data) {
        self.stack.push(data)
    }

    /// Get the program counter (current instruction)
    pub fn get_pc(&self) -> usize {
        self.pc
    }

    /// Set the program counter (current instruction)
    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc
    }

    pub fn advance(&mut self) {
        //self.pc = self.pc.wrapping_add(1);
        self.pc += 1;
    }

    pub fn prev(&mut self) {
        //self.pc = self.pc.wrapping_sub(1);
        self.pc = self.pc.wrapping_sub(1);
    }

    pub fn cur_byte(&self) -> Option<u8> {
        self.program.opcode_at(self.pc)
    }

    /// Pop a value of the main stack onto the auxiliary stack
    pub fn to_auxiliary(&mut self) {
        if let Some(val) = self.pop() {
            self.auxiliary_stack.push(val)
        }
    }

    /// Pop a value of the auxiliary stack onto the main stack
    pub fn to_main(&mut self) {
        if let Some(val) = self.auxiliary_stack.pop() {
            self.push(val)
        }
    }

    /// Get iterator of the stack
    pub fn stack_iter(&self) -> impl Iterator<Item = &Data> {
        self.stack.iter().rev()
    }

    /// Get iterator of the aux stack
    pub fn aux_stack_iter(&self) -> impl Iterator<Item = &Data> {
        self.auxiliary_stack.iter().rev()
    }

    pub fn aux_top(&self) -> Option<&Data> {
        self.auxiliary_stack.last()
    }
}

pub trait ProgramStorage {
    fn opcode_at(&self, idx: usize) -> Option<Opcode>;
    unsafe fn opcode_at_unchecked(&self, idx: usize) -> Opcode;
}

impl<'a> ProgramStorage for &'a [u8] {
    fn opcode_at(&self, idx: usize) -> Option<Opcode> {
        self.get(idx).copied()
    }

    unsafe fn opcode_at_unchecked(&self, idx: usize) -> Opcode {
        *self.get_unchecked(idx)
    }
}

pub trait ExtendableProgramStorage: ProgramStorage {
    fn push_opcode(&mut self, op: Opcode);
}

impl ExtendableProgramStorage for Vec<u8> {
    fn push_opcode(&mut self, op: Opcode) {
        self.push(op)
    }
}

impl ProgramStorage for Vec<u8> {
    fn opcode_at(&self, idx: usize) -> Option<Opcode> {
        self.get(idx).copied()
    }

    unsafe fn opcode_at_unchecked(&self, idx: usize) -> Opcode {
        *self.get_unchecked(idx)
    }
}

/* #[cfg(feature="owned_vm")]
mod owned_vm {
    use super::*;

    #[ouroboros::self_referencing(pub_extras)]
    pub struct OwnedVm {
        program: Vec<u8>,
        #[borrows(program)]
        #[covariant]
        ctx: Vm<'this>
    }

    /* #[cfg(feature="owned_vm")]
    impl OwnedVm {
        pub fn push_op(&mut self, op: u8) {
            self.program.push(op)
        }
    } */
}

#[cfg(feature="owned_vm")]
pub use owned_vm::*; */
