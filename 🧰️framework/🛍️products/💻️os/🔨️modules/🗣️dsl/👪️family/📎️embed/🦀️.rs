//! @emoji 📎️ `dsl_family_embed` — embed/island family kit for host-language fences.

pub use crate::os_dsl::notation::{print_edge, EdgeLabel, EdgeLink, EdgeNode, EdgeValue};

/// @emoji 🏷️ Parses `lang` id from a fence header line (` ```jack `).
pub async fn parse_fence_lang(header: &str) -> Option<String> {
    let trimmed = header.trim();
    let rest = trimmed.strip_prefix("```")?;
    let lang = rest.split_whitespace().next()?;
    (!lang.is_empty()).then(|| lang.to_string())
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    /// @emoji 📖️ The fragment's `.grammar` file must parse under `dsl_grammar`'s parser.
    #[semio_framework_async_macros::async_test]
    async fn grammar_file_is_syntactically_valid() {
        let source = include_str!("📖️.grammar.semio");
        let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-embed.grammar must parse");
        assert_eq!(grammar.id, "family-embed");
        assert!(grammar.productions.len() > 4, "family-embed should expose fence vocabulary beyond a one-liner");
    }
}
//#endregion 🔖️Tests
