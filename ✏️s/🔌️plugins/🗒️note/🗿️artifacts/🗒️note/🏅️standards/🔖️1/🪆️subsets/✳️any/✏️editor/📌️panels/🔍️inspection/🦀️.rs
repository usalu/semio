//! 🔍️ Note play app panel — the document-wide properties summary (schema, block count, active
//! utility, snap status).

use crate::schema::flatten_blocks;
use crate::NoteSnapshot;
use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::ui_label;
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, PluginAssemblyError, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_PROPERTIES: &str = "note.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(NOTE_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (a known SDK gap — matches `gis2d`'s inspection panel precedent), so this panel
/// can no longer tell which blocks are selected — it always shows the document-wide summary now; the
/// per-selected-block detail branch (name/x/y/width/height/visible/locked, driven by `patchBlocks`)
/// that used to read `cfg.selected_block_ids` is gone with it.
pub fn render(document: &NoteSnapshot, active_utility_id: &str, labels: &NotePlayLabels) -> UiAssemblyResult<BuiltNode> {
    let mut section = semio_framework_ui_contract::section(ui_label(labels.inspection.as_str())?).default_open(true).try_id("note-inspector").map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note inspector id admission failed"))?;
    for (index, value) in [
        format!("{}: {}", labels.summary_schema.as_str(), document.schema),
        format!("{}: {}", labels.summary_blocks.as_str(), flatten_blocks(&document.blocks).len()),
        format!("{}: {active_utility_id}", labels.summary_utility.as_str()),
        format!("{}: {}", labels.summary_snap.as_str(), if document.snap_enabled.unwrap_or(false) { format!("{}px", document.snap_grid_spacing.unwrap_or(8.0)) } else { labels.summary_off.as_str().into() }),
    ].into_iter().enumerate() {
        let child = semio_framework_ui_contract::text(ui_label(value)?).try_id(format!("note-inspector.summary.{index}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note summary key admission failed"))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note summary text admission failed"))?;
        section = section.try_child(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note inspector child admission failed"))?;
    }
    section.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note inspector node admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semantic-contract/🦀️.rs"]
mod semantic_contract;
