//! 📜️ Block 5D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Block5dSnapshot;

/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block5d` DSL —
/// the `PartKind` slice of `s/plugin/puzzle/app/5d/example/🧩️concrete-forest.puzzle5d`.
pub const BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio");
/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block5d` DSL.
pub const BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio");

/// 📖️ Parses `.block5d` DSL text into a `Block5dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block5dSnapshot, store::TextError> {
    <Block5dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block5dSnapshot` back to `.block5d` DSL text.
pub fn print_dsl(document: &Block5dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
