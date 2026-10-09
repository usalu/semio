//! 🖨️ The canonical printer: ASCII operators, canonical unit symbols and the fewest parentheses that re-parse to the same tree.
//!
//! [`print_with_spans`] also reports, for every node, the character span it occupies in the text so a UI can underline the node an
//! [`crate::ExprError`] points at by its path.

use super::{Span, KEYWORDS};
use crate::tree::{BinaryOp, Expr, UnaryOp};

/// 🖨️ The canonical text of an expression.
pub fn print(expr: &Expr) -> String {
    print_with_spans(expr).0
}

/// 🖍️ The canonical text and the span of every node, keyed by its child-index path from the root and sorted by path.
pub fn print_with_spans(expr: &Expr) -> (String, Vec<(Vec<usize>, Span)>) {
    let mut printer = Printer { out: String::new(), chars: 0, spans: Vec::new() };
    printer.node(expr, &mut Vec::new());
    printer.spans.sort_by(|a, b| a.0.cmp(&b.0));
    (printer.out, printer.spans)
}

fn binding_power(expr: &Expr) -> u8 {
    match expr {
        Expr::If(..) => 0,
        Expr::Or(..) => 10,
        Expr::And(..) => 20,
        Expr::Unary(UnaryOp::Not, _) => 30,
        Expr::Compare(..) => 40,
        Expr::Binary(BinaryOp::Add | BinaryOp::Subtract, ..) => 50,
        Expr::Binary(BinaryOp::Multiply | BinaryOp::Divide, ..) => 60,
        Expr::Unary(UnaryOp::Negate, _) => 70,
        Expr::Binary(BinaryOp::Power, ..) => 80,
        _ => 100,
    }
}

fn number(value: f64) -> String {
    if value != 0.0 && (value.abs() >= 1e16 || value.abs() < 1e-6) {
        format!("{value:e}")
    } else {
        format!("{value}")
    }
}

fn is_plain_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_alphabetic() || c == '_') && chars.all(|c| c.is_alphanumeric() || c == '_') && !KEYWORDS.contains(&name)
}

fn escaped(text: &str, quote: char) -> String {
    let mut out = String::new();
    out.push(quote);
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\n' if quote == '"' => out.push_str("\\n"),
            '\t' if quote == '"' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

struct Printer {
    out: String,
    chars: usize,
    spans: Vec<(Vec<usize>, Span)>,
}

impl Printer {
    fn push(&mut self, text: &str) {
        self.out.push_str(text);
        self.chars += text.chars().count();
    }

    fn child(&mut self, node: &Expr, index: usize, min_bp: u8, path: &mut Vec<usize>) {
        path.push(index);
        let wrap = binding_power(node) < min_bp;
        if wrap {
            self.push("(");
        }
        self.node(node, path);
        if wrap {
            self.push(")");
        }
        path.pop();
    }

    fn arguments(&mut self, name: &str, args: &[&Expr], path: &mut Vec<usize>) {
        self.push(name);
        self.push("(");
        for (index, arg) in args.iter().enumerate() {
            if index > 0 {
                self.push(", ");
            }
            self.child(arg, index, 0, path);
        }
        self.push(")");
    }

    fn node(&mut self, expr: &Expr, path: &mut Vec<usize>) {
        let start = self.chars;
        match expr {
            Expr::Number(v) => self.push(&number(*v)),
            Expr::Length(v, u) => self.push(&format!("{} {}", number(*v), u.symbol())),
            Expr::Angle(v, u) => self.push(&format!("{} {}", number(*v), u.symbol())),
            Expr::Area(v, u) => self.push(&format!("{} {}", number(*v), u.symbol())),
            Expr::Volume(v, u) => self.push(&format!("{} {}", number(*v), u.symbol())),
            Expr::Bool(v) => self.push(if *v { "true" } else { "false" }),
            Expr::Text(v) => self.push(&escaped(v, '"')),
            Expr::Param(name) if is_plain_name(name) => self.push(name),
            Expr::Param(name) => self.push(&escaped(name, '`')),
            Expr::Unary(UnaryOp::Negate, operand) => {
                self.push("-");
                self.child(operand, 0, 71, path);
            }
            Expr::Unary(UnaryOp::Not, operand) => {
                self.push("not ");
                self.child(operand, 0, 30, path);
            }
            Expr::Binary(op @ (BinaryOp::Min | BinaryOp::Max), left, right) => self.arguments(op.symbol(), &[left.as_ref(), right.as_ref()], path),
            Expr::Binary(op, left, right) => {
                let bp = binding_power(expr);
                let (left_bp, right_bp) = if *op == BinaryOp::Power { (bp + 1, bp) } else { (bp, bp + 1) };
                self.child(left, 0, left_bp, path);
                self.push(&format!(" {} ", op.symbol()));
                self.child(right, 1, right_bp, path);
            }
            Expr::Compare(op, left, right) => {
                self.child(left, 0, 41, path);
                self.push(&format!(" {} ", op.symbol()));
                self.child(right, 1, 41, path);
            }
            Expr::And(left, right) | Expr::Or(left, right) => {
                let (bp, word) = if matches!(expr, Expr::And(..)) { (20, " and ") } else { (10, " or ") };
                self.child(left, 0, bp, path);
                self.push(word);
                self.child(right, 1, bp + 1, path);
            }
            Expr::If(condition, then, otherwise) => {
                self.push("if ");
                self.child(condition, 0, 0, path);
                self.push(" then ");
                self.child(then, 1, 0, path);
                self.push(" else ");
                self.child(otherwise, 2, 0, path);
            }
            Expr::Call(function, args) => self.arguments(function.name(), &args.iter().collect::<Vec<_>>(), path),
        }
        self.spans.push((path.clone(), Span { start, end: self.chars }));
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
