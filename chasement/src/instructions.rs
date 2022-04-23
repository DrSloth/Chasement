use std::io::{Read, self};

use crate::{Context, Data};

//TODO maybe this should be a bit splitted into multiple modules

pub fn error(err: &str) -> ! {
    eprintln!("ERROR: {}", err);
    std::process::exit(1)
}

/// (' ') Do nothing. Represented by one spacebar
pub fn nop(_ctx: &mut Context) {}

/// ('#') Comment out everything to the next '#' or '\n'
pub fn comment(ctx: &mut Context) {
    ctx.advance();
    while let Some(ch) = ctx.cur_byte() {
        if ch == b'#' || ch == b'\n' {
            break;
        }
        ctx.advance();
    }
}

/// ('0'-'9') Parse a number. Should only be entered through a digit.
///
/// If the current byte at the program counter is not a digit this will push 0.
pub fn digit(ctx: &mut Context) {
    let mut num = 0i64;
    loop {
        if let Some(digit) = ctx.cur_byte() {
            if digit >= 10 + b'0' || digit < b'0' {
                ctx.prev();
                break;
            }
            let digit = digit - b'0';
            num *= 10;

            num += digit as i64;
        }
        ctx.advance();
    }

    ctx.push(Data::Int(num as i64))
}

/// ('a') Pop a value from the main stack and push it to the auxiliary stack.
/// Does nothing if stack is empty
pub fn auxiliary_push(ctx: &mut Context) {
    ctx.to_auxiliary()
}

/// ('m') Pop a value from the main stack and push it to the auxiliary stack
/// Does nothing if auxiliary stack is empty
pub fn main_push(ctx: &mut Context) {
    ctx.to_main()
}

/// ('p') Print the top element of the stack
pub fn print(ctx: &mut Context) {
    if let Some(val) = ctx.pop() {
        print!("{}", val);
    } else {
        error("Called print on an empty stack")
    }
}

/// ('d') Duplicate the top element of the stack
pub fn dup(ctx: &mut Context) {
    let val = if let Some(val) = ctx.top() {
        val.clone()
    } else {
        error("Called dup on an empty stack")
    };

    ctx.push(val.clone())
}

/// ('e') Push to the stack wether the stack is empty.
/// This pushes true if the stack is empty fals otherwise.
pub fn empty(ctx: &mut Context) {
    ctx.push(Data::Bool(matches!(ctx.top(), None)))
}

/// ('j') Jump to the address provided by the top element. Pops one value of the stack.
/// Exits with an error if top element is not an int, or stack is empty.
pub fn jump(ctx: &mut Context) {
    match ctx.pop() {
        Some(Data::Int(i)) => ctx.set_pc((i as usize).wrapping_sub(1)),
        None => error("Called jump on empty stack"),
        _ => error("Called jump on non int element"),
    }
}

/// ('s') Pops the top value and skips one instruction if the top value is a true bool.
pub fn skip_if(ctx: &mut Context) {
    match ctx.pop() {
        Some(Data::Bool(true)) => ctx.advance(),
        Some(Data::Bool(false)) => (),
        _ => error("Skip called on a non boolean value"),
    }
}

/// ('!') Pops a value of the stack and pushes the bitwise negation
pub fn not(ctx: &mut Context) {
    match ctx.pop() {
        Some(Data::Bool(b)) => ctx.push(Data::Bool(!b)),
        Some(Data::Int(i)) => ctx.push(Data::Int(!i)),
        _ => error("Not called on a non Int or Bool value"),
    }
}

/// ('+') Pops two values of the stack and pushes their sum. 
/// Works only for Floats, Ints and Strings
pub fn plus(ctx: &mut Context) {
    match (ctx.pop(), ctx.pop()) {
        (Some(Data::Int(a)), Some(Data::Int(b))) => ctx.push(Data::Int(a + b)),
        (a,b) => error(&format!("'+' (Plus) called on invalid combination ({:?}, {:?})", a, b)),
    }
}

/// ('=') Pops two values and pushes wether they are equal (type and value)
pub fn eq(ctx: &mut Context) {
    match (ctx.pop(), ctx.pop()) {
        (Some(a), Some(b)) => ctx.push(Data::Bool(a == b)),
        (a,b) => error(&format!("'=' (Eq) called on invalid combination ({:?}, {:?})", a, b)),
    }
}

/// ('h') Print the complete stack
pub fn print_stack(ctx: &mut Context) {
    println!("Main: [");
    for val in ctx.stack_iter() {
        println!("    {:?},", val);
    }
    println!("]");
    println!("Aux: [");
    for val in ctx.aux_stack_iter() {
        println!("    {:?},", val);
    }
    println!("]");
}

/// (',') Read one ascii char from stdin
pub fn input(ctx: &mut Context) {
    // TODO this could be made more efficient
    let mut buf = [0;1];
    io::stdin().read(&mut buf).unwrap();
    ctx.push(Data::Char(buf[0] as char));
}

/// ('\'') Push next byte as char to the stack
pub fn charify(ctx: &mut Context) {
    ctx.advance();
    if let Some(byte) = ctx.cur_byte() {
        // Special case for escape sequence
        if byte == b'\\' {
            ctx.advance();
            let byte2 = ctx.cur_byte().unwrap();
            // Match over all supported escape sequences
            match byte2 {
                b'n' => {
                    ctx.push(Data::Char('\n'))
                }
                b => {
                    error(&format!("Invalid escape sequence \\{}", b as char))
                }
            }
        } else {
            ctx.push(Data::Char(byte as char))
        }
    } else {
        error("Used ' directly before EOF")
    }
}

/// ('[') Push current pc to the stack as int
pub fn cur_pc(ctx: &mut Context) {
    ctx.push(Data::Int(ctx.get_pc() as i64));
}

/// (']') Jump back to the last open square bracket '['
pub fn jump_back(ctx: &mut Context) {
    let mut cnt = 0;
    while let Some(b) = ctx.cur_byte() {
        match (b, cnt) {
            (b'[', 1) => {
                ctx.prev();
                break
            }
            (b'[', _) => {
                cnt -= 1;
            }
            (b']', _) => {
                cnt += 1;
            }
            _ => (),
        }
        ctx.prev();
    }
}

/// ('(') Jump ahead to the next closed paranthese ')'
pub fn paren_open(ctx: &mut Context) {
    let mut cnt = 0;
    while let Some(byte) = ctx.cur_byte() {
        match (byte, cnt) {
            (b')', 1) => break,
            (b')', _) => cnt -=1,
            (b'(', _) => cnt +=1,
            _ => (),
        }
        ctx.advance();
    }
}

/// ('w') Swap the top two values, panics if there are less than two values on the stack
pub fn swap(ctx: &mut Context) {
    match (ctx.pop(), ctx.pop()) {
        (Some(a), Some(b)) => {
            ctx.push(a);
            ctx.push(b);
        },
        v => error(&format!("'w' (Swap) called on invalid stack ({:?})", v)),
    }
}

/// ('o') Drop the top value
pub fn drop(ctx: &mut Context) {
    ctx.pop();
}

/// ('x') Drop the top value
pub fn exit(_ctx: &mut Context) {
    // TODO probably all the exits should rather be handled through a custom panic hook
    std::process::exit(0);
}

/// ('z') Auxiliary stack zero. Push if the auxiliary stack is empty 
pub fn aux_empty(ctx: &mut Context) {
    ctx.push(Data::Bool(matches!(ctx.aux_top(), None)))
}
