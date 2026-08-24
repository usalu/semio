//! 📄️ Block 5D play app panel — the document tree: grip-kind catalog + rim-grip templates, selectable.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::editor::block5d::{ui_node_list, BLOCK5D_INTERACTION_GRIP};
use semio_framework_plugin::{tree_item_desc, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK5D_BODY_DOCUMENT: &str = "block5d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK5D_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn icon_item(id: String, label: Label, description: Option<String>, icon: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut node = tree_item_desc(id, label, description)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "block5d tree icon admission failed"))?);
    }
    Ok(node)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `gripKind:{id}`/`grip:{id}` targets `Block5dPlayApp::interaction_topology` declares for the `grip`
/// domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology.
pub async fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let builder = PanelTreeBuilder::new("block5d-play-document")?;
    let grip_kind_items = ui_node_list(definition.grip_kinds.iter().map(|kind| icon_item(format!("gripKind:{}", kind.id), Label::data(kind.label.clone()), Some(kind.color.clone()), "circle")))?;
    let grip_items = ui_node_list(definition.grips.iter().map(|grip| icon_item(format!("grip:{}", grip.id), Label::data(grip.grip_kind.clone()), Some(format!("{:.2}", grip.angle)), "circle-dot")))?;
    builder
        .section_or_placeholder("block5d-play-document.grip-kinds", Some(labels.grip_kinds.into()), true, grip_kind_items, labels.no_grip_kinds)?
        .section_or_placeholder("block5d-play-document.grips", Some(labels.grips.into()), true, grip_items, labels.no_grips)?
        .interaction_domain(BLOCK5D_INTERACTION_GRIP)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block5d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_document_tree() {
        let mut app = new_app();
        assert!(render_body(&mut app, BLOCK5D_BODY_DOCUMENT).contains("Grip Kinds"));
    }
}
//#endregion 🧪️Tests
