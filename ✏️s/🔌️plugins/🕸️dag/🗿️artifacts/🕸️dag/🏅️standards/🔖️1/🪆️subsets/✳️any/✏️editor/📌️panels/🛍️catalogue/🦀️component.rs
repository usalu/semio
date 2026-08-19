//! 🛍️ DAG play app panel — the node-kind catalogue (drag/click-to-add palette).

use crate::editor::dag::terminology::DagPlayLabels;
use crate::editor::dag::dag_action;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const DAG_PLAY_BODY_CATALOGUE: &str = "dag.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), group: PanelGroup::Workbench, body_key: Some(DAG_PLAY_BODY_CATALOGUE.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(labels: &DagPlayLabels) -> UiNode {
    let kinds = [("computation", labels.kind_computation), ("slider", labels.kind_slider), ("select", labels.kind_select), ("screen", labels.kind_screen), ("note", labels.kind_note), ("preview", labels.kind_preview)];
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "dag-play-catalogue.node-kinds".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: kinds.iter().map(|(kind, label)| tree_item_with_action(format!("dag-play-catalogue.kind.{kind}"), *label, Some((*kind).into()), dag_action("addNode", Some(json!({ "kind": kind }))))).collect(),
        }],
        presence: UiPresence::default(),
        // 🕹️ A palette, not entity selection — declares no interaction domain (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); rows keep their plain per-item
        // `addNode` action above.
        interaction_domain: None,
        drop_action: None,
        menu: None,
    })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::dag::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_every_node_kind() {
        let mut app = new_app();
        let json = render_body(&mut app, DAG_PLAY_BODY_CATALOGUE);
        for kind in ["computation", "slider", "select", "screen", "note", "preview"] {
            assert!(json.contains(kind), "catalogue must list the {kind} kind: {json}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
