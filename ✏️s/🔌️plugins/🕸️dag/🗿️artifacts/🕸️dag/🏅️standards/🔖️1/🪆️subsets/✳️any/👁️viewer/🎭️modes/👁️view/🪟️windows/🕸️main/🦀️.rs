//! 🕸️ DAG viewer — the main window: a read-only node-graph render of the current fixture, built from
//! the same subset-level `document_to_workflow` pure snapshot→scene helper the editor's own main
//! window (`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main`) uses — this file itself imports nothing from
//! the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No selection, no
//! drag, no add-node: a viewer has no utilities that edit and emits no mutations by construction
//! (`ViewEmit`).

use crate::schema::document_to_workflow;
use crate::DagSnapshot;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
