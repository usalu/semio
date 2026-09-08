//! 🗣️ Puzzle 5d artifact — the textual `.puzzle5d` document grammar surface and its laws, plus the
//! two handcrafted example fixtures the play app's example picker loads.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Puzzle5dSnapshot;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio");
pub const PUZZLE5D_CAPSULE_DREAM_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌙️capsule-dream/🖼️assets/🌙️dream/🗣️.dsl.semio");

/// 📖️ Parses `.puzzle5d` DSL text into a `Puzzle5dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Puzzle5dSnapshot, store::TextError> {
    <Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle5dSnapshot` back to `.puzzle5d` DSL text.
pub fn print_dsl(document: &Puzzle5dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Puzzle5dSnapshotText = String;
//#endregion 🚚️Carrier
