//! 🔤️ The text syntax of expressions: a lexer, a Pratt parser with spans and readable errors, and the canonical printer.
//!
//! ```text
//! expr     = "if" expr "then" expr "else" expr | or
//! or       = and { "or" and }                      and = not { "and" not }
//! not      = "not" not | compare                   compare = sum [ ("=" | "!=" | "<" | "<=" | ">" | ">=") sum ]
//! sum      = product { ("+" | "-") product }       product = unary { ("*" | "/") unary }
//! unary    = "-" unary | power                     power = atom [ "^" unary-or-power ]   (right associative)
//! atom     = number [unit] | "text" | true | false | name | `quoted name` | function "(" expr {"," expr} ")" | "(" expr ")"
//! unit     = mm cm m km in ft | deg ° rad | mm2 cm2 m2 | mm3 cm3 m3 l
//! ```
//!
//! The printer writes the canonical form (ASCII operators, `deg`, `m2`, minimal parentheses, quoted names only when needed) and
//! `parse(print(e)) == e` holds for every [`crate::Expr::is_canonical`] tree. Spans count characters, not bytes.

#[path = "🔍️lexer/🦀️.rs"]
mod lexer;

#[path = "🌲️parser/🦀️.rs"]
mod parser;

#[path = "🖨️printer/🦀️.rs"]
mod printer;

pub use parser::parse;
pub use printer::{print, print_with_spans};

/// 📍️ A half-open range of characters in the source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// 🚨️ A syntax error with the span it points at.
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    pub span: Span,
    pub kind: ParseErrorKind,
}

/// 🧾️ What is wrong with the text.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseErrorKind {
    UnexpectedChar { found: char },
    InvalidNumber { text: String },
    UnterminatedText,
    UnterminatedName,
    EmptyName,
    InvalidEscape { found: char },
    UnexpectedToken { found: String, expected: &'static str },
    UnexpectedEnd { expected: &'static str },
    UnknownUnit { name: String },
    UnknownFunction { name: String },
    WrongArgumentCount { function: String, expected: usize, exact: bool, found: usize },
    ChainedComparison,
}

impl ParseError {
    /// 🏗️ An error at a span.
    pub fn new(start: usize, end: usize, kind: ParseErrorKind) -> Self {
        ParseError { span: Span { start, end }, kind }
    }

    /// 🏷️ The stable kebab-case code, the key of translations and fixtures.
    pub fn code(&self) -> &'static str {
        match self.kind {
            ParseErrorKind::UnexpectedChar { .. } => "unexpected-character",
            ParseErrorKind::InvalidNumber { .. } => "invalid-number",
            ParseErrorKind::UnterminatedText => "unterminated-text",
            ParseErrorKind::UnterminatedName => "unterminated-name",
            ParseErrorKind::EmptyName => "empty-name",
            ParseErrorKind::InvalidEscape { .. } => "invalid-escape",
            ParseErrorKind::UnexpectedToken { .. } => "unexpected-token",
            ParseErrorKind::UnexpectedEnd { .. } => "unexpected-end",
            ParseErrorKind::UnknownUnit { .. } => "unknown-unit",
            ParseErrorKind::UnknownFunction { .. } => "unknown-function",
            ParseErrorKind::WrongArgumentCount { .. } => "argument-count",
            ParseErrorKind::ChainedComparison => "chained-comparison",
        }
    }

    /// 💬️ The English fallback message.
    pub fn message(&self) -> String {
        match &self.kind {
            ParseErrorKind::UnexpectedChar { found } => format!("unexpected character `{found}`"),
            ParseErrorKind::InvalidNumber { text } => format!("`{text}` is not a finite number"),
            ParseErrorKind::UnterminatedText => "the text literal is missing its closing quote".to_string(),
            ParseErrorKind::UnterminatedName => "the quoted name is missing its closing backtick".to_string(),
            ParseErrorKind::EmptyName => "a quoted name cannot be empty".to_string(),
            ParseErrorKind::InvalidEscape { found } => format!("unknown escape `\\{found}`"),
            ParseErrorKind::UnexpectedToken { found, expected } => format!("found `{found}` where {expected} was expected"),
            ParseErrorKind::UnexpectedEnd { expected } => format!("the expression ends where {expected} was expected"),
            ParseErrorKind::UnknownUnit { name } => format!("`{name}` is not a unit; write an operator between a number and a name"),
            ParseErrorKind::UnknownFunction { name } => format!("unknown function `{name}`"),
            ParseErrorKind::WrongArgumentCount { function, expected, exact: true, found } => format!("`{function}` takes {expected} argument(s) but got {found}"),
            ParseErrorKind::WrongArgumentCount { function, expected, exact: false, found } => format!("`{function}` takes at least {expected} arguments but got {found}"),
            ParseErrorKind::ChainedComparison => "comparisons cannot be chained; combine them with `and`".to_string(),
        }
    }

    /// 🖍️ The message above the offending source line with carets under the span.
    pub fn render(&self, source: &str) -> String {
        let chars: Vec<char> = source.chars().collect();
        let start = self.span.start.min(chars.len());
        let line_start = chars[..start].iter().rposition(|c| *c == '\n').map_or(0, |i| i + 1);
        let line_end = chars[start..].iter().position(|c| *c == '\n').map_or(chars.len(), |i| start + i);
        let line: String = chars[line_start..line_end].iter().collect();
        let width = self.span.end.min(line_end).saturating_sub(start).max(1);
        format!("{}\n{}\n{}{}", self.message(), line, " ".repeat(start - line_start), "^".repeat(width))
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}..{}", self.message(), self.span.start, self.span.end)
    }
}

impl std::error::Error for ParseError {}

/// 🔑️ The words that cannot be written as a bare parameter name.
pub const KEYWORDS: &[&str] = &["if", "then", "else", "and", "or", "not", "true", "false"];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
