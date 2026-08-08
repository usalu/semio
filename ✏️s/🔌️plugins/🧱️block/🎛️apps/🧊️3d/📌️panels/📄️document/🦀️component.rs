//! 📄️ Block 3D play app panel — the document tree: representation catalog + rim-vortex templates,
//! selectable.

use crate::apps::block3d::block3d_action;
use crate::apps::block3d::terminology::Block3dLabels;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};

//#region 🔖️Constants
pub const BLOCK3D_BODY_DOCUMENT: &str = "block3d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK3D_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(definition: &Block3dSnapshot, selected: &[String], labels: &Block3dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block3d-play-document");
    let representation_items: Vec<UiTreeItemNode> = definition
        .representations
        .iter()
        .map(|representation| UiTreeItemNode {
            icon_id: Some("box".into()),
            ..tree_item_with_action(builder.item_id("representation", &representation.id), Label::data(representation.name.clone()), representation.mesh_url.clone(), block3d_action("setSelection", None))
        })
        .collect();
    let vortex_items: Vec<UiTreeItemNode> = definition
        .vortices
        .iter()
        .map(|vortex| UiTreeItemNode { icon_id: Some("circle-dot".into()), ..tree_item_with_action(builder.item_id("vortex", &vortex.id), Label::data(vortex.vortex_kind.clone()), None, block3d_action("setSelection", None)) })
        .collect();
    builder
        .section_or_placeholder("block3d-play-document.representations", Some(labels.representations.into()), true, representation_items, labels.no_representations)
        .section_or_placeholder("block3d-play-document.vortices", Some(labels.vortices.into()), true, vortex_items, labels.no_vortices)
        .selected(selected.to_vec())
        .selection_change(block3d_action("setSelection", None))
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
