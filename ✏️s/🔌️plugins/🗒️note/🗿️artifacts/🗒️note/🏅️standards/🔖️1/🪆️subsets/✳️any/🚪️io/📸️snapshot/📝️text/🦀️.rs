//! 📜️ Note artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::NoteSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The `semio` example document, handcrafted in the `.note` DSL.
pub const SEMIO_NOTE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.note` DSL text into a `NoteSnapshot`.
pub fn parse_dsl(text: &str) -> Result<NoteSnapshot, store::TextError> {
    <NoteSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `NoteSnapshot` back to `.note` DSL text.
pub fn print_dsl(document: &NoteSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🔖️ExternalBridges
/// 📖️ Parses `.note` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_note_dsl(text: &str) -> Result<NoteSnapshot, String> {
    <NoteSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`NoteSnapshot`] back to `.note` DSL text under a name an external caller can reach, paired
/// with [`parse_note_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_note_dsl(snapshot: &NoteSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges
