//! @emoji 🗂️ `dsl_family_catalog` — the catalog family notation kit, shared by `block2d/3d/5d`,
//! `curation`, and `forms`. Two small literal forms these apps need that no other family does
//! (slash-paths, count literals), plus a re-export of `dsl_notation`'s edge grammar for "compat
//! pairs" (`b-l -- b-s`, an anonymous undirected edge — exactly what that grammar already is, no
//! catalog-specific extension needed).

pub use crate::os_dsl::notation::{print_edge, EdgeLabel, EdgeLink, EdgeNode, EdgeValue};

use crate::os_dsl::{lex, Limits, TextError, TextSpan, TokenKind};

//#region 🔖️SlashPath
/// @emoji 🌲️ Parses a slash-path ident (`beams/solid-timber/glulam`) into its segments. `/` is
/// `dsl_core` ident-continue, so a slash-path already lexes as ONE `Ident` token — this just
/// splits it, rejecting empty segments (`a//b`, a leading/trailing `/`) since those would silently
/// round-trip to a different-looking value.
pub async fn parse_slash_path_text(text: &str) -> Result<Vec<String>, TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let [token] = tokens.as_slice() else {
        return Err(TextError::new("expected a single slash-path ident", tokens.get(1).map_or(TextSpan::at(1, 1), |t| t.span)));
    };
    if token.kind != TokenKind::Ident {
        return Err(TextError::new(format!("expected an ident, found {:?}", token.kind), token.span));
    }
    let raw = token.text.as_str();
    let segments: Vec<String> = raw.split('/').map(str::to_string).collect();
    if segments.iter().any(|s| s.is_empty()) {
        return Err(TextError::new(format!("slash-path `{raw}` has an empty segment (leading/trailing/doubled `/`)"), token.span));
    }
    Ok(segments)
}

/// @emoji 🖨️ Canonical printer — the inverse of [`parse_slash_path_text`].
pub async fn print_slash_path(segments: &[String]) -> String {
    segments.join("/")
}
//#endregion 🔖️SlashPath

//#region 🔖️Count
/// @emoji 🔢️ Parses a count literal (`x24`) into its value. Lexes as one plain `Ident` token
/// (`x` is alphabetic, the digits are alphanumeric — nothing distinguishes it from any other ident
/// at the lexer level), so this checks the shape explicitly: a leading `x` followed by one or more
/// ASCII digits and nothing else.
pub async fn parse_count_text(text: &str) -> Result<u64, TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let [token] = tokens.as_slice() else {
        return Err(TextError::new("expected a single count literal", tokens.get(1).map_or(TextSpan::at(1, 1), |t| t.span)));
    };
    if token.kind != TokenKind::Ident {
        return Err(TextError::new(format!("expected an ident, found {:?}", token.kind), token.span));
    }
    let raw = token.text.as_str();
    let digits = raw.strip_prefix('x').filter(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()));
    let Some(digits) = digits else {
        return Err(TextError::new(format!("expected a count literal like `x24`, found `{raw}`"), token.span));
    };
    digits.parse().map_err(|_| TextError::new(format!("count `{raw}` overflows u64"), token.span))
}

/// @emoji 🖨️ Canonical printer — the inverse of [`parse_count_text`].
pub async fn print_count(n: u64) -> String {
    format!("x{n}")
}
//#endregion 🔖️Count

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
