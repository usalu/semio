//! 📄️ DAG play app panel — the node/edge outline tree.

use crate::DagSnapshot;
use crate::editor::dag::terminology::DagPlayLabels;
use crate::editor::dag::DAG_PLAY_INTERACTION_DOMAIN;
use semio_framework_artifact_infinite_dag::dag_node_kind_tag;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const DAG_PLAY_BODY_DOCUMENT: &str = "dag.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(DAG_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME raw node/edge
/// ids `DagPlayApp::interaction_topology` registers for the `graph` domain — the framework stamps this
/// tree's selection/hover presence from that domain (`.interaction_domain`) and prunes stale ids
/// through that same topology, so no per-item click action is declared here anymore (clicks are
/// translated into `interactionSelect` generically)?.
pub fn render(document: &DagSnapshot, labels: &DagPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let scene = crate::dag_working_scene(document);
    let node_items = crate::editor::dag::ui_node_list(
        scene.nodes.iter().map(|node| tree_item_desc(node.id.clone(), if node.name.is_empty() { node.id.clone() } else { node.name.clone() }, Some(dag_node_kind_tag(&node.kind).into()))),
    )?;
    let edge_items = crate::editor::dag::ui_node_list(scene.edges.iter().map(|edge| tree_item_desc(edge.id.clone(), format!("{} → {}", edge.source, edge.target), Some(edge.id.clone()))))?;
    PanelTreeBuilder::new("dag-play-document")?
        .section_or_placeholder("dag-play-document.nodes", Some(semio_framework_plugin::plugin_app_close_prelude::Label::try_from(labels.nodes.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "outline heading admission failed"))?), true, node_items, labels.empty.as_str())?
        .section_or_placeholder("dag-play-document.edges", Some(semio_framework_plugin::plugin_app_close_prelude::Label::try_from(labels.edges.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "outline heading admission failed"))?), false, edge_items, labels.empty.as_str())?
        .interaction_domain(DAG_PLAY_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
