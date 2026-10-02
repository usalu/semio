//! ✏️ `html` edit (any) — Main window: real `TextWindowKit`
//! render of the current document (editable variant).

use crate::standards::v5::subsets::any::schema::snapshot::{write_html_document, HtmlSnapshot};
use semio_framework_plugin::app::{TextEditView, TextWindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowKit;

pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    TextWindowKit::editable_window_kind()
}

/// 📝️ The editable text buffer is natural HTML source, edited as the kit's explicit draft (markup: edited locally, ONE
/// `textEdit` on Apply); `parse_dsl` accepts the same source directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &HtmlSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    TextWindowKit::render_editable(&TextEditView { surface_id: WINDOW_KIND_ID.into(), text: write_html_document(snapshot), language: Some("html".into()), revision: None }, locale)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
