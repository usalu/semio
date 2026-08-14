//! 📄️ Block 3D play app panel — the document tree: representation catalog + rim-vortex templates,
//! selectable.

use crate::apps::block3d::terminology::Block3dLabels;
use crate::apps::block3d::BLOCK3D_INTERACTION_VORTEX;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `surface:{id}`/`vortex:{id}` targets `Block3dPlayApp::interaction_topology` declares for the
/// `vortex` domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically).
pub fn render(definition: &Block3dSnapshot, labels: &Block3dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block3d-play-document");
    let representation_items: Vec<UiTreeItemNode> = definition
        .representations
        .iter()
        .map(|representation| UiTreeItemNode { icon_id: Some("box".into()), ..tree_item_desc(format!("surface:{}", representation.id), Label::data(representation.name.clone()), representation.mesh_url.clone()) })
        .collect();
    let vortex_items: Vec<UiTreeItemNode> = definition
        .vortices
        .iter()
        .map(|vortex| UiTreeItemNode { icon_id: Some("circle-dot".into()), ..tree_item_desc(format!("vortex:{}", vortex.id), Label::data(vortex.vortex_kind.clone()), None) })
        .collect();
    builder
        .section_or_placeholder("block3d-play-document.representations", Some(labels.representations.into()), true, representation_items, labels.no_representations)
        .section_or_placeholder("block3d-play-document.vortices", Some(labels.vortices.into()), true, vortex_items, labels.no_vortices)
        .interaction_domain(BLOCK3D_INTERACTION_VORTEX)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::block3d::testkit::{new_app, render as render_body};

    #[test]
    fn renders_document_tree() {
        let mut app = new_app();
        assert!(render_body(&mut app, BLOCK3D_BODY_DOCUMENT).contains("Representations"));
    }
}
//#endregion 🧪️Tests
