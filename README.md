## Idea 
This is a simulation of pushdown automaton with 2 stacks. A main stack and an auxiliary stack. 
The aux stack can not be read or written directly. There are two instructions to interact with 
the aux stack: `aux`('a') and `main`('m'). The aux instruction takes the top element of the main
stack and moves it to the top of the aux stack. The main instruction does the same in reverse,
move the top element of the aux stack to the main stack.

## Name
The name is a wordplay on "basement" and character. 

Every instruction in the language is a single character and the language is stack based. The german
name for a pushdown automaton (an automaton with a stack) is "Kellerautomat" which can be translated
as something like "basement automaton" thus char and basement.

### "Proof" why this should be turing complete
A PDA with two Stacks is (or at least should be) turing complete. 
Generally you can think of the read/write head as being on the top of the main stack. 
The rest of the main stack is everything that is left from the read/write head, the aux stack
is everything that is right from the read/write head. This means an `aux` instruction is the same
as moving left in a turing machine and `main` is moving right.

## Features
Currently this is only a simulation for the above described automaton. 
Maybe features like arithmetic or logic instructions will be added to make this a stupid compile 
target. Then there will be a command line flag which turns of the "extra" instructions. 

There are some "unnecessary" instructions which could have been implemented using other 
instructions. For instance the `w` instruction could have been implemented with `aamm`.
All instructions that currently exist (besides the `+` instruction) should be things that a normal
PDA with access to two stacks should be able to perform.

### Examples
There are some examples in the [examples](/examples/) directory. 
For instance the [check_anbncn.chase](examples/check_anbncn.chase) file is can check if an input
is of the form `a^nb^nc^n` which is a context sensitive grammar, thus the automaton can definitely
do more than a normal PDA. If this automaton can check every context sensitive grammar obviously
can't be tested, but every grammar of the form `a^nb^nc^n`, `a^nb^nc^nd^n`... can be checked.

## Instructions
TODO add documentation of all instructions

## License
This project is licensed under the MIT license see [LICENSE](/LICENSE) for more info.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by
you shall be licensed under the MIT license too.
