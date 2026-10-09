//! 🧮️ The deterministic evaluator: kind inference first, then a strict left-to-right walk in SI base units.
//!
//! `and`, `or` and `if` are lazy, so `x != 0 and 1 / x > 2` never divides by zero; their unevaluated side is still kind-checked.
//! Numeric comparison is tolerant: two values are equal when they differ by at most [`TOLERANCE`] relative to `max(1, |a|, |b|)`, and
//! `<`, `<=`, `>`, `>=` agree with that equality. `round` rounds half away from zero.

use crate::errors::{ErrorKind, ExprError};
use crate::kinds::{binary_kind, infer, literal_exponent, Kind};
use crate::tree::{BinaryOp, CompareOp, Expr, Function, UnaryOp, Value};
use std::collections::BTreeMap;

/// 🎯️ The relative tolerance of numeric equality.
pub const TOLERANCE: f64 = 1e-9;

/// ▶️ Evaluates an expression against parameter values; an ill-kinded expression fails with the same error [`infer`] reports.
pub fn evaluate(expr: &Expr, env: &BTreeMap<String, Value>) -> Result<Value, ExprError> {
    let kinds = env.iter().map(|(name, value)| (name.clone(), value.kind())).collect();
    evaluate_with(expr, env, &kinds)
}

/// ▶️ [`evaluate`] with the parameter kinds already known, for callers that evaluate many formulas against one growing environment.
pub fn evaluate_with(expr: &Expr, env: &BTreeMap<String, Value>, kinds: &BTreeMap<String, Kind>) -> Result<Value, ExprError> {
    infer(expr, kinds)?;
    eval(expr, env, &mut Vec::new())
}

/// 🟰️ Whether two magnitudes are equal within [`TOLERANCE`].
pub fn approx_eq(x: f64, y: f64) -> bool {
    x == y || (x - y).abs() <= TOLERANCE * 1.0_f64.max(x.abs()).max(y.abs())
}

fn magnitude(value: &Value) -> f64 {
    value.magnitude().unwrap_or(0.0)
}

fn finite(at: &[usize], value: f64) -> Result<f64, ExprError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ExprError::at(at, ErrorKind::Overflow))
    }
}

fn numeric(at: &[usize], kind: Kind, value: f64) -> Result<Value, ExprError> {
    let value = finite(at, value)?;
    Value::of_kind(kind, value).ok_or_else(|| ExprError::at(at, ErrorKind::Overflow))
}

fn flag(value: &Value) -> bool {
    matches!(value, Value::Bool(true))
}

