mod instructions;

use std::{
    collections::HashMap,
    env,
    fmt::{self, Display, Formatter},
    fs,
    io::{self, Read},
};

pub type Instruction = fn(&mut Context);
pub type InstructionSet = HashMap<u8, Instruction, nohash::BuildNoHashHasher<u8>>;

#[derive(Clone)]
pub struct VM<'a> {
    /// All available instructions, indexed by the ascii value of its responding char.
    /// Will be changed to a const array later.
    instructions: InstructionSet,
    ctx: Context<'a>,
}

impl<'a> VM<'a> {
    pub fn new(instructions: InstructionSet, data: &'a [u8]) -> Self {
        Self {
            instructions,
            ctx: Context::new(data),
        }
    }

    pub fn run(&mut self) {
        while let Some(byte) = self.ctx.program.get(self.ctx.pc) {
            let instruction = self.instructions.get(byte).unwrap_or_else(|| {
                panic!("No instruction for {:?} at {}", *byte as char, self.ctx.pc)
            });
            instruction(&mut self.ctx);
            //Use wrapping_add here because of jumps semantics
            self.ctx.pc = self.ctx.pc.wrapping_add(1);
        }
    }

    pub fn get_context(&self) -> &Context {
        &self.ctx
    }

    pub fn get_context_mut(&mut self) -> &mut Context<'a> {
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
    Float(f64)
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
pub struct Context<'a> {
    /// Value Stack
    stack: Vec<Data>,
    /// Auxiliary stack
    auxiliary_stack: Vec<Data>,
    /// Program counter (current instruction)
    pc: usize,
    program: &'a [u8],
}

impl<'a> Context<'a> {
    /// Create an empty Context
    pub fn new(program: &'a [u8]) -> Self {
        Self {
            stack: Default::default(),
            auxiliary_stack: Default::default(),
            pc: 0,
            program,
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
        self.program.get(self.pc).copied()
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
    pub fn stack_iter(&self) -> impl Iterator<Item=&Data> {
        self.stack.iter().rev()
    }

    /// Get iterator of the aux stack
    pub fn aux_stack_iter(&self) -> impl Iterator<Item=&Data> {
        self.auxiliary_stack.iter().rev()
    }

    pub fn aux_top(&self) -> Option<&Data> {
        self.auxiliary_stack.last()
    }
}

fn main() {
    let program = env::args()
        .skip(1)
        .next()
        .map(|path| fs::read(path).unwrap())
        .unwrap_or_else(|| {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            let mut v = Vec::new();

            stdin.read_to_end(&mut v).unwrap();
            v
        });

    let instructions = {
        let mut instructions: InstructionSet = HashMap::with_hasher(Default::default());
        instructions.insert(b'#', instructions::comment);
        instructions.insert(b',', instructions::input);
        instructions.insert(b'\'', instructions::charify);
        instructions.insert(b' ', instructions::nop);
        instructions.insert(b'\n', instructions::nop);
        instructions.insert(b'a', instructions::auxiliary_push);
        instructions.insert(b'd', instructions::dup);
        instructions.insert(b'e', instructions::empty);
        instructions.insert(b'f', |ctx| ctx.push(Data::Bool(false)));
        instructions.insert(b'h', instructions::print_stack);
        instructions.insert(b'j', instructions::jump);
        instructions.insert(b'm', instructions::main_push);
        //instructions.insert(b'n', instructions::negate);
        instructions.insert(b'o', instructions::drop);
        instructions.insert(b'p', instructions::print);
        instructions.insert(b's', instructions::skip_if);
        instructions.insert(b't', |ctx| ctx.push(Data::Bool(true)));
        instructions.insert(b'w', instructions::swap);
        instructions.insert(b'x', instructions::exit);
        instructions.insert(b'z', instructions::aux_empty);
        for c in b'0'..=b'9' {
            instructions.insert(c, instructions::digit);
        }
        //arithmetic operators
        instructions.insert(b'+', instructions::plus);
        //instructions.insert(b'-', instructions::minus);
        //instructions.insert(b'*', instructions::mul);
        //instructions.insert(b'/', instructions::div);
        //instructions.insert(b'%', instructions::modulo);
        //logic operators
        instructions.insert(b'!', instructions::not);
        //instructions.insert(b'&', instructions::and);
        //instructions.insert(b'|', instructions::or);
        //instructions.insert(b'^', instructions::xor);
        //comparison operators
        //instructions.insert(b'>', instructions::gt);
        //instructions.insert(b'<', instructions::lt);
        instructions.insert(b'=', instructions::eq);

        instructions.insert(b'[', instructions::cur_pc);
        instructions.insert(b']', instructions::jump_back);

        instructions.insert(b'(', instructions::paren_open);
        instructions.insert(b')', instructions::nop);

        instructions
    };

    VM::new(instructions, &program).run();
}
