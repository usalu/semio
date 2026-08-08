//! 📄️ Block 5D play app panel — the document tree: grip-kind catalog + rim-grip templates, selectable.

use crate::apps::block5d::block5d_action;
use crate::apps::block5d::terminology::Block5dLabels;
use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};

//#region 🔖️Constants
pub const BLOCK5D_BODY_DOCUMENT: &str = "block5d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK5D_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(definition: &Block5dSnapshot, selected: &[String], labels: &Block5dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block5d-play-document");
    let grip_kind_items: Vec<UiTreeItemNode> = definition
        .grip_kinds
        .iter()
        .map(|kind| UiTreeItemNode { icon_id: Some("circle".into()), menu: None, ..tree_item_with_action(builder.item_id("grip-kind", &kind.id), Label::data(kind.label.clone()), Some(kind.color.clone()), block5d_action("setSelection", None)) })
        .collect();
    let grip_items: Vec<UiTreeItemNode> = definition
        .grips
        .iter()
        .map(|grip| UiTreeItemNode {
            icon_id: Some("circle-dot".into()),
            menu: None,
            ..tree_item_with_action(builder.item_id("grip", &grip.id), Label::data(grip.grip_kind.clone()), Some(format!("{:.2}", grip.angle)), block5d_action("setSelection", None))
        })
        .collect();
    builder
        .section_or_placeholder("block5d-play-document.grip-kinds", Some(labels.grip_kinds.into()), true, grip_kind_items, labels.no_grip_kinds)
        .section_or_placeholder("block5d-play-document.grips", Some(labels.grips.into()), true, grip_items, labels.no_grips)
        .selected(selected.to_vec())
        .selection_change(block5d_action("setSelection", None))
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
