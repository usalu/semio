//! 📜️ S Home launcher artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::SHomeSnapshot;

/// 📦️ The `home` app's "default" example, embedded at compile time as handcrafted `.shome` DSL text —
/// exercised by the round-trip test below. Not yet wired into a `.example(...)` manifest registration
/// (the `home` UI manifest has none today).
pub const HOME_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.shome` DSL text into an `SHomeSnapshot`.
pub fn parse_dsl(text: &str) -> Result<SHomeSnapshot, store::TextError> {
    <SHomeSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `SHomeSnapshot` back to `.shome` DSL text.
pub fn print_dsl(document: &SHomeSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SHomeSnapshotText = String;
//#endregion 🚚️Carrier
