//! 📄️ Note play app panel — the document tree: every block, with quick-add rows.

use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::{ui_label, NOTE_INTERACTION_BLOCKS, NOTE_INTERACTION_GRANULARITY, NOTE_PLAY_CONTROLLER_ID};
use crate::schema::{block_icon, block_kind, block_name, block_tree_row_id, block_visible};
use crate::{NoteBlockNode, NoteSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{
    tree_item_with_action, tree_window_item, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiFixedList, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_ARTIFACT: &str = "note.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(NOTE_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `note-play-block:{id}` targets `NotePlayApp::interaction_topology` declares for the "blocks"
/// domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
///
/// 🪟️ A `Group` nests through [`tree_window_item`], so every level of a recursively nested document
/// carries its own host window and stamps its own `total` — no level can overflow its parent's slots.
fn block_tree_item(block: &NoteBlockNode, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let row_id = block_tree_row_id(block);
    let item = ui::tree_item(ui_label(block_name(block))?)
        .try_id(&row_id)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note block id admission failed"))?
        .description(UiText::try_from_str(block_kind(block)).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block kind admission failed"))?)
        .icon(UiText::try_from_str(block_icon(block_kind(block))).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block icon admission failed"))?)
        .granularity(UiText::try_from_str(NOTE_INTERACTION_GRANULARITY).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block granularity admission failed"))?)
        .draggable(true)
        .dimmed(!block_visible(block));
    match block {
        NoteBlockNode::Group { children, .. } => tree_window_item(windows, item, &row_id, true, children, |child| block_tree_item(child, windows)),
        _ => item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note block row admission failed")),
    }
}

fn add_block_args(kind: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note action map admission failed"))?;
    let kind = UiText::try_from_str(kind).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block kind admission failed"))?;
    args.push("kind".into(), kind).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note action entry admission failed"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🧰️ The five quick-add rows live in their own small, fixed section so panel chrome never competes
/// with document content for the windowed section's slots (ticket
/// 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.1).
pub fn render(document: &NoteSnapshot, labels: &NotePlayLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut add_rows = UiFixedList::default();
    for (kind, label, icon) in [("text", labels.add_text, "type"), ("table", labels.add_table, "table-2"), ("math", labels.add_math, "note-math"), ("image", labels.add_image, "image"), ("group", labels.add_group, "folder-plus")] {
        let action = ActionFactory::new(NOTE_PLAY_CONTROLLER_ID).action("addBlock", Some(add_block_args(kind)?))?;
        let mut item = tree_item_with_action(format!("note-play-blocks.add.{kind}"), ui_label(label.as_str())?, None, action)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note add-block icon admission failed"))?);
        }
        add_rows.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note action row admission failed"))?;
    }
    PanelTreeBuilder::new("note-play-blocks")?
        .section("note-play-blocks.add", Some(ui_label(labels.artifact.as_str())?), true, add_rows)?
        .window_section_or_placeholder(windows, "note-play-blocks.blocks", Some(ui_label(labels.artifact.as_str())?), true, &document.blocks, |block| block_tree_item(block, windows), ui_label(labels.document_empty.as_str())?)?
        .interaction_domain(NOTE_PLAY_CONTROLLER_ID, NOTE_INTERACTION_BLOCKS)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
