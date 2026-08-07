//! @emoji 🎬️ `dsl_family_scene` — scene/layout family kit: layer stacks and shared edge notation.

pub use crate::os_dsl::notation::{print_edge, EdgeLabel, EdgeLink, EdgeNode, EdgeValue};

use crate::os_dsl::{lex, Limits, TextError, TokenKind};

/// @emoji 📐️ Parses `id@x y [z]` layer placement literals.
pub fn parse_layer_anchor_text(text: &str) -> Result<(String, f64, f64, Option<f64>), TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let id = tokens.first().ok_or_else(|| TextError::new("expected layer id", crate::os_dsl::TextSpan::at(1, 1)))?;
    if id.kind != TokenKind::Ident {
        return Err(TextError::new("expected layer id", id.span.clone()));
    }
    if tokens.get(1).map(|t| t.kind) != Some(TokenKind::At) {
        return Err(TextError::new("expected `@` after layer id", id.span.clone()));
    }
    let x = tokens.get(2).ok_or_else(|| TextError::new("expected x", id.span.clone()))?;
    let y = tokens.get(3).ok_or_else(|| TextError::new("expected y", id.span.clone()))?;
    if !matches!(x.kind, TokenKind::Float | TokenKind::Int) || !matches!(y.kind, TokenKind::Float | TokenKind::Int) {
        return Err(TextError::new("expected numeric x y", y.span.clone()));
    }
    let xf: f64 = x.text.as_str().parse().map_err(|_| TextError::new("bad x", x.span.clone()))?;
    let yf: f64 = y.text.as_str().parse().map_err(|_| TextError::new("bad y", y.span.clone()))?;
    let z = tokens.get(4).and_then(|t| {
        if matches!(t.kind, TokenKind::Float | TokenKind::Int) {
            t.text.as_str().parse().ok()
        } else {
            None
        }
    });
    Ok((id.text.as_str().to_string(), xf, yf, z))
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    /// @emoji 📖️ The fragment's `.grammar` file must parse under `dsl_grammar`'s parser.
    #[test]
    fn grammar_file_is_syntactically_valid() {
        let source = include_str!("📖️family-scene.grammar.semio");
        let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-scene.grammar must parse");
        assert_eq!(grammar.id, "family-scene");
        assert!(grammar.productions.len() > 4, "family-scene should expose a real shared vocabulary");
    }
}
//#endregion 🔖️Tests
