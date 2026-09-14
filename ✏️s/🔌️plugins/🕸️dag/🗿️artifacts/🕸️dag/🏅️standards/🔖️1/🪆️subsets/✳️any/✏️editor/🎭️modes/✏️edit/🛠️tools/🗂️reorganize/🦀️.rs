//! 🗂️ Edit-mode tool — Reorganize: the DAG's layered layout (`DagHost::reorganize`, computed once per run) reached
//! through the framework's stepped layout run (`semio-framework-graph-layout-run`). Every node is anchored to its
//! layered target, so each iteration visibly pulls the committed graph toward the layered one: node moves are
//! provisional `move-node` ops the canvas renders from the ToolRun overlay, every node carries an entity trace verdict,
//! and finalize publishes one edit with one move per moved node (`📋️tool-run-contract.md` §3.7, `📓️wave-W3-F.md` §3.6).

use crate::editor::dag::config::{dag_config_camera, DagConfig};
use crate::op::DagMutation;
use crate::DagSnapshot;
use infinite_board_port_directed_dag::{DagHost, DagLayoutOptions};
use semio_framework_artifact_infinite_dag::{dag_fixture_from_document, DagFixture, DAG_DOCUMENT_SCHEMA};
use semio_framework_graph_layout_run::{layout_run_definition, layout_run_entity, layout_run_job, layout_run_overlay_positions, LayoutRunConfig, LayoutRunEdge, LayoutRunEncodeError, LayoutRunGraph, LayoutRunNode, LayoutRunOpEncoder, LayoutRunPoint, LayoutRunResume, LayoutRunSpringLaw};
use semio_framework_plugin::{Fault, LocalizedLabel, ToolDefinition, ToolRunJob};
use semio_framework_tool_run::{JobKindId, ToolRunIdentity};
use std::collections::{BTreeSet, HashMap};

//#region 🔖️Constants
pub const TOOL_ID: &str = "reorganize";
/// 🧵️ Job kind of the layout run (`ToolRunDefinition.runJob`).
pub const LAYOUT_RUN_JOB: &str = "dag.dag.layoutRun";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::dag::create_dag_app`; the run vocabulary and its EN/DE labels
/// are the shared layout run's.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(layout_run_definition(JobKindId::new(LAYOUT_RUN_JOB))), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Reorganize", "Neu anordnen"), "rotate-cw")) }
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

/// 🌳️ Every node's layered target: the document's layered layout (`DagHost::reorganize`), computed once, O(V + E).
pub fn layered_targets(snapshot: &DagSnapshot, config: &DagConfig) -> Result<HashMap<String, LayoutRunPoint>, Fault> {
    let fixture = DagFixture { schema: DAG_DOCUMENT_SCHEMA.into(), ..dag_fixture_from_document(&semio_framework_artifact_infinite_dag::DagSnapshot::from(snapshot), dag_config_camera(config)) };
    let mut host = DagHost::load_fixture_json(&dsl::json::to_json_string(&fixture)).map_err(|error| Fault::from(format!("dag.layout-run.layered-load: {error}")))?;
    host.reorganize(&DagLayoutOptions::default()).map_err(|error| Fault::from(format!("dag.layout-run.layered-layout: {error}")))?;
    let layered: DagFixture = host.fixture_json().ok().and_then(|json| dsl::json::from_json_str(&json).ok()).ok_or_else(|| Fault::from("dag.layout-run.layered-fixture"))?;
    Ok(layered.nodes.into_iter().map(|node| (node.id, LayoutRunPoint::new(node.x, node.y))).collect())
}

/// 🗺️ Maps the document: one body per first-seen node id at its committed position, repelling by half its diagonal,
/// anchored to its layered target; one unit spring per distinct node pair an edge connects (ports stripped).
pub fn layout_graph(snapshot: &DagSnapshot, targets: &HashMap<String, LayoutRunPoint>) -> DagLayoutGraph {
    let (mut layout, mut index) = (DagLayoutGraph::default(), HashMap::new());
    for node in snapshot.nodes() {
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
    for edge in snapshot.edges() {
        let (Some(source), Some(target)) = (index.get(&crate::schema::split_endpoint(&edge.source).0).copied(), index.get(&crate::schema::split_endpoint(&edge.target).0).copied()) else { continue };
        if source != target && seen.insert((source.min(target), source.max(target))) {
            layout.graph.edges.push(LayoutRunEdge { source: source.min(target), target: source.max(target), weight: 1.0 });
        }
    }
    layout
}
//#endregion 🔖️Graph

//#region 🔖️Job
/// ✍️ Encodes one layout move as this artifact's `move-node` op.
pub fn move_encoder(node_ids: Vec<String>) -> impl LayoutRunOpEncoder + 'static {
    move |node: u32, _entity: u64, position: LayoutRunPoint| protocol::OpBinary::encode_op(&crate::mutations::move_node(node_ids[node as usize].clone(), position.x, position.y)).map_err(|error| LayoutRunEncodeError(format!("{error:?}")))
}

/// 📍️ The overlay positions a resumed run continues from: the base origins with the provisional moves folded in.
fn overlay_positions(layout: &DagLayoutGraph, provisional: &[DagMutation]) -> Vec<LayoutRunPoint> {
    let index: HashMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();
    let moves = provisional.iter().filter_map(|op| match op {
        DagMutation::MoveNode(payload) => index.get(payload.id.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.x, payload.y))),
        _ => None,
    });
    layout_run_overlay_positions(&layout.graph, moves)
}

/// 🧵️ Builds the layout run over the run's base; a settings change resumes it from the ledger's checkpoint with the
/// provisional moves folded over the base positions (`📓️wave-W3-F.md` §3.3).
pub fn build_job(identity: ToolRunIdentity, snapshot: &DagSnapshot, config: &DagConfig, checkpoint: Option<&[u8]>, provisional: &[DagMutation]) -> Result<ToolRunJob, Fault> {
    let layout = layout_graph(snapshot, &layered_targets(snapshot, config)?);
    let resume = checkpoint.map(|checkpoint| LayoutRunResume { checkpoint, positions: overlay_positions(&layout, provisional), provisional_len: provisional.len() as u32 });
    let job = layout_run_job(identity, &layout.graph, layout_config(), || move_encoder(layout.node_ids.clone()), resume).map_err(|error| Fault::from(format!("dag.layout-run: {error:?}")))?;
    Ok(Box::new(job))
}
//#endregion 🔖️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
