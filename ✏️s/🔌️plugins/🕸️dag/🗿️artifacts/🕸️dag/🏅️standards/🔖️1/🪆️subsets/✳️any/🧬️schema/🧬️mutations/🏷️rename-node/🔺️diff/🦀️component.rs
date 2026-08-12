//! 🔺️ Sparse diff builder for `RenameNode` — patches the node's `id` in place (no reorder side
//! effect) and rewrites every edge endpoint string that pointed at the old id.
use crate::artifacts::dag::diff::{DagDiff, DagEdgePatchEntry, DagEdgesDelta, DagNodeExtraPatch, DagNodeExtraPatchEntry, DagNodesDelta};
use crate::artifacts::dag::engine::split_endpoint;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::DagEdgePatch;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameNode, base: &DagSnapshot) -> DagDiff {
    if !base.nodes.iter().any(|node| node.id == payload.id) {
        return DagDiff::default();
    }
    let patched: Vec<DagEdgePatchEntry> = base
        .edges
        .iter()
        .filter_map(|edge| {
            let (source_node, source_port) = split_endpoint(&edge.source);
            let (target_node, target_port) = split_endpoint(&edge.target);
            let touches_source = source_node == payload.id;
            let touches_target = target_node == payload.id;
            if !touches_source && !touches_target {
                return None;
            }
            Some(DagEdgePatchEntry {
                id: edge.id.clone(),
                patch: DagEdgePatch {
                    source: touches_source.then(|| format!("{}@{}", payload.new_id, source_port)),
                    target: touches_target.then(|| format!("{}@{}", payload.new_id, target_port)),
                },
            })
        })
        .collect();
    DagDiff {
        nodes: Some(DagNodesDelta {
            extra_patched: vec![DagNodeExtraPatchEntry { id: payload.id.clone(), patch: DagNodeExtraPatch { new_id: Some(payload.new_id.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        edges: if patched.is_empty() { None } else { Some(DagEdgesDelta { patched, ..Default::default() }) },
        ..Default::default()
    }
}
//#endregion 🔖️Diff
