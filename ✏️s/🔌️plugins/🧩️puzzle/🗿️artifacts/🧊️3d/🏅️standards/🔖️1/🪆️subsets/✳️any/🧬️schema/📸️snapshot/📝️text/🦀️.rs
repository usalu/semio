//! 🗣️ Puzzle 3d artifact — the textual `.puzzle3d` document grammar surface and its laws, plus the
//! two handcrafted example fixtures the play app's example picker loads.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Puzzle3dSnapshot;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle3d` DSL.
pub const PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle3d` DSL.
pub const PUZZLE3D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio");

/// 📖️ Parses `.puzzle3d` DSL text into a `Puzzle3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Puzzle3dSnapshot, store::TextError> {
    <Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle3dSnapshot` back to `.puzzle3d` DSL text.
pub fn print_dsl(document: &Puzzle3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Puzzle3dSnapshotText = String;
//#endregion 🚚️Carrier
