//! 🗂️ Edit-mode tool — Reorganize: the Trinity graph laid out by the framework's stepped force-layout run
//! (`semio-framework-graph-layout-run`). Every iteration is one visible unit: node moves are provisional graph `move-node`
//! ops of the composed `content` child (a child-target run, design §12, §20.15) the graph window renders from the ToolRun
//! overlay, each node carries an entity trace verdict, and finalize publishes ONE `content` member edit with one move per
//! moved node (`📋️tool-run-contract.md` §3.7, `📓️wave-W3-F.md` §3).

use semio_framework_graph_layout_run::{layout_run_definition, layout_run_entity, layout_run_job, layout_run_overlay_positions, LayoutRunConfig, LayoutRunEdge, LayoutRunEncodeError, LayoutRunGraph, LayoutRunNode, LayoutRunOpEncoder, LayoutRunPoint, LayoutRunResume};
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin, ToolDefinition, ToolRunJob};
use semio_framework_tool_run::{JobKindId, ToolRunIdentity};
use semio_framework_ui_locale::LocalizedLabel;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use std::collections::{BTreeSet, HashMap};

//#region 🔖️Constants
pub const TOOL_ID: &str = "reorganize";
/// 🧵️ Job kind of the layout run (`ToolRunDefinition.runJob`).
pub const LAYOUT_RUN_JOB: &str = "trinity.jack.layoutRun";
/// 🧩️ The composed-child slot the run edits.
pub const LAYOUT_RUN_MEMBER: &str = "content";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::jack::create_trinity_jack_app`; the run vocabulary and its
/// EN/DE labels are the shared layout run's.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(semio_framework_tool_run::ToolRunDefinition { member: Some(LAYOUT_RUN_MEMBER.into()), ..layout_run_definition(JobKindId::new(LAYOUT_RUN_JOB)) }), ..::semio_framework_async::poll::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Reorganize", "Neu anordnen"), "rotate-cw")) }
}

/// 🎛️ The shared Fruchterman-Reingold defaults over at most 120 iterations, the budget the graph's reorganize has.
pub fn layout_config() -> LayoutRunConfig {
    LayoutRunConfig { max_iterations: 120, ..LayoutRunConfig::default() }
}
//#endregion 🔖️Definition

//#region 🔖️Graph
/// 🕸️ The layout graph of a Trinity graph document plus the node id behind every layout node index.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JackLayoutGraph {
    pub graph: LayoutRunGraph,
    pub node_ids: Vec<String>,
}

/// 🗺️ Maps the content child: one body per first-seen node id at its committed position, repelling by a quarter of its
/// clamped extent sum; one unit spring per distinct node pair a connection joins.
pub fn layout_graph(content: &SemioGraphSnapshot) -> JackLayoutGraph {
    let (mut layout, mut index) = (JackLayoutGraph::default(), HashMap::new());
    for node in &content.nodes {
        if index.contains_key(&node.id.value) {
            continue;
        }
        index.insert(node.id.value.clone(), layout.node_ids.len() as u32);
        let origin = (node.position.x.is_finite() && node.position.y.is_finite()).then(|| LayoutRunPoint::new(node.position.x, node.position.y));
        let radius = (node.width.max(48.0) + node.height.max(24.0)) * 0.25;
        layout.graph.nodes.push(LayoutRunNode { entity: layout_run_entity(&node.id.value), origin, radius: if radius.is_finite() { radius } else { 18.0 }, pinned: false, anchor: None });
        layout.node_ids.push(node.id.value.clone());
    }
    let mut seen = BTreeSet::new();
    for edge in &content.edges {
        let (Some(source), Some(target)) = (index.get(&edge.source.value).copied(), index.get(&edge.target.value).copied()) else { continue };
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
fn overlay_positions(layout: &JackLayoutGraph, member_ops: &[Vec<u8>]) -> Vec<LayoutRunPoint> {
    let index: HashMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();
    let moves = member_ops.iter().filter_map(|bytes| match <SemioGraphMutation as protocol::OpBinary>::decode_op(bytes) {
        Ok(SemioGraphMutation::MoveNode(payload)) => index.get(payload.id.value.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.new_position.x, payload.new_position.y))),
        _ => None,
    });
    layout_run_overlay_positions(&layout.graph, moves)
}

/// 🧵️ Builds the layout run over the run's base content child; a settings change resumes it from the ledger's checkpoint
/// with the provisional member moves folded over the base positions (`📓️wave-W3-F.md` §3.3).
pub fn build_job(identity: ToolRunIdentity, content: &SemioGraphSnapshot, checkpoint: Option<&[u8]>, member_ops: &[Vec<u8>]) -> Result<ToolRunJob, Fault> {
    let layout = layout_graph(content);
    let resume = checkpoint.map(|checkpoint| LayoutRunResume { checkpoint, positions: overlay_positions(&layout, member_ops), provisional_len: member_ops.len() as u32 });
    let job = layout_run_job(identity, &layout.graph, layout_config(), || move_encoder(layout.node_ids.clone()), resume).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("trinity.jack.layout-run.start"), format!("{error:?}")))?;
    Ok(Box::new(job))
}
//#endregion 🔖️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
