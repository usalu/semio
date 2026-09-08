//! 📄️ Note play app panel — the document tree: every block, with quick-add rows.

use crate::schema::{block_icon, block_kind, block_name, block_tree_row_id, block_visible};
use crate::{NoteBlockNode, NoteSnapshot};
use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::{ui_label, NOTE_INTERACTION_BLOCKS, NOTE_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item, tree_item_desc, tree_item_with_action, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_DOCUMENT: &str = "note.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(NOTE_PLAY_BODY_DOCUMENT.into()),
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
fn block_tree_item(block: &NoteBlockNode) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let nested = match block {
        NoteBlockNode::Group { children, .. } => fixed_nodes(children.iter().map(block_tree_item))?,
        _ => UiFixedList::default(),
    };
    let mut node = tree_item_desc(block_tree_row_id(block), ui_label(block_name(block))?, Some(block_kind(block).into()))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str(block_icon(block_kind(block))).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block icon admission failed"))?);
        props.default_open = Some(matches!(block, NoteBlockNode::Group { .. }));
        props.draggable = Some(true);
        props.dimmed = Some(!block_visible(block));
    }
    for child in nested {
        node.children.try_push(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note nested block admission failed"))?;
    }
    Ok(node)
}

fn fixed_nodes(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut nodes = UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note node admission failed"))?;
    }
    Ok(nodes)
}

fn add_block_args(kind: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note action map admission failed"))?;
    let kind = UiText::try_from_str(kind).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block kind admission failed"))?;
    args.push("kind".into(), kind).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note action entry admission failed"))?;
    Ok(UiValue::Map(args.finish()))
}

pub fn render(document: &NoteSnapshot, labels: &NotePlayLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut items = UiFixedList::default();
    for (kind, label, icon) in [("text", labels.add_text, "type"), ("table", labels.add_table, "table-2"), ("math", labels.add_math, "note-math"), ("image", labels.add_image, "image"), ("group", labels.add_group, "folder-plus")] {
        let action = ActionFactory::new(NOTE_PLAY_CONTROLLER_ID).action("addBlock", Some(add_block_args(kind)?))?;
        let mut item = tree_item_with_action(format!("note-play-blocks.add.{kind}"), ui_label(label.as_str())?, None, action)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note add-block icon admission failed"))?);
        }
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note action row admission failed"))?;
    }
    if document.blocks.is_empty() {
        let mut item = tree_item("note-play-blocks.empty", ui_label(labels.document_empty.as_str())?)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(UiText::try_from_str("sticky-note").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note empty icon admission failed"))?);
        }
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note empty row admission failed"))?;
    } else {
        for block in &document.blocks {
            items.try_push(block_tree_item(block)?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note block list admission failed"))?;
        }
    }
    PanelTreeBuilder::new("note-play-blocks")?.section("note-play-blocks", Some(ui_label(labels.document.as_str())?), true, items)?.interaction_domain(NOTE_INTERACTION_BLOCKS)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
