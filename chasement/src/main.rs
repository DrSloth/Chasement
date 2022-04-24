use std::{
    env, fs,
    io::{self, Read},
};

use chasement::{InstructionSet, Vm};

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

    /* let instructions = {
        let mut instructions: InstructionSet = HashMap::with_hasher(Default::default());
        //arithmetic operators
        instructions.insert(b'+', instructions::plus);
        //instructions.insert(b'-', instructions::minus);
        //instructions.insert(b'*', instructions::mul);
        //instructions.insert(b'/', instructions::div);
        //instructions.insert(b'%', instructions::modulo);
        //logic operators
        //instructions.insert(b'&', instructions::and);
        //instructions.insert(b'|', instructions::or);
        //instructions.insert(b'^', instructions::xor);
        //comparison operators
        //instructions.insert(b'>', instructions::gt);
        //instructions.insert(b'<', instructions::lt);


        instructions
    }; */

    let instructions = InstructionSet::new_with(|me| {
        me.with_base_instructions();
    });

    Vm::new(instructions, &program as &[u8]).run();
}
