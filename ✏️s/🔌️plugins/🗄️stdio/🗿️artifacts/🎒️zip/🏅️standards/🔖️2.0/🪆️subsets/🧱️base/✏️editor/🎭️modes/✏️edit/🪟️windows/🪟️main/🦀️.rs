//! 🎒️ Windowed archive comment and entry-name drafts.

use crate::ZipSnapshot;
use semio_framework_plugin::app::{TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;

pub use crate::editor::editing::{COMMENT_NODE_ID, ENTRY_NODE_PREFIX, entry_node_id};
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::zip::base::create_zip_any_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    let mut definition = TreeWindowKit::editable_window_kind();
    definition.label = LocalizedLabel::native("Archive", "Archiv");
    definition.icon_id = "archive".into();
    if let Some(action) = definition.actions.iter_mut().find(|action| action.id == "set-node") {
        action.in_palette = false;
        action.args = vec![
            semio_framework_plugin::ActionArgDef::text("nodeId", LocalizedLabel::native("Entry", "Eintrag")).required(),
            semio_framework_plugin::ActionArgDef::text("value", LocalizedLabel::native("Name or comment", "Name oder Kommentar")).min_length(0).required(),
            semio_framework_plugin::ActionArgDef::text("revision", LocalizedLabel::native("Saved revision", "Gespeicherte Revision")).required(),
        ];
    }
    definition
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Shares the same localized, guarded editing controls across ZIP dialects.
pub fn render(document: &ZipSnapshot, windows: &TreeWindows<'_>, locale: semio_framework_plugin::Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    crate::editor::editing::render(document, windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
