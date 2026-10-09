//! 📏️ Dimensional kinds and their inference.
//!
//! `Length * Length` is an `Area`, `Area / Length` is a `Length`, `Length + Angle` is an error: the rules below are the whole table.
//! A dimensional base can only be raised to a literal power that lands on a kind (`Length ^ 2` is an `Area`, `Area ^ 0.5` a `Length`).

use crate::errors::{ErrorKind, ExprError};
use crate::tree::{BinaryOp, Expr, Function, UnaryOp};
use std::collections::BTreeMap;

/// 🏷️ What an expression computes: a dimensionless number, a physical quantity, a boolean or text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Number,
    Length,
    Angle,
    Area,
    Volume,
    Bool,
    Text,
}

impl Kind {
    /// 📋️ Every kind in canonical order.
    pub const ALL: &'static [Kind] = &[Kind::Number, Kind::Length, Kind::Angle, Kind::Area, Kind::Volume, Kind::Bool, Kind::Text];

    /// 🔢️ The kinds that carry a magnitude.
    pub const NUMERIC: &'static [Kind] = &[Kind::Number, Kind::Length, Kind::Angle, Kind::Area, Kind::Volume];

    /// 🏷️ The stable lowercase name.
    pub fn name(self) -> &'static str {
        match self {
            Kind::Number => "number",
            Kind::Length => "length",
            Kind::Angle => "angle",
            Kind::Area => "area",
            Kind::Volume => "volume",
            Kind::Bool => "bool",
            Kind::Text => "text",
        }
    }

    /// 🔎️ Resolves a kind by its stable name.
    pub fn from_name(name: &str) -> Option<Kind> {
        Kind::ALL.iter().copied().find(|k| k.name() == name)
    }

    /// 🔢️ Whether the kind carries a magnitude.
    pub fn is_numeric(self) -> bool {
        !matches!(self, Kind::Bool | Kind::Text)
    }
}

/// 🧮️ The kind of `left op right` for an arithmetic operator, or `None` when the dimensions do not combine; `exponent` is the literal power of a `Power`.
pub fn binary_kind(op: BinaryOp, left: Kind, right: Kind, exponent: Option<f64>) -> Option<Kind> {
    use Kind::*;
    match op {
        BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Min | BinaryOp::Max => (left == right).then_some(left),
        BinaryOp::Multiply => match (left, right) {
            (k, Number) | (Number, k) => Some(k),
            (Length, Length) => Some(Area),
            (Length, Area) | (Area, Length) => Some(Volume),
            _ => None,
        },
        BinaryOp::Divide => match (left, right) {
            (k, Number) => Some(k),
            (a, b) if a == b => Some(Number),
            (Area, Length) => Some(Length),
            (Volume, Length) => Some(Area),
            (Volume, Area) => Some(Length),
            _ => None,
        },
        BinaryOp::Power => match (left, right) {
            (Number, Number) => Some(Number),
            (base, Number) => match (base, exponent?) {
                (Length, p) if p == 1.0 => Some(Length),
                (Length, p) if p == 2.0 => Some(Area),
                (Length, p) if p == 3.0 => Some(Volume),
                (Area, p) if p == 1.0 => Some(Area),
                (Area, p) if p == 0.5 => Some(Length),
                (Volume, p) if p == 1.0 => Some(Volume),
                (Angle, p) if p == 1.0 => Some(Angle),
                _ => None,
            },
            _ => None,
        },
    }
}

/// 🔢️ The literal value of a power's exponent: a non-negative number literal, possibly negated.
pub fn literal_exponent(expr: &Expr) -> Option<f64> {
    match expr {
        Expr::Number(v) => Some(*v),
        Expr::Unary(UnaryOp::Negate, inner) => match **inner {
            Expr::Number(v) => Some(-v),
            _ => None,
        },
        _ => None,
    }
}

/// 🧰️ The kind of a function call over argument kinds, or the first rejected argument position with the kinds it expected.
pub fn call_kind(function: Function, args: &[Kind]) -> Result<Kind, CallReject> {
    use Kind::*;
    let reject = |position: usize, expected: &'static [Kind]| Err(CallReject::Operand { position, expected });
    match function {
        Function::Sqrt => match args[0] {
            Number => Ok(Number),
            Area => Ok(Length),
            _ => reject(0, &[Number, Area]),
        },
        Function::Abs => match args[0] {
            k if k.is_numeric() => Ok(k),
            _ => reject(0, Kind::NUMERIC),
        },
        Function::Round | Function::Floor | Function::Ceil => match args[0] {
            Number => Ok(Number),
            _ => reject(0, &[Number]),
        },
        Function::Sin | Function::Cos | Function::Tan => match args[0] {
            Angle => Ok(Number),
            _ => reject(0, &[Angle]),
        },
        Function::Atan2 => match (args[0], args[1]) {
            (a, _) if !a.is_numeric() => reject(0, Kind::NUMERIC),
            (_, b) if !b.is_numeric() => reject(1, Kind::NUMERIC),
            (a, b) if a != b => Err(CallReject::Mixed),
            _ => Ok(Angle),
        },
    }
}

/// 🚫️ Why [`call_kind`] refused its arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallReject {
    Operand { position: usize, expected: &'static [Kind] },
    Mixed,
}

