//! 🧮️ PostScript calculator functions (PDF function type 4).
//! [ISO 32000-1 §7.10.5](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)

const MAX_STACK: usize = 100;
const MAX_STEPS: usize = 1 << 20;
const MAX_DEPTH: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Value {
    Int(i64),
    Real(f64),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Push(Value),
    Name(&'static str),
    If(Vec<Op>),
    IfElse(Vec<Op>, Vec<Op>),
}

/// 🧮️ A parsed type 4 function body.
#[derive(Clone, Debug, PartialEq)]
pub struct PostScriptProgram(Vec<Op>);

const NAMES: &[&str] = &[
    "abs", "add", "and", "atan", "bitshift", "ceiling", "copy", "cos", "cvi", "cvr", "div", "dup", "eq", "exch", "exp", "false", "floor", "ge", "gt",
    "idiv", "index", "le", "ln", "log", "lt", "mod", "mul", "ne", "neg", "not", "or", "pop", "roll", "round", "sin", "sqrt", "sub", "true", "truncate", "xor",
];

fn parse_block(tokens: &mut std::iter::Peekable<std::vec::IntoIter<String>>, depth: usize) -> Option<Vec<Op>> {
    if depth > MAX_DEPTH { return None; }
    let mut ops: Vec<Op> = Vec::new();
    let mut blocks: Vec<Vec<Op>> = Vec::new();
    while let Some(token) = tokens.next() {
        match token.as_str() {
            "{" => blocks.push(parse_block(tokens, depth + 1)?),
            "}" => return blocks.is_empty().then_some(ops),
            "if" => ops.push(Op::If(blocks.pop()?)),
            "ifelse" => {
                let otherwise = blocks.pop()?;
                ops.push(Op::IfElse(blocks.pop()?, otherwise));
            }
            other => {
                if !blocks.is_empty() { return None; }
                if let Ok(int) = other.parse::<i64>() { ops.push(Op::Push(Value::Int(int))); }
                else if let Ok(real) = other.parse::<f64>() { ops.push(Op::Push(Value::Real(real))); }
                else { ops.push(Op::Name(NAMES.iter().find(|name| **name == other)?)); }
            }
        }
    }
    None
}

impl PostScriptProgram {
    /// 📥️ Parses a calculator body such as `{ dup mul 1 exch sub }`.
    pub fn parse(code: &str) -> Option<Self> {
        let spaced = code.replace('{', " { ").replace('}', " } ");
        let mut tokens = spaced.split_whitespace().map(str::to_owned).collect::<Vec<_>>().into_iter().peekable();
        (tokens.next()? == "{").then_some(())?;
        let ops = parse_block(&mut tokens, 0)?;
        tokens.next().is_none().then_some(Self(ops))
    }

    /// ▶️ Runs the program on `inputs` and returns the top `outputs` stack values.
    pub fn run(&self, inputs: &[f64], outputs: usize) -> Option<Vec<f64>> {
        let mut stack: Vec<Value> = inputs.iter().map(|&x| Value::Real(x)).collect();
        let mut steps = 0usize;
        execute(&self.0, &mut stack, &mut steps)?;
        (stack.len() >= outputs).then(|| stack[stack.len() - outputs..].iter().map(|&value| number(value)).collect())
    }
}

fn number(value: Value) -> f64 {
    match value { Value::Int(i) => i as f64, Value::Real(r) => r, Value::Bool(b) => f64::from(u8::from(b)) }
}

fn pop(stack: &mut Vec<Value>) -> Option<Value> { stack.pop() }

fn pop_int(stack: &mut Vec<Value>) -> Option<i64> {
    match pop(stack)? { Value::Int(i) => Some(i), Value::Real(r) if r.fract() == 0.0 => Some(r as i64), _ => None }
}

fn pop_real(stack: &mut Vec<Value>) -> Option<f64> {
    match pop(stack)? { Value::Bool(_) => None, other => Some(number(other)) }
}

fn execute(ops: &[Op], stack: &mut Vec<Value>, steps: &mut usize) -> Option<()> {
    for op in ops {
        *steps += 1;
        if *steps > MAX_STEPS || stack.len() > MAX_STACK { return None; }
        match op {
            Op::Push(value) => stack.push(*value),
            Op::If(body) => { if matches!(pop(stack)?, Value::Bool(true)) { execute(body, stack, steps)?; } }
            Op::IfElse(then, otherwise) => {
                let Value::Bool(condition) = pop(stack)? else { return None };
                execute(if condition { then } else { otherwise }, stack, steps)?;
            }
            Op::Name(name) => apply(name, stack)?,
        }
    }
    Some(())
}

fn arithmetic(stack: &mut Vec<Value>, int: fn(i64, i64) -> Option<i64>, real: fn(f64, f64) -> f64) -> Option<()> {
    let (b, a) = (pop(stack)?, pop(stack)?);
    stack.push(match (a, b) {
        (Value::Int(x), Value::Int(y)) => match int(x, y) { Some(value) => Value::Int(value), None => Value::Real(real(x as f64, y as f64)) },
        (Value::Bool(_), _) | (_, Value::Bool(_)) => return None,
        (x, y) => Value::Real(real(number(x), number(y))),
    });
    Some(())
}

fn compare(stack: &mut Vec<Value>, test: fn(f64, f64) -> bool) -> Option<()> {
    let (b, a) = (pop(stack)?, pop(stack)?);
    stack.push(Value::Bool(test(number(a), number(b))));
    Some(())
}

fn logic(stack: &mut Vec<Value>, boolean: fn(bool, bool) -> bool, int: fn(i64, i64) -> i64) -> Option<()> {
    let (b, a) = (pop(stack)?, pop(stack)?);
    stack.push(match (a, b) {
        (Value::Bool(x), Value::Bool(y)) => Value::Bool(boolean(x, y)),
        (Value::Int(x), Value::Int(y)) => Value::Int(int(x, y)),
        _ => return None,
    });
    Some(())
}

fn unary(stack: &mut Vec<Value>, int: fn(i64) -> i64, real: fn(f64) -> f64) -> Option<()> {
    let value = pop(stack)?;
    stack.push(match value { Value::Int(i) => Value::Int(int(i)), Value::Real(r) => Value::Real(real(r)), Value::Bool(_) => return None });
    Some(())
}

fn apply(name: &str, stack: &mut Vec<Value>) -> Option<()> {
    match name {
        "abs" => unary(stack, i64::wrapping_abs, f64::abs),
        "add" => arithmetic(stack, i64::checked_add, |a, b| a + b),
        "sub" => arithmetic(stack, i64::checked_sub, |a, b| a - b),
        "mul" => arithmetic(stack, i64::checked_mul, |a, b| a * b),
        "div" => { let (b, a) = (pop_real(stack)?, pop_real(stack)?); stack.push(Value::Real(a / b)); Some(()) }
        "idiv" => { let (b, a) = (pop_int(stack)?, pop_int(stack)?); stack.push(Value::Int(a.checked_div(b)?)); Some(()) }
        "mod" => { let (b, a) = (pop_int(stack)?, pop_int(stack)?); stack.push(Value::Int(a.checked_rem(b)?)); Some(()) }
        "neg" => unary(stack, i64::wrapping_neg, |r| -r),
        "ceiling" => unary(stack, |i| i, f64::ceil),
        "floor" => unary(stack, |i| i, f64::floor),
        "round" => unary(stack, |i| i, |r| (r + 0.5).floor()),
        "truncate" => unary(stack, |i| i, f64::trunc),
        "sqrt" => { let x = pop_real(stack)?; stack.push(Value::Real(x.sqrt())); Some(()) }
        "sin" => { let x = pop_real(stack)?; stack.push(Value::Real(x.to_radians().sin())); Some(()) }
        "cos" => { let x = pop_real(stack)?; stack.push(Value::Real(x.to_radians().cos())); Some(()) }
        "atan" => { let (den, num) = (pop_real(stack)?, pop_real(stack)?); stack.push(Value::Real(num.atan2(den).to_degrees().rem_euclid(360.0))); Some(()) }
        "exp" => { let (e, b) = (pop_real(stack)?, pop_real(stack)?); stack.push(Value::Real(b.powf(e))); Some(()) }
        "ln" => { let x = pop_real(stack)?; stack.push(Value::Real(x.ln())); Some(()) }
        "log" => { let x = pop_real(stack)?; stack.push(Value::Real(x.log10())); Some(()) }
        "cvi" => { let x = pop_real(stack)?; stack.push(Value::Int(x.trunc() as i64)); Some(()) }
        "cvr" => { let x = pop_real(stack)?; stack.push(Value::Real(x)); Some(()) }
        "eq" => compare(stack, |a, b| a == b),
        "ne" => compare(stack, |a, b| a != b),
        "ge" => compare(stack, |a, b| a >= b),
        "gt" => compare(stack, |a, b| a > b),
        "le" => compare(stack, |a, b| a <= b),
        "lt" => compare(stack, |a, b| a < b),
        "and" => logic(stack, |a, b| a && b, |a, b| a & b),
        "or" => logic(stack, |a, b| a || b, |a, b| a | b),
        "xor" => logic(stack, |a, b| a ^ b, |a, b| a ^ b),
        "not" => { let value = pop(stack)?; stack.push(match value { Value::Bool(b) => Value::Bool(!b), Value::Int(i) => Value::Int(!i), Value::Real(_) => return None }); Some(()) }
        "bitshift" => {
            let (shift, value) = (pop_int(stack)?, pop_int(stack)?);
            stack.push(Value::Int(if shift >= 0 { value.checked_shl(shift as u32)? } else { value >> (-shift).min(63) }));
            Some(())
        }
        "true" => { stack.push(Value::Bool(true)); Some(()) }
        "false" => { stack.push(Value::Bool(false)); Some(()) }
        "pop" => pop(stack).map(|_| ()),
        "dup" => { let top = *stack.last()?; stack.push(top); Some(()) }
        "exch" => { let length = stack.len(); (length >= 2).then(|| stack.swap(length - 1, length - 2)) }
        "copy" => {
            let count = usize::try_from(pop_int(stack)?).ok()?;
            let start = stack.len().checked_sub(count)?;
            stack.extend_from_within(start..);
            Some(())
        }
        "index" => { let at = usize::try_from(pop_int(stack)?).ok()?; stack.push(*stack.get(stack.len().checked_sub(at + 1)?)?); Some(()) }
        "roll" => {
            let (shift, count) = (pop_int(stack)?, usize::try_from(pop_int(stack)?).ok()?);
            let start = stack.len().checked_sub(count)?;
            if count > 0 { stack[start..].rotate_right(shift.rem_euclid(count as i64) as usize); }
            Some(())
        }
        _ => None,
    }
}
