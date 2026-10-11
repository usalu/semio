//! 🗂️ Edit-mode tool — Reorganize: the DAG's layered layout (`DagHost::reorganize`, computed once per run) reached
//! through the framework's stepped layout run (`semio-framework-graph-layout-run`). Every node is anchored to its
//! layered target, so each iteration visibly pulls the committed graph toward the layered one: node moves are
//! provisional graph `move-node` ops of the composed `content` child (a child-target run, design §12, §20.15) the canvas
//! renders from the ToolRun overlay composed on read, every node carries an entity trace verdict, and finalize publishes
//! ONE `content` member edit with one move per moved node (`📋️tool-run-contract.md` §3.7, `📓️wave-W3-F.md` §3.6).

use crate::editor::dag::config::{dag_config_camera, DagConfig};
use crate::{DagScene, SemioGraphMutation};
use infinite_canvas::board::schema::layout::{DagLayoutOptions};
use infinite_board_port_directed_dag::{DagHost};
use semio_framework_artifact_infinite_dag::{dag_host_snapshot_from_document, DagHostSnapshot, DAG_DOCUMENT_SCHEMA};
use semio_framework_graph_layout_run::{layout_run_definition, layout_run_entity, layout_run_job, layout_run_overlay_positions, LayoutRunConfig, LayoutRunEdge, LayoutRunEncodeError, LayoutRunGraph, LayoutRunNode, LayoutRunOpEncoder, LayoutRunPoint, LayoutRunResume, LayoutRunSpringLaw};
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ToolDefinition;
use semio_framework_plugin::ToolRunJob;
use semio_framework_tool_run::{JobKindId, ToolRunIdentity};
use std::collections::{BTreeSet, HashMap};

//#region 🔖️Constants
pub const TOOL_ID: &str = "reorganize";
/// 🧵️ Job kind of the layout run (`ToolRunDefinition.runJob`).
pub const LAYOUT_RUN_JOB: &str = "dag.dag.layoutRun";
/// 🧩️ The composed-child slot the run edits.
pub const LAYOUT_RUN_MEMBER: &str = "content";
//#endregion 🔖️Constants

//#region 🔖️Faults
/// 📢️ The localized notices of the run's refusal codes (design §20.12).
pub fn layout_run_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 4]> = std::sync::LazyLock::new(|| {
        [
            ("dag.layout-run.layered-load", LocalizedLabel::native("The graph could not be prepared for the layered layout.", "Der Graph konnte nicht für die Schichtanordnung vorbereitet werden.")),
            ("dag.layout-run.layered-layout", LocalizedLabel::native("The layered layout could not be computed for this graph.", "Die Schichtanordnung konnte für diesen Graphen nicht berechnet werden.")),
            ("dag.layout-run.layered-result", LocalizedLabel::native("The layered layout returned no usable positions.", "Die Schichtanordnung lieferte keine verwendbaren Positionen.")),
            ("dag.layout-run.start", LocalizedLabel::native("The reorganize run could not start.", "Der Neuanordnungslauf konnte nicht starten.")),
        ]
    });
    NOTICES.as_slice()
}

