//! 🌐️ Trinity Jack viewer — the Graph window: a read-only node-graph render of the live
//! `JackSnapshot` projection, built from the same artifact-level pure `nodes()`/`edges()`
//! accessors and `port_node_id` helper the editor's own Graph window uses — this file itself
//! imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it
//! outright). No selection, no LOD toggle, no engagement: a viewer has no utilities that mutate
//! and emits no mutations by construction (`ViewEmit`).

use crate::JackSnapshot;

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
        initial_utility_id: None,
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
/// 👁️ Renders the original rich published child through its visual behavior.
pub fn render(document:&JackSnapshot,child:&dyn crate::JackContentVisual)->semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>{
 let(count_nodes,count_edges)=child.visual_counts();
 let nodes=(0..count_nodes).filter_map(|index|{
  let node=child.node_view(index)?;
  let ports=|inbound:bool|{(0..node.port_count).filter_map(|port_index|child.port_view(index,port_index)).filter(|port|if inbound{port.inbound}else{port.outbound}).map(|port|NodeGraphPortRecord{id:port.id.into(),label:Some(port.label.into()),..Default::default()}).collect()};
  let(width,height)=node.presented_extent();
  Some(NodeGraphNodeRecord{id:node.id.into(),label:Some(node.presented_label().into()),x:node.x,y:node.y,width,height,inputs:ports(true),outputs:ports(false),..Default::default()})
 }).collect();
 let edges=(0..count_edges).filter_map(|index|child.edge_view(index)).map(|edge|NodeGraphEdgeRecord{id:edge.id.into(),source_node_id:edge.source.into(),source_port_id:edge.source_port.unwrap_or("in").into(),target_node_id:edge.target.into(),target_port_id:edge.target_port.unwrap_or("in").into(),label:Some(edge.label.into())}).collect();
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
