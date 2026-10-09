//! 🌲️ A Pratt parser from tokens to [`Expr`]: binding powers instead of one function per precedence level.
//!
//! Binding powers, loosest first: `or` 10, `and` 20, `not` 30 (prefix), comparison 40 (not chainable), `+ -` 50, `* /` 60,
//! prefix `-` 70, `^` 80 (right associative). `if` is a prefix form whose `else` branch extends as far right as it can.

use super::lexer::{lex, Lexed, Token};
use super::{ParseError, ParseErrorKind, KEYWORDS};
use crate::tree::{AngleUnit, AreaUnit, BinaryOp, CompareOp, Expr, Function, LengthUnit, UnaryOp, VolumeUnit};

enum Infix {
    Or,
    And,
    Compare(CompareOp),
    Binary(BinaryOp),
}

const NOT_BP: u8 = 30;
const NEGATE_BP: u8 = 70;

fn infix(token: &Token) -> Option<(u8, u8, Infix)> {
    Some(match token {
        Token::Ident(word) if word == "or" => (10, 11, Infix::Or),
        Token::Ident(word) if word == "and" => (20, 21, Infix::And),
        Token::Eq => (40, 41, Infix::Compare(CompareOp::Equal)),
        Token::Ne => (40, 41, Infix::Compare(CompareOp::NotEqual)),
        Token::Lt => (40, 41, Infix::Compare(CompareOp::Less)),
        Token::Le => (40, 41, Infix::Compare(CompareOp::LessEqual)),
        Token::Gt => (40, 41, Infix::Compare(CompareOp::Greater)),
        Token::Ge => (40, 41, Infix::Compare(CompareOp::GreaterEqual)),
        Token::Plus => (50, 51, Infix::Binary(BinaryOp::Add)),
        Token::Minus => (50, 51, Infix::Binary(BinaryOp::Subtract)),
        Token::Star => (60, 61, Infix::Binary(BinaryOp::Multiply)),
        Token::Slash => (60, 61, Infix::Binary(BinaryOp::Divide)),
        Token::Caret => (80, 80, Infix::Binary(BinaryOp::Power)),
        _ => return None,
    })
}

fn is_comparison(token: &Token) -> bool {
    matches!(token, Token::Eq | Token::Ne | Token::Lt | Token::Le | Token::Gt | Token::Ge)
}

fn with_unit(value: f64, symbol: &str) -> Option<Expr> {
    LengthUnit::from_symbol(symbol)
        .map(|u| Expr::Length(value, u))
        .or_else(|| AngleUnit::from_symbol(symbol).map(|u| Expr::Angle(value, u)))
        .or_else(|| AreaUnit::from_symbol(symbol).map(|u| Expr::Area(value, u)))
        .or_else(|| VolumeUnit::from_symbol(symbol).map(|u| Expr::Volume(value, u)))
}

struct Parser<'a> {
    chars: &'a [char],
    tokens: Vec<Lexed>,
    pos: usize,
}

