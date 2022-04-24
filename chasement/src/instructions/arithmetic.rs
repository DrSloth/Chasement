/// ('+') Pops two values of the stack and pushes their sum.
/// Works only for Floats, Ints and Strings
pub fn plus(ctx: &mut Context) {
    match (ctx.pop(), ctx.pop()) {
        (Some(Data::Int(a)), Some(Data::Int(b))) => ctx.push(Data::Int(a + b)),
        (a, b) => error(&format!(
            "'+' (Plus) called on invalid combination ({:?}, {:?})",
            a, b
        )),
    }
}