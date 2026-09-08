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
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;
