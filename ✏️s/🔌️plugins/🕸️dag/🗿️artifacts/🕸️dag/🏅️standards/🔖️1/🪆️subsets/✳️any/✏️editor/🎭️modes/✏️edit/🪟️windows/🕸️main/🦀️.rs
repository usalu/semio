//! 🕸️ DAG play app — the main window: the live node-graph canvas (dag's primary editing surface).

use crate::editor::dag::terminology::DagPlayLabels;
use crate::schema::document_to_workflow;
use crate::DagSnapshot;
use semio_framework_artifact_infinite_dag::DagCamera;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, NodeGraphScene, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_os_kernel::Viewport2d;

//#region 🔖️Constants
pub const DAG_PLAY_WINDOW_MAIN: &str = "dag-main";
pub const DAG_PLAY_BODY_MAIN: &str = "dag.play.main";
const DAG_PLAY_SURFACE_MAIN: &str = "dag.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::dag::create_dag_app`. `options.measures` stays
/// empty here on purpose: this window has no chrome measures at all (no `🎚️options`).
pub fn definition() -> WindowKindDefinition {
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
pub fn render(document: &DagSnapshot, camera: &DagCamera, _labels: &DagPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let (nodes, edges) = document_to_workflow(document);
    let viewport = Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom };
    scene_surface(DAG_PLAY_SURFACE_MAIN, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::NodeGraph, &NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
