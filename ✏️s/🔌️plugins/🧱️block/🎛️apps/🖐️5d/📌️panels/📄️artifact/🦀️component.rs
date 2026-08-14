//! 📄️ Block 5D play app panel — the document tree: grip-kind catalog + rim-grip templates, selectable.

use crate::apps::block5d::terminology::Block5dLabels;
use crate::apps::block5d::BLOCK5D_INTERACTION_GRIP;
use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK5D_BODY_DOCUMENT: &str = "block5d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `gripKind:{id}`/`grip:{id}` targets `Block5dPlayApp::interaction_topology` declares for the `grip`
/// domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology.
pub fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block5d-play-document");
    let grip_kind_items: Vec<UiTreeItemNode> = definition
        .grip_kinds
        .iter()
        .map(|kind| UiTreeItemNode { icon_id: Some("circle".into()), menu: None, ..tree_item_desc(format!("gripKind:{}", kind.id), Label::data(kind.label.clone()), Some(kind.color.clone())) })
        .collect();
    let grip_items: Vec<UiTreeItemNode> = definition
        .grips
        .iter()
        .map(|grip| UiTreeItemNode { icon_id: Some("circle-dot".into()), menu: None, ..tree_item_desc(format!("grip:{}", grip.id), Label::data(grip.grip_kind.clone()), Some(format!("{:.2}", grip.angle))) })
        .collect();
    builder
        .section_or_placeholder("block5d-play-document.grip-kinds", Some(labels.grip_kinds.into()), true, grip_kind_items, labels.no_grip_kinds)
        .section_or_placeholder("block5d-play-document.grips", Some(labels.grips.into()), true, grip_items, labels.no_grips)
        .interaction_domain(BLOCK5D_INTERACTION_GRIP)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::block5d::testkit::{new_app, render as render_body};

    #[test]
    fn renders_document_tree() {
        let mut app = new_app();
        assert!(render_body(&mut app, BLOCK5D_BODY_DOCUMENT).contains("Grip Kinds"));
    }
}
//#endregion 🧪️Tests