/// 🔍️ Infers the kind of an expression given the kinds of its parameters, rejecting the first ill-kinded node in evaluation order.
pub fn infer(expr: &Expr, params: &BTreeMap<String, Kind>) -> Result<Kind, ExprError> {
    walk(expr, params, &mut Vec::new())
}

fn walk(expr: &Expr, params: &BTreeMap<String, Kind>, at: &mut Vec<usize>) -> Result<Kind, ExprError> {
    let child = |index: usize, node: &Expr, at: &mut Vec<usize>| {
        at.push(index);
        let kind = walk(node, params, at);
        at.pop();
        kind
    };
    let fail = |at: &[usize], kind: ErrorKind| Err(ExprError::at(at, kind));
    match expr {
        Expr::Number(_) => Ok(Kind::Number),
        Expr::Length(..) => Ok(Kind::Length),
        Expr::Angle(..) => Ok(Kind::Angle),
        Expr::Area(..) => Ok(Kind::Area),
        Expr::Volume(..) => Ok(Kind::Volume),
        Expr::Bool(_) => Ok(Kind::Bool),
        Expr::Text(_) => Ok(Kind::Text),
        Expr::Param(name) => match params.get(name) {
            Some(kind) => Ok(*kind),
            None => fail(at, ErrorKind::UnknownParam { name: name.clone() }),
        },
        Expr::Unary(op, operand) => {
            let found = child(0, operand, at)?;
            let (accepts, expected): (bool, &'static [Kind]) = match op {
                UnaryOp::Negate => (found.is_numeric(), Kind::NUMERIC),
                UnaryOp::Not => (found == Kind::Bool, &[Kind::Bool]),
            };
            if accepts {
                Ok(found)
            } else {
                fail(at, ErrorKind::OperandKind { site: op.symbol(), position: 0, expected, found })
            }
        }
        Expr::Binary(op, left, right) => {
            let (l, r) = (child(0, left, at)?, child(1, right, at)?);
            let site = op.symbol();
            if !l.is_numeric() {
                return fail(at, ErrorKind::OperandKind { site, position: 0, expected: Kind::NUMERIC, found: l });
            }
            if !r.is_numeric() {
                return fail(at, ErrorKind::OperandKind { site, position: 1, expected: Kind::NUMERIC, found: r });
            }
            if *op == BinaryOp::Power {
                if r != Kind::Number {
                    return fail(at, ErrorKind::OperandKind { site, position: 1, expected: &[Kind::Number], found: r });
                }
                let exponent = literal_exponent(right);
                return binary_kind(*op, l, r, exponent).map_or_else(|| fail(at, ErrorKind::Exponent { base: l, exponent }), Ok);
            }
            binary_kind(*op, l, r, None).map_or_else(|| fail(at, ErrorKind::MixedKinds { site, left: l, right: r }), Ok)
        }
        Expr::Compare(op, left, right) => {
            let (l, r) = (child(0, left, at)?, child(1, right, at)?);
            let site = op.symbol();
            if !op.is_equality() {
                if !l.is_numeric() {
                    return fail(at, ErrorKind::OperandKind { site, position: 0, expected: Kind::NUMERIC, found: l });
                }
                if !r.is_numeric() {
                    return fail(at, ErrorKind::OperandKind { site, position: 1, expected: Kind::NUMERIC, found: r });
                }
            }
            if l == r {
                Ok(Kind::Bool)
            } else {
                fail(at, ErrorKind::MixedKinds { site, left: l, right: r })
            }
        }
        Expr::And(left, right) | Expr::Or(left, right) => {
            let site = if matches!(expr, Expr::And(..)) { "and" } else { "or" };
            let (l, r) = (child(0, left, at)?, child(1, right, at)?);
            for (position, found) in [l, r].into_iter().enumerate() {
                if found != Kind::Bool {
                    return fail(at, ErrorKind::OperandKind { site, position, expected: &[Kind::Bool], found });
                }
            }
            Ok(Kind::Bool)
        }
        Expr::If(condition, then, otherwise) => {
            let c = child(0, condition, at)?;
            let (t, e) = (child(1, then, at)?, child(2, otherwise, at)?);
            if c != Kind::Bool {
                return fail(at, ErrorKind::OperandKind { site: "if", position: 0, expected: &[Kind::Bool], found: c });
            }
            if t == e {
                Ok(t)
            } else {
                fail(at, ErrorKind::Branches { then: t, otherwise: e })
            }
        }
        Expr::Call(function, args) => {
            let site = function.name();
            let mut kinds = Vec::with_capacity(args.len());
            for (index, arg) in args.iter().enumerate() {
                kinds.push(child(index, arg, at)?);
            }
            if kinds.len() != function.arity() {
                return fail(at, ErrorKind::Arity { site, expected: function.arity(), found: kinds.len() });
            }
            match call_kind(*function, &kinds) {
                Ok(kind) => Ok(kind),
                Err(CallReject::Operand { position, expected }) => fail(at, ErrorKind::OperandKind { site, position, expected, found: kinds[position] }),
                Err(CallReject::Mixed) => fail(at, ErrorKind::MixedKinds { site, left: kinds[0], right: kinds[1] }),
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
