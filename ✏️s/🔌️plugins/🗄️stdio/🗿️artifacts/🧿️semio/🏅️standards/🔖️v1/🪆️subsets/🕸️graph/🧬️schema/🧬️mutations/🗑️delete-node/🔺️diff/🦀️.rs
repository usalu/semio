//! 🔺️ Diff for `DeleteNode`.

use crate::standards::v1::subsets::base::schema::triples::{IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
/// 🧮️ The incident edges ONE `delete-node` may sever: the inverse rows its payload schema declares
/// (`x-semio-inverse-rows`, read through the derived `MutationLeaf::inverse_rows`) less the one row that restores the node.
/// The undo restores one row per severed edge, so a larger degree would outgrow the fold footprint the store admits for
/// the leaf from that same declaration.
pub fn cascade_edges_maximum(payload: &super::DeleteNode) -> usize {
    protocol::MutationLeaf::inverse_rows(payload).saturating_sub(1)
}

/// 🗑️ Removes the node together with every edge it severs. An unknown id is `mutation.target-missing`; a node whose
/// incident edges exceed [`cascade_edges_maximum`] is `mutation.target-referenced` and changes nothing, so the person
/// deletes edges first and the recorded undo never exceeds its declared rows.
pub fn diff(payload: &super::DeleteNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if !base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    }
    let severed = base.edges.iter().filter(|e| e.source == payload.id || e.target == payload.id).count();
    let maximum = cascade_edges_maximum(payload);
    if severed > maximum {
        return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetReferenced, format!("Node \"{}\" is still referenced by {severed} edges; one delete severs at most {maximum}.", payload.id.value), [payload.id.value.clone()]);
    }
    let incident: Vec<usize> = base.edges.iter().enumerate().filter(|(_, e)| e.source == payload.id || e.target == payload.id).map(|(index, _)| index).collect();
    let at = base.nodes.iter().position(|n| n.id == payload.id).expect("checked above");
    let outcome = protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }), edges: (!incident.is_empty()).then(|| IndexedTripleDiff { removed: incident, ..Default::default() }) });
    if severed > 0 {
        outcome.info("mutation.cascade", format!("Deleting node \"{}\" also severed {severed} edge(s).", payload.id.value))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
