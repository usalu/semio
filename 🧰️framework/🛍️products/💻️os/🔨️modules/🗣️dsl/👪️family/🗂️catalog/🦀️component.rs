//! @emoji 🗂️ `dsl_family_catalog` — the catalog family notation kit, shared by `block2d/3d/5d`,
//! `curate`, and `forms`. Two small literal forms these apps need that no other family does
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
    let tokens: Vec<_> = lex(text, &limits, false).await?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let [token] = tokens.as_slice() else {
        return Err(TextError::new("expected a single slash-path ident", tokens.get(1).map(|t| t.span).unwrap_or(TextSpan::at(1, 1))));
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
    let tokens: Vec<_> = lex(text, &limits, false).await?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let [token] = tokens.as_slice() else {
        return Err(TextError::new("expected a single count literal", tokens.get(1).map(|t| t.span).unwrap_or(TextSpan::at(1, 1))));
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn parses_and_prints_a_slash_path() {
        let segments = parse_slash_path_text("beams/solid-timber/glulam").await.expect("parse_slash_path_text");
        assert_eq!(segments, vec!["beams".to_string(), "solid-timber".to_string(), "glulam".to_string()]);
        assert_eq!(print_slash_path(&segments).await, "beams/solid-timber/glulam");
    }

    #[semio_framework_async_macros::async_test]
    async fn single_segment_path_round_trips() {
        let segments = parse_slash_path_text("beams").await.expect("parse_slash_path_text");
        assert_eq!(segments, vec!["beams".to_string()]);
        assert_eq!(print_slash_path(&segments).await, "beams");
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_empty_segments() {
        assert!(parse_slash_path_text("a//b").await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_more_than_one_token() {
        assert!(parse_slash_path_text("a b").await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_and_prints_a_count_literal() {
        let n = parse_count_text("x24").await.expect("parse_count_text");
        assert_eq!(n, 24);
        assert_eq!(print_count(n).await, "x24");
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_a_non_count_ident() {
        let err = parse_count_text("beam").await.unwrap_err();
        assert!(err.message.contains("count literal"), "unexpected message: {}", err.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_bare_x_with_no_digits() {
        assert!(parse_count_text("x").await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn compat_pair_reuses_the_edge_grammar_directly() {
        let value = crate::os_dsl::notation::parse_edge_text("b-l--b-s").await.expect("parse_edge_text");
        assert_eq!(value.from, EdgeNode { id: "b-l".to_string(), kind: None, port: None });
        let printed = print_edge(&value).await;
        let link = value.link.expect("link");
        assert_eq!(link.directed, false);
        assert_eq!(link.to, EdgeNode { id: "b-s".to_string(), kind: None, port: None });
        assert_eq!(printed, "b-l--b-s");
    }

    /// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser.
    #[semio_framework_async_macros::async_test]
    async fn grammar_file_is_syntactically_valid() {
        let source = include_str!("📖️family-catalog.grammar.semio");
        let grammar = crate::os_dsl::grammar::parse_grammar(source).await.expect("family-catalog.grammar must parse");
        assert_eq!(grammar.id, "family-catalog");
        assert!(grammar.productions.len() > 5, "family-catalog should cover stock, slash-path, compat");
    }
}
//#endregion 🔖️Tests
