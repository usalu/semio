//! 🌐️ Trinity Jack viewer — the Graph window: a read-only node-graph render of the live
//! `JackSnapshot` projection, built from the same artifact-level pure `nodes()`/`edges()`
//! accessors and `port_node_id` helper the editor's own Graph window uses — this file itself
//! imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it
//! outright). No selection, no LOD toggle, no engagement: a viewer has no utilities that mutate
//! and emits no mutations by construction (`ViewEmit`).

use crate::artifacts::jack::{JackSnapshot, Node, PortDirection};
use semio_framework_plugin::{LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, NodeGraphViewport, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::{Buildable, HasBase, SurfaceKind};

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
        surface_kind: SurfaceKind::NodeGraph,
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
/// 🩹 Read-only twin of the editor's `split_endpoint` — duplicated on purpose rather than imported
/// through the sibling `✏️editor` module, which `policyViewerPurityBreaches` forbids outright.
fn split_endpoint(endpoint: &str) -> (String, String) {
    crate::artifacts::jack::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
}

fn node_to_record(node: &Node) -> NodeGraphNodeRecord {
    let width = if node.width > 0.0 { node.width } else { 96.0 };
    let height = if node.height > 0.0 { node.height } else { 48.0 };
    NodeGraphNodeRecord {
        id: node.id.clone(),
        label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
        x: node.x,
        y: node.y,
        width,
        height,
        inputs: node.ports.iter().filter(|port| port.direction == PortDirection::In).map(|port| NodeGraphPortRecord { id: crate::artifacts::jack::port_key(&node.id, &port.id), label: Some(port.id.clone()), ..Default::default() }).collect(),
        outputs: node.ports.iter().filter(|port| port.direction == PortDirection::Out).map(|port| NodeGraphPortRecord { id: crate::artifacts::jack::port_key(&node.id, &port.id), label: Some(port.id.clone()), ..Default::default() }).collect(),
        ..Default::default()
    }
}

/// 👁️ Pure `JackSnapshot -> BuiltNode` read: no selection, no LOD, no query text — the viewer renders
/// the live fixture graph exactly as it stands.
pub fn render(document: &JackSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let nodes: Vec<NodeGraphNodeRecord> = document.nodes().iter().map(node_to_record).collect();
    let edges: Vec<NodeGraphEdgeRecord> = document
        .edges()
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    let viewport = NodeGraphViewport { x: document.camera.x, y: document.camera.y, zoom: document.camera.zoom };
    let mut scene = NodeGraphScene { editable: Some(false), ..NodeGraphScene::base(nodes, edges, viewport) };
    scene.controls_json = Some(pack::json!({ "controllerId": TRINITY_JACK_VIEW_CONTROLLER_ID }).to_string());
    let props = semio_framework_ui_scene::encode(SurfaceKind::NodeGraph, &scene).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.scene.encode", "Trinity node-graph scene admission failed"))?;
    semio_framework_ui_contract::surface(props)
        .try_id(SURFACE_ID)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.scene.id", "Trinity node-graph surface id admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.scene.build", "Trinity node-graph surface admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_node_graph_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::NodeGraph);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::jack::empty_trinity_graph_fixture();
        let node = render(&document);
        assert!(pack::to_json_string(&node).contains("node-graph"));
    }
}
//#endregion 🧪️Tests
