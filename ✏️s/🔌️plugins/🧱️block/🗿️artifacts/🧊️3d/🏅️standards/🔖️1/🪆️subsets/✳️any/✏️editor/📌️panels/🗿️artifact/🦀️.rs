//! 📄️ Block 3D play app panel — the document tree: representation catalog + rim-vortex templates,
//! selectable.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::terminology::Block3dLabels;
use crate::editor::block3d::{ui_node_list, BLOCK3D_INTERACTION_VORTEX};
use semio_framework_plugin::{tree_item_desc, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK3D_BODY_DOCUMENT: &str = "block3d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
fn icon_item(id: String, label: Label, description: Option<String>, icon: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut node = tree_item_desc(id, label, description)?;
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
pub async fn render(definition: &Block3dSnapshot, labels: &Block3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let builder = PanelTreeBuilder::new("block3d-play-document")?;
    let representation_items = ui_node_list(definition.representations.iter().map(|representation| icon_item(format!("surface:{}", representation.id), Label::data(representation.name.clone()), representation.mesh_url.clone(), "box")))?;
    let vortex_items = ui_node_list(definition.vortices.iter().map(|vortex| icon_item(format!("vortex:{}", vortex.id), Label::data(vortex.vortex_kind.clone()), None, "circle-dot")))?;
    builder
        .section_or_placeholder("block3d-play-document.representations", Some(labels.representations.into()), true, representation_items, labels.no_representations)?
        .section_or_placeholder("block3d-play-document.vortices", Some(labels.vortices.into()), true, vortex_items, labels.no_vortices)?
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
