//! \u{1fa9f}️ Own PDF 1.4 page drafts, dimensions and structural actions.
use crate::editor::pdf14::page;
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::{TreeWindows, WindowKindDefinition, UiAssemblyResult};
use semio_framework_ui_contract::{BuiltNode, UiPublicationRevision};

pub const WINDOW_KIND_ID: &str = page::WINDOW_KIND_ID;
pub const BODY_KEY: &str = page::BODY_KEY;

/// \u{1fa9f}️ Declares the own page window's command vocabulary.
pub fn definition() -> WindowKindDefinition { page::window_definition() }

/// \u{1f4c4}️ Renders own resolved page targets with the caller's locale and publication.
pub fn render_windowed(document: &PdfSnapshot, revision: UiPublicationRevision, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    page::render_windowed(document, revision, windows, locale)
}

#[cfg(test)]
#[path = "\u{1f9ea}️tests/\u{1f52c}️unit/\u{1f980}️.rs"]
mod tests;
