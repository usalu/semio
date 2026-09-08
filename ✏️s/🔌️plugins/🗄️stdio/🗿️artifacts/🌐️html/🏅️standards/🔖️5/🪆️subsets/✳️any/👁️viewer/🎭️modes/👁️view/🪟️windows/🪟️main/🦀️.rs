//! 👁️ `html` view (any) — Main window: real `TextWindowKit`
//! render of the current document (read-only).

use crate::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};
use store::ArtifactDsl;

pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    TextWindowKit::window_kind()
}

/// 📝️ The editable text buffer is the artifact's own DSL text envelope (`print_dsl`), not literal
/// markup — the same textual form `parse_dsl` accepts back on `replace-text` (see the sibling root
/// `handle`). Round-trips exactly for any document this format's own grammar can already print.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &HtmlSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    TextWindowKit::render(&TextView { text: snapshot.print_dsl(), language: Some("html".into()), read_only: true })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
