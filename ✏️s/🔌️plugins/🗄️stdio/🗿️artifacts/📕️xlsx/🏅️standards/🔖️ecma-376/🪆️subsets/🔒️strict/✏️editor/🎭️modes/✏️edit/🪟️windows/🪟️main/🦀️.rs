//! 📊️ Strict XLSX editor grid, backed by the shared canonical worksheet renderer.

use crate::XlsxSnapshot;
use semio_framework_plugin::app::{TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, TreeWindows, WindowKindDefinition};
use semio_framework_ui_locale::Locale;

pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;

pub fn definition() -> WindowKindDefinition {
    crate::editor::xlsx::standards::v_ecma_376::subsets::base::modes::edit::windows::main::definition()
}

pub fn render(document: &XlsxSnapshot, locale: Locale, windows: &TreeWindows<'_>, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    crate::editor::xlsx::standards::v_ecma_376::subsets::base::modes::edit::windows::main::render_for_controller(document, locale, windows, "s.stdio.xlsx@ecma-376/strict#editor", publication_revision)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
