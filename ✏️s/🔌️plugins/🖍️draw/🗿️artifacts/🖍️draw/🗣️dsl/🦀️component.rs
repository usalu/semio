//! 📜️ Draw artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::draw::DrawDocument;

/// 🗄️ The Semio emblem example fixture, handcrafted in `draw`'s DSL (`store::DocumentDsl`).
pub const SEMIO_DRAW_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🖍️semio.draw");

/// 📖️ Parses `.draw` DSL text into a `DrawDocument`.
pub fn parse_dsl(text: &str) -> Result<DrawDocument, store::TextError> {
    <DrawDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DrawDocument` back to `.draw` DSL text.
pub fn print_dsl(document: &DrawDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}