/// 🚫️ A named refusal of the layout run, its detail kept for diagnostics.
fn layout_fault(code: &'static str, detail: String) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), detail)
}
//#endregion 🔖️Faults

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::dag::create_dag_app`; the run vocabulary and its EN/DE labels
/// are the shared layout run's.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(semio_framework_tool_run::ToolRunDefinition { member: Some(LAYOUT_RUN_MEMBER.into()), ..layout_run_definition(JobKindId::new(LAYOUT_RUN_JOB)) }), ..::semio_framework_async::poll::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Reorganize", "Neu anordnen"), "rotate-cw")) }
}

/// 🎛️ Anchor-only physics: no repulsion and no springs, a linear pull toward every node's layered target, settled
/// once no node moves more than a hundredth of a world unit, so the run lands on the layered layout.
pub fn layout_config() -> LayoutRunConfig {
    LayoutRunConfig { repulsion_strength: 0.0, spring_strength: 0.0, spring_law: LayoutRunSpringLaw::Linear, anchor_strength: 1.0, settle_displacement: 0.01, ..LayoutRunConfig::default() }
}
//#endregion 🔖️Definition

//#region 🔖️Graph
/// 🕸️ The layout graph of a DAG document plus the node id behind every layout node index.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DagLayoutGraph {
    pub graph: LayoutRunGraph,
    pub node_ids: Vec<String>,
}

/// 🌳️ Every node's layered target: the scene's layered layout (`DagHost::reorganize`), computed once, O(V + E).
pub fn layered_targets(scene: &DagScene, config: &DagConfig) -> Result<HashMap<String, LayoutRunPoint>, Fault> {
    let document = semio_framework_artifact_infinite_dag::DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: scene.nodes.clone(), edges: scene.edges.clone() };
    let fixture = DagHostSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), ..dag_host_snapshot_from_document(&document, dag_config_camera(config)) };
    let mut host = DagHost::load_host_snapshot_json(&semio_framework_pack_json::to_json_string(&fixture)).map_err(|error| layout_fault("dag.layout-run.layered-load", error.to_string()))?;
    let mut progress=|_|true;let mut control=infinite_canvas::board::schema::layout::LayoutControl::new(100_000_000,&mut progress);host.reorganize(&DagLayoutOptions::default(),&mut control).map_err(|error| layout_fault("dag.layout-run.layered-layout", error.to_string()))?;
    let layered: DagHostSnapshot = host.host_snapshot_json().ok().and_then(|json| semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).ok_or_else(|| layout_fault("dag.layout-run.layered-result", "the layered host returned no decodable fixture".into()))?;
    Ok(layered.nodes.into_iter().map(|node| (node.id, LayoutRunPoint::new(node.x, node.y))).collect())
}

/// 🗺️ Maps the scene: one body per first-seen node id at its committed position, repelling by half its diagonal,
/// anchored to its layered target; one unit spring per distinct node pair an edge connects (ports stripped).
pub fn layout_graph(scene: &DagScene, targets: &HashMap<String, LayoutRunPoint>) -> DagLayoutGraph {
    let (mut layout, mut index) = (DagLayoutGraph::default(), HashMap::new());
    for node in scene.nodes.iter().cloned() {
        if index.contains_key(&node.id) {
            continue;
        }
        index.insert(node.id.clone(), layout.node_ids.len() as u32);
        let origin = (node.x.is_finite() && node.y.is_finite()).then(|| LayoutRunPoint::new(node.x, node.y));
        let radius = ((node.width * node.width + node.height * node.height).sqrt() * 0.5).max(8.0);
        let radius = if radius.is_finite() { radius } else { 8.0 };
        layout.graph.nodes.push(LayoutRunNode { entity: layout_run_entity(&node.id), origin, radius, pinned: false, anchor: targets.get(&node.id).copied().filter(|target| target.x.is_finite() && target.y.is_finite()) });
        layout.node_ids.push(node.id);
    }
    let mut seen = BTreeSet::new();
    for edge in &scene.edges {
        let (Some(source), Some(target)) = (index.get(&crate::schema::split_endpoint(&edge.source).0).copied(), index.get(&crate::schema::split_endpoint(&edge.target).0).copied()) else { continue };
        if source != target && seen.insert((source.min(target), source.max(target))) {
            layout.graph.edges.push(LayoutRunEdge { source: source.min(target), target: source.max(target), weight: 1.0 });
        }
    }
    layout
}
//#endregion 🔖️Graph

//#region 🔖️Job
/// ✍️ Encodes one layout move as the `content` child's absolute graph `move-node` op.
pub fn move_encoder(node_ids: Vec<String>) -> impl LayoutRunOpEncoder + 'static {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::move_node::MoveNode;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;
    move |node: u32, _entity: u64, position: LayoutRunPoint| {
        let leaf = SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new(node_ids[node as usize].clone()), new_position: SemioPoint2 { x: position.x, y: position.y } });
        protocol::OpBinary::encode_op(&leaf).map_err(|error| LayoutRunEncodeError(format!("{error:?}")))
    }
}

/// 📍️ The overlay positions a resumed run continues from: the base origins with the provisional member moves folded in.
fn overlay_positions(layout: &DagLayoutGraph, member_ops: &[Vec<u8>]) -> Vec<LayoutRunPoint> {
    let index: HashMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();
    let moves = member_ops.iter().filter_map(|bytes| match <SemioGraphMutation as protocol::OpBinary>::decode_op(bytes) {
        Ok(SemioGraphMutation::MoveNode(payload)) => index.get(payload.id.value.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.new_position.x, payload.new_position.y))),
        _ => None,
    });
    layout_run_overlay_positions(&layout.graph, moves)
}

/// 🧵️ Builds the layout run over the run's base scene (composed from the `content` member); a settings change resumes it
/// from the ledger's checkpoint with the provisional member moves folded over the base positions (`📓️wave-W3-F.md` §3.3).
pub fn build_job(identity: ToolRunIdentity, scene: &DagScene, config: &DagConfig, checkpoint: Option<&[u8]>, member_ops: &[Vec<u8>]) -> Result<ToolRunJob, Fault> {
    let layout = layout_graph(scene, &layered_targets(scene, config)?);
    let resume = checkpoint.map(|checkpoint| LayoutRunResume { checkpoint, positions: overlay_positions(&layout, member_ops), provisional_len: member_ops.len() as u32 });
    let job = layout_run_job(identity, &layout.graph, layout_config(), || move_encoder(layout.node_ids.clone()), resume).map_err(|error| layout_fault("dag.layout-run.start", format!("{error:?}")))?;
    Ok(Box::new(job))
}
//#endregion 🔖️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
