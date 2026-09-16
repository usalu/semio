//! 🔍️ Note play app panel — the document-wide properties summary (schema, block count, active
//! utility, snap status).

use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::ui_label;
use crate::schema::flatten_blocks;
use crate::NoteSnapshot;
use semio_framework_plugin::{tree_item_desc, ui_node_list, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
    let snap = if document.snap_enabled.unwrap_or(false) {
        format!("{}px", document.snap_grid_spacing.unwrap_or(8.0))
    } else {
        labels.summary_off.as_str().into()
    };
    let items = ui_node_list([
        tree_item_desc("note-inspector.schema", ui_label(labels.summary_schema.as_str())?, Some(document.schema.clone())),
        tree_item_desc("note-inspector.blocks", ui_label(labels.summary_blocks.as_str())?, Some(flatten_blocks(&document.blocks).len().to_string())),
        tree_item_desc("note-inspector.utility", ui_label(labels.summary_utility.as_str())?, Some(active_utility_id.into())),
        tree_item_desc("note-inspector.snap", ui_label(labels.summary_snap.as_str())?, Some(snap)),
    ])?;
    PanelTreeBuilder::new("note-inspector")?
        .section("note-inspector.summary", Some(ui_label(labels.inspection.as_str())?), true, items)?
        .build()
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