/// 📖️ Parses the text into an expression, or reports the first syntax error with its span.
pub fn parse(source: &str) -> Result<Expr, ParseError> {
    let chars: Vec<char> = source.chars().collect();
    let tokens = lex(&chars)?;
    let mut parser = Parser { chars: &chars, tokens, pos: 0 };
    let expr = parser.expr(0)?;
    match parser.peek() {
        None => Ok(expr),
        Some(_) => Err(parser.unexpected("an operator or the end of the expression")),
    }
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Lexed> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Lexed> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += usize::from(token.is_some());
        token
    }

    fn unexpected(&self, expected: &'static str) -> ParseError {
        match self.peek() {
            Some(t) => ParseError::new(t.start, t.end, ParseErrorKind::UnexpectedToken { found: self.chars[t.start..t.end].iter().collect(), expected }),
            None => ParseError::new(self.chars.len(), self.chars.len(), ParseErrorKind::UnexpectedEnd { expected }),
        }
    }

    fn expect(&mut self, token: &Token, expected: &'static str) -> Result<Lexed, ParseError> {
        match self.peek().filter(|t| &t.token == token).cloned() {
            Some(t) => {
                self.pos += 1;
                Ok(t)
            }
            None => Err(self.unexpected(expected)),
        }
    }

    fn expect_word(&mut self, word: &str, expected: &'static str) -> Result<(), ParseError> {
        self.expect(&Token::Ident(word.to_string()), expected).map(|_| ())
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut left = self.prefix()?;
        while let Some((lbp, rbp, kind)) = self.peek().and_then(|t| infix(&t.token)) {
            if lbp < min_bp {
                break;
            }
            self.pos += 1;
            let right = self.expr(rbp)?;
            left = match kind {
                Infix::Or => Expr::Or(Box::new(left), Box::new(right)),
                Infix::And => Expr::And(Box::new(left), Box::new(right)),
                Infix::Binary(op) => Expr::binary(op, left, right),
                Infix::Compare(op) => {
                    if let Some(t) = self.peek().filter(|t| is_comparison(&t.token)) {
                        return Err(ParseError::new(t.start, t.end, ParseErrorKind::ChainedComparison));
                    }
                    Expr::compare(op, left, right)
                }
            };
        }
        Ok(left)
    }

    fn prefix(&mut self) -> Result<Expr, ParseError> {
        let Some(Lexed { token, start, end }) = self.next() else {
            return Err(self.unexpected("an expression"));
        };
        match token {
            Token::Number(value) => self.literal(value),
            Token::Text(text) => Ok(Expr::Text(text)),
            Token::Name(name) => Ok(Expr::Param(name)),
            Token::Minus => Ok(Expr::unary(UnaryOp::Negate, self.expr(NEGATE_BP)?)),
            Token::LParen => {
                let inner = self.expr(0)?;
                self.expect(&Token::RParen, "`)`")?;
                Ok(inner)
            }
            Token::Ident(word) => self.word(word, start, end),
            _ => {
                self.pos -= 1;
                Err(self.unexpected("an expression"))
            }
        }
    }

    fn literal(&mut self, value: f64) -> Result<Expr, ParseError> {
        match self.peek().map(|t| (t.token.clone(), t.start, t.end)) {
            Some((Token::Degree, ..)) => {
                self.pos += 1;
                Ok(Expr::Angle(value, AngleUnit::Degree))
            }
            Some((Token::Ident(word), start, end)) if !KEYWORDS.contains(&word.as_str()) => match with_unit(value, &word) {
                Some(expr) => {
                    self.pos += 1;
                    Ok(expr)
                }
                None => Err(ParseError::new(start, end, ParseErrorKind::UnknownUnit { name: word })),
            },
            _ => Ok(Expr::Number(value)),
        }
    }

    fn word(&mut self, word: String, start: usize, end: usize) -> Result<Expr, ParseError> {
        match word.as_str() {
            "true" => Ok(Expr::Bool(true)),
            "false" => Ok(Expr::Bool(false)),
            "not" => Ok(Expr::unary(UnaryOp::Not, self.expr(NOT_BP)?)),
            "if" => {
                let condition = self.expr(0)?;
                self.expect_word("then", "`then`")?;
                let then = self.expr(0)?;
                self.expect_word("else", "`else`")?;
                Ok(Expr::conditional(condition, then, self.expr(0)?))
            }
            "and" | "or" | "then" | "else" => {
                self.pos -= 1;
                Err(self.unexpected("an expression"))
            }
            _ if self.peek().is_some_and(|t| t.token == Token::LParen) => self.call(&word, start, end),
            _ => Ok(Expr::Param(word)),
        }
    }

    fn call(&mut self, name: &str, start: usize, end: usize) -> Result<Expr, ParseError> {
        let variadic = matches!(name, "min" | "max");
        let function = Function::from_name(name);
        if !variadic && function.is_none() {
            return Err(ParseError::new(start, end, ParseErrorKind::UnknownFunction { name: name.to_string() }));
        }
        self.pos += 1;
        let mut args = Vec::new();
        if self.peek().is_some_and(|t| t.token == Token::RParen) {
            self.pos += 1;
        } else {
            loop {
                args.push(self.expr(0)?);
                if self.peek().is_some_and(|t| t.token == Token::Comma) {
                    self.pos += 1;
                    continue;
                }
                self.expect(&Token::RParen, "`,` or `)`")?;
                break;
            }
        }
        let close = self.tokens[self.pos - 1].end;
        let (expected, exact) = if variadic { (2, false) } else { (function.map_or(1, Function::arity), true) };
        if args.len() < expected || (exact && args.len() != expected) {
            return Err(ParseError::new(start, close, ParseErrorKind::WrongArgumentCount { function: name.to_string(), expected, exact, found: args.len() }));
        }
        let op = match name {
            "min" => Some(BinaryOp::Min),
            "max" => Some(BinaryOp::Max),
            _ => None,
        };
        match (op, function) {
            (Some(op), _) => Ok(args.into_iter().reduce(|a, b| Expr::binary(op, a, b)).unwrap_or(Expr::Number(0.0))),
            (None, Some(function)) => Ok(Expr::Call(function, args)),
            (None, None) => Err(ParseError::new(start, end, ParseErrorKind::UnknownFunction { name: name.to_string() })),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
