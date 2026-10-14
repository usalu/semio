//! 📄️ DOCX subset document window backed by the canonical base controls.

use semio_s_artifact_stdio_docx_base::editor::docx::standards::v_ecma_376::subsets::base::modes::edit::windows::main as canonical;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;
use semio_framework_plugin::app::{DocumentWindowKit, EditableDocumentPage, WindowKit};
use semio_framework_plugin::{BuiltNode, TreeWindows, WindowKindDefinition};
#[cfg(test)] use semio_framework_plugin::{UiValue};
use semio_framework_ui_locale::Locale;

pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
const CONTROLLER_ID: &str = "s.stdio.docx@ecma-376/strict#editor";

pub fn definition() -> WindowKindDefinition {
    canonical::definition()
}

fn editable_pages(document: &DocxSnapshot) -> semio_framework_plugin::UiAssemblyResult<Vec<EditableDocumentPage>> {
    canonical::editable_pages(document)
}

pub fn render(document: &DocxSnapshot, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    canonical::render_windowed_for_controller(document, &TreeWindows::unhosted(), Locale::En, CONTROLLER_ID, publication_revision)
}

pub fn render_windowed(document: &DocxSnapshot, windows: &TreeWindows<'_>, locale: Locale, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    canonical::render_windowed_for_controller(document, windows, locale, CONTROLLER_ID, publication_revision)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
