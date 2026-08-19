//! 🔤️ Trinity jack lexer.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TokenClass {
    Keyword,
    Ident,
    Number,
    String,
    Operator,
    Punctuation,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSpan {
    pub class: TokenClass,
    pub start: usize,
    pub end: usize,
}

/// 🔤️ Fine-grained jack token kind, shared with editor consumers that need exact keyword/punctuation
/// identity rather than the coarse [`TokenClass`] used for syntax highlighting.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    KwMatch,
    KwWhere,
    KwReturn,
    KwCreate,
    KwDelete,
    KwSet,
    KwMerge,
    Ident(String),
    Number(f64),
    StringLit(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,
    Eq,
    Ne,
    Dash,
    Arrow,
    And,
    Or,
    Eof,
}

/// 🔤️ A [`Token`] with its source byte span.
#[derive(Clone, Debug, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}

async fn token_class(token: &Token) -> TokenClass {
    match token {
        Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::And | Token::Or => TokenClass::Keyword,
        Token::Ident(_) => TokenClass::Ident,
        Token::Number(_) => TokenClass::Number,
        Token::StringLit(_) => TokenClass::String,
        Token::Eq | Token::Ne | Token::Dash | Token::Arrow => TokenClass::Operator,
        Token::LParen | Token::RParen | Token::LBracket | Token::RBracket | Token::Colon | Token::Comma | Token::Dot => TokenClass::Punctuation,
        Token::Eof => TokenClass::Punctuation,
    }
}

async fn push_spanned(tokens: &mut Vec<SpannedToken>, token: Token, start: usize, end: usize) {
    tokens.push(SpannedToken { token, start, end });
}

/// 🔤️ Byte-span-tracked jack lexer. `forgiving = true` never fails (used by live editors mid-keystroke);
/// `forgiving = false` rejects unterminated strings and unrecognized characters.
pub async fn lex_spanned(input: &str, forgiving: bool) -> Result<Vec<SpannedToken>, String> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        match c {
            b'(' => {
                push_spanned(&mut tokens, Token::LParen, start, start + 1);
                i += 1;
            }
            b')' => {
                push_spanned(&mut tokens, Token::RParen, start, start + 1);
                i += 1;
            }
            b'[' => {
                push_spanned(&mut tokens, Token::LBracket, start, start + 1);
                i += 1;
            }
            b']' => {
                push_spanned(&mut tokens, Token::RBracket, start, start + 1);
                i += 1;
            }
            b':' => {
                push_spanned(&mut tokens, Token::Colon, start, start + 1);
                i += 1;
            }
            b',' => {
                push_spanned(&mut tokens, Token::Comma, start, start + 1);
                i += 1;
            }
            b'.' => {
                push_spanned(&mut tokens, Token::Dot, start, start + 1);
                i += 1;
            }
            b'!' if i + 1 < bytes.len() && bytes[i + 1] == b'=' => {
                push_spanned(&mut tokens, Token::Ne, start, start + 2);
                i += 2;
            }
            b'=' => {
                push_spanned(&mut tokens, Token::Eq, start, start + 1);
                i += 1;
            }
            b'\'' | b'"' => {
                let quote = c;
                i += 1;
                let lit_start = i;
                while i < bytes.len() && bytes[i] != quote {
                    i += 1;
                }
                if i >= bytes.len() {
                    if forgiving {
                        let s = String::from_utf8_lossy(&bytes[lit_start..i]).into_owned();
                        push_spanned(&mut tokens, Token::StringLit(s), start, i);
                        break;
                    }
                    return Err("unterminated string".into());
                }
                let s = String::from_utf8_lossy(&bytes[lit_start..i]).into_owned();
                i += 1;
                push_spanned(&mut tokens, Token::StringLit(s), start, i);
            }
            b'-' if i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                push_spanned(&mut tokens, Token::Arrow, start, start + 2);
                i += 2;
            }
            b'0'..=b'9' | b'-' if i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() => {
                let num_start = i;
                if bytes[i] == b'-' {
                    i += 1;
                }
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }
                let num: f64 = match std::str::from_utf8(&bytes[num_start..i]) {
                    Ok(s) => match s.parse() {
                        Ok(n) => n,
                        Err(_e) if forgiving => {
                            push_spanned(&mut tokens, Token::Ident(s.to_string()), num_start, i);
                            continue;
                        }
                        Err(e) => return Err(e.to_string()),
                    },
                    Err(_e) if forgiving => {
                        push_spanned(&mut tokens, Token::Ident(String::new()), num_start, i);
                        continue;
                    }
                    Err(e) => return Err(e.to_string()),
                };
                push_spanned(&mut tokens, Token::Number(num), num_start, i);
            }
            b'-' => {
                push_spanned(&mut tokens, Token::Dash, start, start + 1);
                i += 1;
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let word = std::str::from_utf8(&bytes[start..i]).unwrap().to_ascii_uppercase();
                let tok = match word.as_str() {
                    "MATCH" => Token::KwMatch,
                    "WHERE" => Token::KwWhere,
                    "RETURN" => Token::KwReturn,
                    "CREATE" => Token::KwCreate,
                    "DELETE" => Token::KwDelete,
                    "SET" => Token::KwSet,
                    "MERGE" => Token::KwMerge,
                    "AND" => Token::And,
                    "OR" => Token::Or,
                    _ => Token::Ident(std::str::from_utf8(&bytes[start..i]).unwrap().to_string()),
                };
                push_spanned(&mut tokens, tok, start, i);
            }
            _ if forgiving => {
                push_spanned(&mut tokens, Token::Ident(String::from(c as char)), start, start + 1);
                i += 1;
            }
            _ => return Err(format!("unexpected char {}", c as char)),
        }
    }
    push_spanned(&mut tokens, Token::Eof, input.len(), input.len());
    Ok(tokens)
}

pub async fn lex(input: &str) -> Result<Vec<Token>, String> {
    lex_spanned(input, false).map(|spanned| spanned.into_iter().map(|row| row.token).collect())
}

/// 🎨️ Tokenize jack source for editor highlighting (never fails).
pub async fn tokenize(input: &str) -> Vec<TokenSpan> {
    lex_spanned(input, true)
        .unwrap_or_default()
        .into_iter()
        .filter(|row| !matches!(row.token, Token::Eof))
        .map(|row| {
            let mut class = token_class(&row.token);
            if matches!(row.token, Token::StringLit(_)) {
                let quote = input.as_bytes().get(row.start);
                if quote == Some(&b'\'') || quote == Some(&b'"') {
                    let closed = input.as_bytes().get(row.end.saturating_sub(1)) == quote;
                    if !closed {
                        class = TokenClass::Error;
                    }
                }
            }
            TokenSpan { class, start: row.start, end: row.end }
        })
        .collect()
}
// #endregion 🔖️Lexer
