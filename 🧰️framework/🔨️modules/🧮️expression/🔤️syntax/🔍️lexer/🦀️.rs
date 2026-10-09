//! 🔍️ Splits expression text into tokens with character spans.

use super::{ParseError, ParseErrorKind};

/// 🧱️ One lexical element; units are plain identifiers until the parser sees them after a number.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Name(String),
    Text(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Degree,
}

/// 📍️ A token with the characters it covers.
#[derive(Clone, Debug, PartialEq)]
pub struct Lexed {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// ✂️ Tokenises the text, or reports the first character or literal that cannot be read.
pub fn lex(chars: &[char]) -> Result<Vec<Lexed>, ParseError> {
    let mut out = Vec::new();
    let mut i = 0;
    let at = |i: usize| chars.get(i).copied();
    while let Some(c) = at(i) {
        let start = i;
        let simple = |token: Token, len: usize| (token, len);
        let (token, len) = match c {
            c if c.is_whitespace() => {
                i += 1;
                continue;
            }
            '+' => simple(Token::Plus, 1),
            '-' | '−' => simple(Token::Minus, 1),
            '*' | '×' => simple(Token::Star, 1),
            '/' | '÷' => simple(Token::Slash, 1),
            '^' => simple(Token::Caret, 1),
            '(' => simple(Token::LParen, 1),
            ')' => simple(Token::RParen, 1),
            ',' => simple(Token::Comma, 1),
            '°' => simple(Token::Degree, 1),
            '≠' => simple(Token::Ne, 1),
            '≤' => simple(Token::Le, 1),
            '≥' => simple(Token::Ge, 1),
            '=' if at(i + 1) == Some('=') => simple(Token::Eq, 2),
            '=' => simple(Token::Eq, 1),
            '!' if at(i + 1) == Some('=') => simple(Token::Ne, 2),
            '<' if at(i + 1) == Some('=') => simple(Token::Le, 2),
            '<' => simple(Token::Lt, 1),
            '>' if at(i + 1) == Some('=') => simple(Token::Ge, 2),
            '>' => simple(Token::Gt, 1),
            '"' | '`' => {
                let (text, next) = quoted(chars, i)?;
                i = next;
                out.push(Lexed { token: if c == '"' { Token::Text(text) } else { Token::Name(text) }, start, end: i });
                continue;
            }
            c if c.is_ascii_digit() || (c == '.' && at(i + 1).is_some_and(|d| d.is_ascii_digit())) => {
                let (value, next) = number(chars, i)?;
                i = next;
                out.push(Lexed { token: Token::Number(value), start, end: i });
                continue;
            }
            c if is_ident_start(c) => {
                let mut j = i + 1;
                while at(j).is_some_and(is_ident_continue) {
                    j += 1;
                }
                i = j;
                out.push(Lexed { token: Token::Ident(chars[start..j].iter().collect()), start, end: j });
                continue;
            }
            found => return Err(ParseError::new(i, i + 1, ParseErrorKind::UnexpectedChar { found })),
        };
        i += len;
        out.push(Lexed { token, start, end: i });
    }
    Ok(out)
}

fn number(chars: &[char], start: usize) -> Result<(f64, usize), ParseError> {
    let digit = |i: usize| chars.get(i).is_some_and(char::is_ascii_digit);
    let mut i = start;
    while digit(i) {
        i += 1;
    }
    if chars.get(i) == Some(&'.') && digit(i + 1) {
        i += 1;
        while digit(i) {
            i += 1;
        }
    }
    if matches!(chars.get(i), Some('e' | 'E')) {
        let sign = usize::from(matches!(chars.get(i + 1), Some('+' | '-')));
        if digit(i + 1 + sign) {
            i += 1 + sign;
            while digit(i) {
                i += 1;
            }
        }
    }
    let text: String = chars[start..i].iter().collect();
    match text.parse::<f64>() {
        Ok(value) if value.is_finite() => Ok((value, i)),
        _ => Err(ParseError::new(start, i, ParseErrorKind::InvalidNumber { text })),
    }
}

fn quoted(chars: &[char], start: usize) -> Result<(String, usize), ParseError> {
    let quote = chars[start];
    let mut text = String::new();
    let mut i = start + 1;
    loop {
        match chars.get(i) {
            None if quote == '"' => return Err(ParseError::new(start, chars.len(), ParseErrorKind::UnterminatedText)),
            None => return Err(ParseError::new(start, chars.len(), ParseErrorKind::UnterminatedName)),
            Some(&c) if c == quote => break,
            Some('\\') => {
                let escaped = chars.get(i + 1).copied();
                match (escaped, quote) {
                    (Some('\\'), _) => text.push('\\'),
                    (Some('"'), '"') => text.push('"'),
                    (Some('`'), '`') => text.push('`'),
                    (Some('n'), '"') => text.push('\n'),
                    (Some('t'), '"') => text.push('\t'),
                    (Some(found), _) => return Err(ParseError::new(i, i + 2, ParseErrorKind::InvalidEscape { found })),
                    (None, _) => return Err(ParseError::new(start, chars.len(), if quote == '"' { ParseErrorKind::UnterminatedText } else { ParseErrorKind::UnterminatedName })),
                }
                i += 2;
            }
            Some(&c) => {
                text.push(c);
                i += 1;
            }
        }
    }
    if quote == '`' && text.is_empty() {
        return Err(ParseError::new(start, i + 1, ParseErrorKind::EmptyName));
    }
    Ok((text, i + 1))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
