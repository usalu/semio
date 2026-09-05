//! 📄️ Block 3D play app panel — the document tree: representation catalog + rim-vortex templates,
//! selectable.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::terminology::Block3dLabels;
use crate::editor::block3d::{ui_label, ui_node_list, BLOCK3D_INTERACTION_VORTEX};
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK3D_BODY_DOCUMENT: &str = "block3d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK3D_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn icon_item(id: String, label: &str, description: Option<String>, icon: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut node = tree_item_desc(id, ui_label(label)?, description)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "block3d tree icon admission failed"))?);
    }
    Ok(node)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `surface:{id}`/`vortex:{id}` targets `Block3dPlayApp::interaction_topology` declares for the
/// `vortex` domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
pub fn render(definition: &Block3dSnapshot, labels: &Block3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let builder = PanelTreeBuilder::new("block3d-play-document")?;
    let representation_items =
        ui_node_list(definition.representations.iter().map(|representation| icon_item(format!("surface:{}", representation.id), &representation.name, representation.mesh_url.clone(), "box")))?;
    let vortex_items = ui_node_list(definition.vortices.iter().map(|vortex| icon_item(format!("vortex:{}", vortex.id), &vortex.vortex_kind, None, "circle-dot")))?;
    builder
        .section_or_placeholder("block3d-play-document.representations", Some(ui_label(labels.representations.as_str())?), true, representation_items, ui_label(labels.no_representations.as_str())?)?
        .section_or_placeholder("block3d-play-document.vortices", Some(ui_label(labels.vortices.as_str())?), true, vortex_items, ui_label(labels.no_vortices.as_str())?)?
        .interaction_domain(BLOCK3D_INTERACTION_VORTEX)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block3d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_document_tree() {
        let mut app = new_app();
        assert!(render_body(&mut app, BLOCK3D_BODY_DOCUMENT).contains("Representations"));
    }
}
//#endregion 🧪️Tests
