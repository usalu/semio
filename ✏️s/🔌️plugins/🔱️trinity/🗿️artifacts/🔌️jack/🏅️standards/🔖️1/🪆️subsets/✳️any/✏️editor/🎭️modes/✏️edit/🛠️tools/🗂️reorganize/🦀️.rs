//! 🗂️ Edit-mode tool — Reorganize: the Trinity graph laid out by the framework's stepped force-layout run
//! (`semio-framework-graph-layout-run`). Every iteration is one visible unit: node moves are provisional `move-node`
//! ops the graph window renders from the ToolRun overlay, each node carries an entity trace verdict, and finalize
//! publishes one edit with one move per moved node (`📋️tool-run-contract.md` §3.7, `📓️wave-W3-F.md` §3).

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_graph_layout_run::{layout_run_definition, layout_run_entity, layout_run_job, layout_run_overlay_positions, LayoutRunConfig, LayoutRunEdge, LayoutRunEncodeError, LayoutRunGraph, LayoutRunNode, LayoutRunOpEncoder, LayoutRunPoint, LayoutRunResume};
use semio_framework_plugin::{Fault, LocalizedLabel, ToolDefinition, ToolRunJob};
use semio_framework_tool_run::{JobKindId, ToolRunIdentity};
use std::collections::{BTreeSet, HashMap};

//#region 🔖️Constants
pub const TOOL_ID: &str = "reorganize";
/// 🧵️ Job kind of the layout run (`ToolRunDefinition.runJob`).
pub const LAYOUT_RUN_JOB: &str = "trinity.jack.layoutRun";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::jack::create_trinity_jack_app`; the run vocabulary and its
/// EN/DE labels are the shared layout run's.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(layout_run_definition(JobKindId::new(LAYOUT_RUN_JOB))), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Reorganize", "Neu anordnen"), "rotate-cw")) }
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

/// 🗺️ Maps the document: one body per first-seen node id at its committed position, repelling by a quarter of its
/// clamped extent sum; one unit spring per distinct node pair a connection joins (ports stripped).
pub fn layout_graph(snapshot: &JackSnapshot) -> JackLayoutGraph {
    let scene = crate::jack_working_scene(snapshot);
    let (mut layout, mut index) = (JackLayoutGraph::default(), HashMap::new());
    for node in scene.nodes {
        if index.contains_key(&node.id) {
            continue;
        }
        index.insert(node.id.clone(), layout.node_ids.len() as u32);
        let origin = (node.x.is_finite() && node.y.is_finite()).then(|| LayoutRunPoint::new(node.x, node.y));
        let radius = (node.width.max(48.0) + node.height.max(24.0)) * 0.25;
        layout.graph.nodes.push(LayoutRunNode { entity: layout_run_entity(&node.id), origin, radius: if radius.is_finite() { radius } else { 18.0 }, pinned: false, anchor: None });
        layout.node_ids.push(node.id);
    }
    let mut seen = BTreeSet::new();
    for edge in scene.edges {
        let (Some(source), Some(target)) = (index.get(&crate::editor::jack::split_endpoint(&edge.source).0).copied(), index.get(&crate::editor::jack::split_endpoint(&edge.target).0).copied()) else { continue };
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
    move |node: u32, _entity: u64, position: LayoutRunPoint| protocol::OpBinary::encode_op(&crate::standards::v1::subsets::any::schema::mutations::move_node(node_ids[node as usize].clone(), position.x, position.y)).map_err(|error| LayoutRunEncodeError(format!("{error:?}")))
}

/// 📍️ The overlay positions a resumed run continues from: the base origins with the provisional moves folded in.
fn overlay_positions(layout: &JackLayoutGraph, provisional: &[TrinityGraphMutation]) -> Vec<LayoutRunPoint> {
    let index: HashMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();
    let moves = provisional.iter().filter_map(|op| match op {
        TrinityGraphMutation::MoveNode(payload) => index.get(payload.id.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.x, payload.y))),
        _ => None,
    });
    layout_run_overlay_positions(&layout.graph, moves)
}

/// 🧵️ Builds the layout run over the run's base; a settings change resumes it from the ledger's checkpoint with the
/// provisional moves folded over the base positions (`📓️wave-W3-F.md` §3.3).
pub fn build_job(identity: ToolRunIdentity, snapshot: &JackSnapshot, checkpoint: Option<&[u8]>, provisional: &[TrinityGraphMutation]) -> Result<ToolRunJob, Fault> {
    let layout = layout_graph(snapshot);
    let resume = checkpoint.map(|checkpoint| LayoutRunResume { checkpoint, positions: overlay_positions(&layout, provisional), provisional_len: provisional.len() as u32 });
    let job = layout_run_job(identity, &layout.graph, layout_config(), || move_encoder(layout.node_ids.clone()), resume).map_err(|error| Fault::from(format!("trinity.jack.layout-run: {error:?}")))?;
    Ok(Box::new(job))
}
//#endregion 🔖️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
