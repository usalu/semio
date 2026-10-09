//! ⚠️ Errors as values: every failure of inference, evaluation and parameter resolution is plain data with a stable code, never a panic.
//!
//! The English `message` is a fallback for logs; a UI localises from [`ErrorKind::code`] and the structured fields.

use crate::kinds::Kind;
use std::fmt;

/// 🚨️ A failure located at a node of the expression, addressed by child indices from the root (see [`crate::Expr::at`]).
#[derive(Clone, Debug, PartialEq)]
pub struct ExprError {
    pub path: Vec<usize>,
    pub kind: ErrorKind,
}

/// 🧾️ What went wrong; `site` is the operator symbol or function name that rejected its operands.
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    UnknownParam { name: String },
    OperandKind { site: &'static str, position: usize, expected: &'static [Kind], found: Kind },
    MixedKinds { site: &'static str, left: Kind, right: Kind },
    Arity { site: &'static str, expected: usize, found: usize },
    Exponent { base: Kind, exponent: Option<f64> },
    Branches { then: Kind, otherwise: Kind },
    DivisionByZero,
    Domain { site: &'static str },
    Overflow,
    Cycle { members: Vec<String> },
    FailedDependency { name: String },
    UnknownOverride,
    DeclaredKind { declared: Kind, found: Kind },
}

impl ExprError {
    /// 🏗️ An error at a path.
    pub fn at(path: &[usize], kind: ErrorKind) -> Self {
        ExprError { path: path.to_vec(), kind }
    }

    /// 🏗️ An error about a whole parameter rather than a node inside its formula.
    pub fn parameter(kind: ErrorKind) -> Self {
        ExprError { path: Vec::new(), kind }
    }

    /// 🏷️ The stable machine code of the error.
    pub fn code(&self) -> &'static str {
        self.kind.code()
    }
}

impl ErrorKind {
    /// 🏷️ The stable kebab-case code, the key of translations and fixtures.
    pub fn code(&self) -> &'static str {
        match self {
            ErrorKind::UnknownParam { .. } => "unknown-parameter",
            ErrorKind::OperandKind { .. } => "operand-kind",
            ErrorKind::MixedKinds { .. } => "mixed-kinds",
            ErrorKind::Arity { .. } => "arity",
            ErrorKind::Exponent { .. } => "exponent",
            ErrorKind::Branches { .. } => "branch-kinds",
            ErrorKind::DivisionByZero => "division-by-zero",
            ErrorKind::Domain { .. } => "domain",
            ErrorKind::Overflow => "overflow",
            ErrorKind::Cycle { .. } => "cycle",
            ErrorKind::FailedDependency { .. } => "failed-dependency",
            ErrorKind::UnknownOverride => "unknown-override",
            ErrorKind::DeclaredKind { .. } => "declared-kind",
        }
    }

    /// 💬️ The English fallback message.
    pub fn message(&self) -> String {
        match self {
            ErrorKind::UnknownParam { name } => format!("unknown parameter `{name}`"),
            ErrorKind::OperandKind { site, position, expected, found } => {
                let wanted: Vec<&str> = expected.iter().map(|k| k.name()).collect();
                format!("operand {} of `{site}` must be {} but is {}", position + 1, wanted.join(" or "), found.name())
            }
            ErrorKind::MixedKinds { site, left, right } => format!("`{site}` cannot combine {} with {}", left.name(), right.name()),
            ErrorKind::Arity { site, expected, found } => format!("`{site}` takes {expected} argument(s) but got {found}"),
            ErrorKind::Exponent { base, exponent: Some(e) } => format!("a {} cannot be raised to the power {e}", base.name()),
            ErrorKind::Exponent { base, exponent: None } => format!("a {} can only be raised to a literal power", base.name()),
            ErrorKind::Branches { then, otherwise } => format!("the branches of `if` differ: {} versus {}", then.name(), otherwise.name()),
            ErrorKind::DivisionByZero => "division by zero".to_string(),
            ErrorKind::Domain { site } => format!("`{site}` is undefined for this operand"),
            ErrorKind::Overflow => "the result is not a finite number".to_string(),
            ErrorKind::Cycle { members } => format!("circular reference between {}", members.join(", ")),
            ErrorKind::FailedDependency { name } => format!("depends on `{name}` which has no value"),
            ErrorKind::UnknownOverride => "overrides a parameter that does not exist".to_string(),
            ErrorKind::DeclaredKind { declared, found } => format!("declared as {} but computes {}", declared.name(), found.name()),
        }
    }
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.message())
    }
}

impl std::error::Error for ExprError {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
