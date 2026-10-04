//! 🌐️ Trinity Jack viewer — the Graph window: a read-only node-graph render of the live
//! `JackSnapshot` projection, built from the same artifact-level pure `nodes()`/`edges()`
//! accessors and `port_node_id` helper the editor's own Graph window uses — this file itself
//! imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it
//! outright). No selection, no LOD toggle, no engagement: a viewer has no utilities that mutate
//! and emits no mutations by construction (`ViewEmit`).

use crate::JackSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphNode,SemioGraphPortKind,SemioGraphSnapshot};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::NodeGraphEdgeRecord;
use semio_framework_plugin::NodeGraphNodeRecord;
use semio_framework_plugin::NodeGraphPortRecord;
use semio_framework_plugin::NodeGraphScene;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_os_kernel::Viewport2d;
use semio_framework_ui_contract::SurfaceKind;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "trinity-jack-view-graph";
pub const BODY_KEY: &str = "trinity.jack.view.graph";
pub const SURFACE_ID: &str = "trinity.jack.view.graph";
/// 👁️ Read-only counterpart of the editor's `TRINITY_JACK_PLAY_CONTROLLER_ID` — kept distinct so a
/// viewer session's node-graph controller can never be mistaken for an editor session's.
const TRINITY_JACK_VIEW_CONTROLLER_ID: &str = "trinity-jack-view";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::jack::create_trinity_jack_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Nakagin Graph", "Nakagin-Graph"),
        body_key: BODY_KEY.into(),
        surface_kind: semio_framework_plugin::SurfaceKind::NodeGraph,
        icon_id: "graph-dag".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn node_to_record(node: &SemioGraphNode) -> NodeGraphNodeRecord {
    let width = if node.width > 0.0 { node.width } else { 96.0 };
    let height = if node.height > 0.0 { node.height } else { 48.0 };
    NodeGraphNodeRecord {
        id: node.id.value.clone(),
        label: Some(if node.label.is_empty() { node.id.value.clone() } else { node.label.clone() }),
        x: node.position.x,
        y: node.position.y,
        width,
        height,
        inputs: node.ports.iter().filter(|port| matches!(port.kind,SemioGraphPortKind::In|SemioGraphPortKind::InOut)).map(|port| NodeGraphPortRecord { id: port.name.clone(), label: Some(port.name.clone()), ..Default::default() }).collect(),
        outputs: node.ports.iter().filter(|port| matches!(port.kind,SemioGraphPortKind::Out|SemioGraphPortKind::InOut)).map(|port| NodeGraphPortRecord { id: port.name.clone(), label: Some(port.name.clone()), ..Default::default() }).collect(),
        ..Default::default()
    }
}

/// 👁️ Pure read of the parent and its published `content` child: no selection, no LOD, no query text — the viewer
/// renders the live graph exactly as it stands.
pub fn render(document: &JackSnapshot, child: &SemioGraphSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let nodes=child.nodes.iter().map(node_to_record).collect();
    let edges=child.edges.iter().map(|edge|NodeGraphEdgeRecord{id:edge.id.value.clone(),source_node_id:edge.source.value.clone(),source_port_id:edge.source_port.clone().unwrap_or_else(||"in".into()),target_node_id:edge.target.value.clone(),target_port_id:edge.target_port.clone().unwrap_or_else(||"in".into()),label:Some(edge.label.clone())}).collect();
    let viewport = Viewport2d { x: document.camera.x, y: document.camera.y, zoom: document.camera.zoom };
    let mut scene = NodeGraphScene { editable: Some(false), ..NodeGraphScene::base(nodes, edges, viewport) };
    scene.controls_json = Some(semio_framework_pack_json::json!({ "controllerId": TRINITY_JACK_VIEW_CONTROLLER_ID }).to_string());
    semio_framework_plugin::scene_surface(SURFACE_ID, SurfaceKind::NodeGraph, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
