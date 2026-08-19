//! @emoji 🌍️ `dsl_family_geo` — geo/crs family kit for spatial artifacts.

pub use crate::os_dsl::notation::{print_edge, EdgeLabel, EdgeLink, EdgeNode, EdgeValue};

use crate::os_dsl::{lex, Limits, TextError, TokenKind};

/// @emoji 📍 Parses `lon lat [alt]` tuples.
pub async fn parse_point_text(text: &str) -> Result<(f64, f64, Option<f64>), TextError> {
    let limits = Limits::default();
    let nums: Vec<f64> = lex(text, &limits, false).await?
        .into_iter()
        .filter(|t| matches!(t.kind, TokenKind::Float | TokenKind::Int))
        .map(|t| t.text.as_str().parse().map_err(|_| TextError::new("bad number", t.span.clone())))
        .collect::<Result<_, _>>()?;
    if nums.len() < 2 {
        return Err(TextError::new("expected at least lon lat", crate::os_dsl::TextSpan::at(1, 1)));
    }
    Ok((nums[0], nums[1], nums.get(2).copied()))
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    /// @emoji 📖️ The fragment's `.grammar` file must parse under `dsl_grammar`'s parser.
    #[semio_framework_async_macros::async_test]
    async fn grammar_file_is_syntactically_valid() {
        let source = include_str!("📖️family-geo.grammar.semio");
        let grammar = crate::os_dsl::grammar::parse_grammar(source).await.expect("family-geo.grammar must parse");
        assert_eq!(grammar.id, "family-geo");
        assert!(grammar.productions.len() > 4, "family-geo should expose a real shared vocabulary");
    }
}
//#endregion 🔖️Tests
