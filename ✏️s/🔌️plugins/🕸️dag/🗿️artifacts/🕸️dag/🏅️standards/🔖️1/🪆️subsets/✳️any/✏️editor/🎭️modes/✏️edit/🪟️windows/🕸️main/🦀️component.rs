//! 🕸️ DAG play app — the main window: the live node-graph canvas (dag's primary editing surface).

use crate::artifacts::dag::schema::document_to_workflow;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::terminology::DagPlayLabels;
use infinite_board_port_directed_dag::DagCamera;
use semio_framework_plugin::{build_node_graph_scene, LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const DAG_PLAY_WINDOW_MAIN: &str = "dag-main";
pub const DAG_PLAY_BODY_MAIN: &str = "dag.play.main";
const DAG_PLAY_SURFACE_MAIN: &str = "dag.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::dag::create_dag_app`. `options.measures` stays
/// empty here on purpose: this window has no chrome measures at all (no `🎚️options`).
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: DAG_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("DAG", "DAG"),
        body_key: DAG_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "graph-dag".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_dag_app`'s `.window_kind_interactions(..)` call — the
        // `graph` domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `render` carries no `InteractionView` (see `DagPlayApp::render`'s own doc comment) and
/// `NodeGraphScene` has no `interaction_domain` field the wrapper could stamp post-render either
/// (unlike `UiNode::Tree`) — `selection`/`hover` are left at `NodeGraphScene::base`'s defaults
/// (empty/none), matching `space`'s workflow window's identical gap.
pub async fn render(document: &DagSnapshot, camera: &DagCamera, _labels: &DagPlayLabels) -> UiNode {
    let (nodes, edges) = document_to_workflow(document);
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    build_node_graph_scene(DAG_PLAY_SURFACE_MAIN, crate::editor::dag::DAG_PLAY_APP_ID, NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::dag::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_node_graph_scene() {
        let mut app = new_app();
        let json = render_body(&mut app, DAG_PLAY_BODY_MAIN);
        assert!(json.contains("node-graph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, DAG_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
        assert!(definition.options.measures.is_empty(), "dag's main window has no chrome measures");
    }
}
//#endregion 🧪️Tests
