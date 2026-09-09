//! 📜️ `trinity.rewrite.rule` artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::RewritingSnapshot;
use store::ArtifactDsl;

/// 📄️ The bundled Nakagin `label-core` rewrite rule, handcrafted in the `.rewriting` DSL — mirrors the
/// `trinity-rewriting` app's own real default rule over a trimmed two-node/one-edge slice of the
/// bundled `🔱️nakagin-capsule-tower.trinity` before-fixture.
pub const NAKAGIN_LABEL_CORE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.rewriting` DSL text into a `RewritingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RewritingSnapshot, store::TextError> {
    <RewritingSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RewritingSnapshot` back to `.rewriting` DSL text.
pub fn print_dsl(document: &RewritingSnapshot) -> String {
    ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RewritingSnapshotText = String;
//#endregion 🚚️Carrier
