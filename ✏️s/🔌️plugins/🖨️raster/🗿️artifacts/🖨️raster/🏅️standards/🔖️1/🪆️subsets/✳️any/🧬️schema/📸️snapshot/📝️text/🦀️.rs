//! 📜️ Raster artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::RasterSnapshot;

/// 📄️ The `semio` example document, handcrafted in the `.raster` DSL.
pub const SEMIO_RASTER_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.raster` DSL text into a `RasterSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RasterSnapshot, store::TextError> {
    <RasterSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RasterSnapshot` back to `.raster` DSL text.
pub fn print_dsl(document: &RasterSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RasterSnapshotText = String;
//#endregion 🚚️Carrier
