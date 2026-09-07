//! 🕸️ DAG viewer — the main window: a read-only node-graph render of the current fixture, built from
//! the same subset-level `document_to_workflow` pure snapshot→scene helper the editor's own main
//! window (`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main`) uses — this file itself imports nothing from
//! the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No selection, no
//! drag, no add-node: a viewer has no utilities that edit and emits no mutations by construction
//! (`ViewEmit`).

use crate::artifacts::dag::schema::document_to_workflow;
use crate::artifacts::dag::DagSnapshot;
use semio_framework_plugin::{scene_surface, BuiltNode, UiAssemblyResult, LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const DAG_VIEW_WINDOW_MAIN: &str = "dag-view-main";
pub const BODY_KEY: &str = "dag.view.main";
const DAG_VIEW_SURFACE_MAIN: &str = "dag.view.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::dag::create_dag_viewer`. `options.measures`
/// stays empty here on purpose: this window has no chrome measures at all (no `🎚️options`), matching
/// the editor's own main window.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: DAG_VIEW_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("DAG", "DAG"),
        body_key: BODY_KEY.into(),
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
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `DagSnapshot -> UiAssemblyResult<BuiltNode>` read: default camera (a viewer has no persisted per-session
/// camera — `Config = NoConfig`), `editable: Some(false)` — the one bit that distinguishes this from
/// the editor's own main-window render.
pub fn render(document: &DagSnapshot) -> UiAssemblyResult<BuiltNode> {
    let (nodes, edges) = document_to_workflow(document);
    let viewport = NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 };
    scene_surface(DAG_VIEW_SURFACE_MAIN, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::NodeGraph, &NodeGraphScene { editable: Some(false), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BODY_KEY);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
        assert!(definition.options.measures.is_empty(), "dag's viewer main window has no chrome measures");
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_read_only_node_graph_scene() {
        let document = crate::artifacts::dag::default_snapshot();
        let node = render(&document).expect("viewer surface");
        let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = node.component else { panic!("viewer must produce a surface") };
        let scene: NodeGraphScene = semio_framework_ui_scene::decode(&props).expect("node graph scene");
        assert_eq!(scene.editable, Some(false));
        let (nodes, edges) = document_to_workflow(&document);
        assert_eq!(scene.nodes, nodes);
        assert_eq!(scene.edges, edges);
    }
}
//#endregion 🧪️Tests
