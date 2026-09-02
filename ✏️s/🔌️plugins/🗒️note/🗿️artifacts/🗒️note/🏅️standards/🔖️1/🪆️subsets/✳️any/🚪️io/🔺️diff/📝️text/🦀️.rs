//! 🔺️ Note diff — grammar spec asset only. `NoteDiff` has no `impl store::ArtifactDsl`/`ArtifactPack`
//! anywhere in this plugin — this facet exists purely to register `note.diff`'s handcrafted text
//! grammar for LSP/verification tooling via `io()`'s `LanguageSpec` (design.md §2's `LanguagePair`
//! doc: "a subset with no hand-authored grammar for a channel still owns that channel's codec, just
//! with no `.grammar.semio` registered" — the inverse case is equally legal: a registered grammar
//! with no literal runtime parser backing it). The real apply/absorb/builder logic that used to live
//! here moved to `🧬️schema/🔺️diff/🦀️.rs` (pure snapshot-algebra transforms, design.md rule 2).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}
