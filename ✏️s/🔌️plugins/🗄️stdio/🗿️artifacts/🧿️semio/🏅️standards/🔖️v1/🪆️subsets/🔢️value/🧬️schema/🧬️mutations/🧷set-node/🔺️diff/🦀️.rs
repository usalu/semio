//! 🔺️ Diff for `SetNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetNode, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    if base.nodes.iter().any(|n| n.id == payload.id && n.value == payload.value) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The node already has that value.");
    }
    let super::SetNode { id, value, at } = payload;
    protocol::MutationOutcome::new(match base.nodes.iter().find(|n| &n.id == id) {
        Some(_) => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: id.clone(), diff: SemioValueDiff::Replace { value: value.clone() } }] }) },
        None => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: at.map_or(base.nodes.len(), |at| at.min(base.nodes.len())), item: SemioValueNode { id: id.clone(), value: value.clone() } }] }) },
    })
}
//#endregion 🔖️Diff
