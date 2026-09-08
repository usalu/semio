//! 📜️ Remodeling artifact — the textual document grammar surface (`dsl`) and its laws.
//!
//! 🔀️ Deviation from every sibling plugin: remodeling ships no handcrafted `.remodeling` example fixture
//! file, so there is no `REMODELING_EXAMPLE_TEXT` constant here — its single "default" example is
//! generated at runtime from `default_remodeling_scene().print_dsl()` (see `create_remodeling_app`).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::RemodelingSnapshot;

/// 📖️ Parses `.remodeling` DSL text into a `RemodelingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RemodelingSnapshot, store::TextError> {
    <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RemodelingSnapshot` back to `.remodeling` DSL text.
pub fn print_dsl(scene: &RemodelingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(scene)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RemodelingSnapshotText = String;
//#endregion 🚚️Carrier
