//! 📄️ Idml viewer window: one read-only page per story, its runs joined by line breaks.

use semio_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_plugin::{BuiltNode, WindowKindDefinition};
use semio_ui_locale::LocalizedLabel;
use semio_s_artifact_stdio_idml_story::story_pages;
use semio_s_artifact_stdio_idml::IdmlSnapshot;

pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;

/// 🧱️ The kit's read-only document window, relabelled for stories.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Stories", "Geschichten"), icon_id: "file-text".into(), ..DocumentWindowKit::window_kind() }
}

/// 👁️ One page per story part.
pub fn render(snapshot: &IdmlSnapshot) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let pages = story_pages(snapshot).into_iter().map(|runs| DocumentPage { text: runs.into_iter().map(|run| run.text).collect::<Vec<_>>().join("\n") }).collect();
    DocumentWindowKit::render(&DocumentView { pages })
}
