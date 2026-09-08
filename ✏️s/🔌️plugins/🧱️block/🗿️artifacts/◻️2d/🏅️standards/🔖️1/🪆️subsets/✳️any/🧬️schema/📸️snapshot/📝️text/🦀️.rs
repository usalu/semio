//! 📜️ Block 2D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Block2dSnapshot;

/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block2d` DSL —
/// the `NodeKind` half of `s/plugin/puzzle/app/2d/manifest/🔣️.json`.
pub const BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio");
/// 📄️ The `hexagonal-cut-concrete-forest-right` example fixture, handcrafted in the `.block2d` DSL.
pub const BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/➡️hexagonal-cut-concrete-forest-right/🖼️assets/➡️hexagonal-cut-concrete-forest-right/🗣️.dsl.semio");

/// 📖️ Parses `.block2d` DSL text into a `Block2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block2dSnapshot, store::TextError> {
    <Block2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block2dSnapshot` back to `.block2d` DSL text.
pub fn print_dsl(document: &Block2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
