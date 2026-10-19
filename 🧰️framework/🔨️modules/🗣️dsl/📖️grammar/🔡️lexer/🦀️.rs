//! 🔡️ The grammar-format pre-scan lexer: `?` and `|` here, every other run delegated to the shared dsl lexer.

use semio_framework_diagnostic::{Limits, TextError, TextSpan};
use semio_framework_dsl_lexical::lexer::lex as core_lex;
use semio_framework_dsl_token::TokenKind as CoreKind;

//#region 🔖️Lexer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GKind {
    Ident,
    Int,
    Text,
    Star,
    Plus,
    Question,
    Pipe,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Equals,
    Newline,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GToken {
    pub kind: GKind,
    pub text: String,
    pub span: TextSpan,
}

/// 🔬️ Pre-scans `?`/`|` (not in `dsl_core`'s alphabet) and delegates every other run of
/// characters whole to `crate::lex`, exactly like `math::graph::crate::lex_spanned` does
/// for its own two Cypher-specific extras.
// 🚫️async: E1 pure — `core_lex`/`core_lex_with` (`🗣️dsl/🔍️lexer/🦀️.rs`) were reverted to
// sync per R9 (0 await, 0 io, measured directly); this wrapper's only suspension source is gone —
// see R9.
fn span_at(text: &str, offset: usize, length: u32) -> TextSpan {
    let line = text[..offset].matches('\n').count() as u32 + 1;
    let column = (offset - text[..offset].rfind('\n').map_or(0, |p| p + 1)) as u32 + 1;
    TextSpan::with_length(line, column, length)
}

pub fn lex(text: &str) -> Result<Vec<GToken>, TextError> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    fn push_segment(text: &str, seg_start: usize, seg_end: usize, tokens: &mut Vec<GToken>) -> Result<(), TextError> {
        if seg_end <= seg_start {
            return Ok(());
        }
        let segment = &text[seg_start..seg_end];
        let origin = span_at(text, seg_start, 0);
        let rebase = |span: TextSpan| {
            let line = origin.line + span.line - 1;
            let column = if span.line == 1 { origin.column + span.column - 1 } else { span.column };
            TextSpan::with_length(line, column, span.length)
        };
        let raw = core_lex(segment, &Limits::default(), false).map_err(|error| TextError { span: rebase(error.span), ..error })?;
        for token in raw {
            if matches!(token.kind, CoreKind::Whitespace | CoreKind::Comment | CoreKind::Eof) {
                continue;
            }
            let kind = match token.kind {
                CoreKind::Ident | CoreKind::Placeholder => GKind::Ident,
                CoreKind::Int => GKind::Int,
                CoreKind::Text => GKind::Text,
                CoreKind::Star => GKind::Star,
                CoreKind::Plus => GKind::Plus,
                CoreKind::LParen => GKind::LParen,
                CoreKind::RParen => GKind::RParen,
                CoreKind::LBrace => GKind::LBrace,
                CoreKind::RBrace => GKind::RBrace,
                CoreKind::LBracket => GKind::LBracket,
                CoreKind::RBracket => GKind::RBracket,
                CoreKind::Comma => GKind::Comma,
                CoreKind::Equals => GKind::Equals,
                CoreKind::Newline => GKind::Newline,
                other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("`.grammar` files cannot contain a {other:?} token here"), rebase(token.span))),
            };
            tokens.push(GToken { kind, text: token.text.as_str().to_string(), span: rebase(token.span) });
        }
        Ok(())
    }

    while i < bytes.len() {
        let c = bytes[i];
        if c == b'#' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if c == b';' {
            push_segment(text, seg_start, i, &mut tokens)?;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            seg_start = i;
            continue;
        }
        if c == b'"' {
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if c == b'?' || c == b'|' || c == b'/' {
            push_segment(text, seg_start, i, &mut tokens)?;
            let span = span_at(text, i, 1);
            tokens.push(GToken { kind: if c == b'?' { GKind::Question } else { GKind::Pipe }, text: (c as char).to_string(), span });
            i += 1;
            seg_start = i;
            continue;
        }
        if c == b'%' {
            let mut end = i + 1;
            while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'-' || bytes[end] == b'.') {
                end += 1;
            }
            if end > i + 1 {
                push_segment(text, seg_start, i, &mut tokens)?;
                let span = span_at(text, i, (end - i) as u32);
                tokens.push(GToken { kind: GKind::Ident, text: text[i..end].to_string(), span });
                i = end;
                seg_start = i;
                continue;
            }
        }
        i += 1;
    }
    push_segment(text, seg_start, bytes.len(), &mut tokens)?;
    let eof_span = tokens.last().map_or(TextSpan::at(1, 1), |t| t.span);
    tokens.push(GToken { kind: GKind::Eof, text: String::new(), span: eof_span });
    Ok(tokens)
}
//#endregion 🔖️Lexer
