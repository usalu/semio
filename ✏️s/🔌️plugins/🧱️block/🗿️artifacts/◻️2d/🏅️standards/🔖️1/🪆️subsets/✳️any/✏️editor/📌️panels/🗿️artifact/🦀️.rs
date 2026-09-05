//! 📄️ Block 2D play app panel — the document tree: handle-kind catalog + rim-handle templates,
//! selectable.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::editor::block2d::terminology::Block2dLabels;
use crate::editor::block2d::{ui_label, ui_node_list, BLOCK2D_INTERACTION_HANDLE};
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK2D_BODY_DOCUMENT: &str = "block2d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK2D_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn icon_item(id: String, label: Label, description: Option<String>, icon: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut node = tree_item_desc(id, label, description)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "block2d tree icon admission failed"))?);
    }
    Ok(node)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `handleKind:{id}`/`handle:{id}` targets `Block2dPlayApp::interaction_topology` declares for the
/// `handle` domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology.
pub fn render(definition: &Block2dSnapshot, labels: &Block2dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let builder = PanelTreeBuilder::new("block2d-play-document")?;
    let handle_kind_items = ui_node_list(definition.handle_kinds.iter().map(|kind| icon_item(format!("handleKind:{}", kind.id), ui_label(&kind.label)?, Some(kind.color.clone()), "circle")))?;
    let handle_items = ui_node_list(definition.handles.iter().map(|handle| icon_item(format!("handle:{}", handle.id), ui_label(&handle.handle_kind)?, Some(format!("{:.2}", handle.angle)), "circle-dot")))?;
    builder
        .section_or_placeholder("block2d-play-document.handle-kinds", Some(ui_label(labels.handle_kinds.as_str())?), true, handle_kind_items, ui_label(labels.no_handle_kinds.as_str())?)?
        .section_or_placeholder("block2d-play-document.handles", Some(ui_label(labels.handles.as_str())?), true, handle_items, ui_label(labels.no_handles.as_str())?)?
        .interaction_domain(BLOCK2D_INTERACTION_HANDLE)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block2d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_document_tree() {
        let mut app = new_app();
        assert!(render_body(&mut app, BLOCK2D_BODY_DOCUMENT).contains("Handle Kinds"));
    }
}
//#endregion 🧪️Tests
