//! ✏️ `html` edit (any) — Main window: real `TextWindowKit`
//! render of the current document (editable variant).

use crate::standards::v5::subsets::any::schema::snapshot::{write_html_document, HtmlSnapshot};
use semio_framework_plugin::app::{TextView, TextWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    TextWindowKit::editable_window_kind()
}

/// 📝️ The editable text buffer is natural HTML source; `parse_dsl` accepts the same source directly
/// when the renderer commits `textEdit`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &HtmlSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    TextWindowKit::render(&TextView { text: write_html_document(snapshot), language: Some("html".into()), read_only: false })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
