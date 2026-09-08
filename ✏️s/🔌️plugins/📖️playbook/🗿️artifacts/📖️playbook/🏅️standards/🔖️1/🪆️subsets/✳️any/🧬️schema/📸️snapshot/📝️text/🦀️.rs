//! 📜️ Playbook artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::PlaybookSnapshot;

/// 📄️ The `facade-generator` example spec, handcrafted in the `.playbook` DSL.
pub const FACADE_GENERATOR_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.playbook` DSL text into a `PlaybookSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PlaybookSnapshot, store::TextError> {
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PlaybookSnapshot` back to `.playbook` DSL text.
pub fn print_dsl(document: &PlaybookSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