fn eval(expr: &Expr, env: &BTreeMap<String, Value>, at: &mut Vec<usize>) -> Result<Value, ExprError> {
    let sub = |index: usize, node: &Expr, at: &mut Vec<usize>| {
        at.push(index);
        let value = eval(node, env, at);
        at.pop();
        value
    };
    match expr {
        Expr::Number(v) => numeric(at, Kind::Number, *v),
        Expr::Length(v, unit) => numeric(at, Kind::Length, unit.to_si(*v)),
        Expr::Angle(v, unit) => numeric(at, Kind::Angle, unit.to_si(*v)),
        Expr::Area(v, unit) => numeric(at, Kind::Area, unit.to_si(*v)),
        Expr::Volume(v, unit) => numeric(at, Kind::Volume, unit.to_si(*v)),
        Expr::Bool(v) => Ok(Value::Bool(*v)),
        Expr::Text(v) => Ok(Value::Text(v.clone())),
        Expr::Param(name) => env.get(name).cloned().ok_or_else(|| ExprError::at(at, ErrorKind::UnknownParam { name: name.clone() })),
        Expr::Unary(UnaryOp::Negate, operand) => {
            let v = sub(0, operand, at)?;
            numeric(at, v.kind(), -magnitude(&v))
        }
        Expr::Unary(UnaryOp::Not, operand) => Ok(Value::Bool(!flag(&sub(0, operand, at)?))),
        Expr::Binary(op, left, right) => {
            let (a, b) = (sub(0, left, at)?, sub(1, right, at)?);
            let kind = binary_kind(*op, a.kind(), b.kind(), literal_exponent(right)).ok_or_else(|| ExprError::at(at, ErrorKind::MixedKinds { site: op.symbol(), left: a.kind(), right: b.kind() }))?;
            let (x, y) = (magnitude(&a), magnitude(&b));
            let raw = match op {
                BinaryOp::Add => x + y,
                BinaryOp::Subtract => x - y,
                BinaryOp::Multiply => x * y,
                BinaryOp::Divide if y == 0.0 => return Err(ExprError::at(at, ErrorKind::DivisionByZero)),
                BinaryOp::Divide => x / y,
                BinaryOp::Power if x == 0.0 && y < 0.0 => return Err(ExprError::at(at, ErrorKind::DivisionByZero)),
                BinaryOp::Power => match x.powf(y) {
                    r if r.is_nan() => return Err(ExprError::at(at, ErrorKind::Domain { site: "^" })),
                    r => r,
                },
                BinaryOp::Min => x.min(y),
                BinaryOp::Max => x.max(y),
            };
            numeric(at, kind, raw)
        }
        Expr::Compare(op, left, right) => {
            let (a, b) = (sub(0, left, at)?, sub(1, right, at)?);
            let result = match (&a, &b) {
                (Value::Bool(x), Value::Bool(y)) => (x == y) == matches!(op, CompareOp::Equal),
                (Value::Text(x), Value::Text(y)) => (x == y) == matches!(op, CompareOp::Equal),
                _ => {
                    let (x, y) = (magnitude(&a), magnitude(&b));
                    let same = approx_eq(x, y);
                    match op {
                        CompareOp::Equal => same,
                        CompareOp::NotEqual => !same,
                        CompareOp::Less => x < y && !same,
                        CompareOp::LessEqual => x < y || same,
                        CompareOp::Greater => x > y && !same,
                        CompareOp::GreaterEqual => x > y || same,
                    }
                }
            };
            Ok(Value::Bool(result))
        }
        Expr::And(left, right) => Ok(Value::Bool(flag(&sub(0, left, at)?) && flag(&sub(1, right, at)?))),
        Expr::Or(left, right) => Ok(Value::Bool(flag(&sub(0, left, at)?) || flag(&sub(1, right, at)?))),
        Expr::If(condition, then, otherwise) => {
            if flag(&sub(0, condition, at)?) {
                sub(1, then, at)
            } else {
                sub(2, otherwise, at)
            }
        }
        Expr::Call(function, args) => {
            let mut values = Vec::with_capacity(args.len());
            for (index, arg) in args.iter().enumerate() {
                values.push(sub(index, arg, at)?);
            }
            call(*function, &values, at)
        }
    }
}

fn call(function: Function, args: &[Value], at: &[usize]) -> Result<Value, ExprError> {
    let x = magnitude(&args[0]);
    let kind = args[0].kind();
    match function {
        Function::Sqrt if x < 0.0 => Err(ExprError::at(at, ErrorKind::Domain { site: "sqrt" })),
        Function::Sqrt => numeric(at, if kind == Kind::Area { Kind::Length } else { Kind::Number }, x.sqrt()),
        Function::Abs => numeric(at, kind, x.abs()),
        Function::Round => numeric(at, kind, x.round()),
        Function::Floor => numeric(at, kind, x.floor()),
        Function::Ceil => numeric(at, kind, x.ceil()),
        Function::Sin => numeric(at, Kind::Number, x.sin()),
        Function::Cos => numeric(at, Kind::Number, x.cos()),
        Function::Tan => numeric(at, Kind::Number, x.tan()),
        Function::Atan2 => numeric(at, Kind::Angle, x.atan2(magnitude(&args[1]))),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
