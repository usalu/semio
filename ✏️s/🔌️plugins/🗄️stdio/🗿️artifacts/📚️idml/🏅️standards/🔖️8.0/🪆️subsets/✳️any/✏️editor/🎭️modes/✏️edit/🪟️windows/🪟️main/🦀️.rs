//! 📄️ Idml story runs as addressed drafts: page is the story ordinal, item the run ordinal, revision the run text revision.

use semio_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_plugin::{BuiltNode, TreeWindows, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_stdio_idml_story::story_pages;
use semio_s_artifact_stdio_idml::IdmlSnapshot;

pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;

/// 🧱️ The kit's editable document window, relabelled for stories.
pub fn definition() -> WindowKindDefinition {
    let mut definition = DocumentWindowKit::editable_window_kind();
    definition.label = LocalizedLabel::native("Stories", "Geschichten");
    definition.icon_id = "file-text".into();
    definition
}

/// 📖️ One editable draft per story `Content` run.
pub fn editable_pages(snapshot: &IdmlSnapshot) -> Vec<EditableDocumentPage> {
    story_pages(snapshot).into_iter().enumerate().flat_map(|(page, runs)| runs.into_iter().enumerate().map(move |(item, run)| EditableDocumentPage::new(page as u32, item as u32, run.text))).collect()
}

pub fn render_windowed(snapshot: &IdmlSnapshot, windows: &TreeWindows<'_>, locale: Locale, publication_revision: semio_plugin::UiPublicationRevision) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(snapshot), publication_revision }, windows, locale)
}
