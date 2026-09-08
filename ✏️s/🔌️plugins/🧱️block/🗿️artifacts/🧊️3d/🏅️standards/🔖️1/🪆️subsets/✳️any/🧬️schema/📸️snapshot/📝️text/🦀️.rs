//! 📜️ Block 3D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Block3dSnapshot;

/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block3d` DSL — the `ObjectKind` half
/// of semio_compose_rs's metabolism-kit `Capsule` type.
pub const BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio");
/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block3d` DSL.
pub const BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio");

/// 📖️ Parses `.block3d` DSL text into a `Block3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block3dSnapshot, store::TextError> {
    <Block3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block3dSnapshot` back to `.block3d` DSL text.
pub fn print_dsl(document: &Block3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

/// 🚀️ The document every `block3d` surface boots with — `hexagonal-cut-concrete-forest-left`, the only
/// built-in example whose every representation `mesh_url` resolves against the delivery catalog
/// (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json` and its nested `🌱️metabolism` collection),
/// so the `World3d` surface renders a real mesh instead of an empty scene on first paint. Falls back to
/// the empty document if the embedded fixture ever stops parsing — a boot must never fault on a fixture.
pub fn block3d_boot_snapshot() -> Block3dSnapshot {
    parse_dsl(BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Block3dSnapshotText = String;
//#endregion 🚚️Carrier
