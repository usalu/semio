//! 🔺️ Diff for `RemoveNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveNode, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    if !base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    }
    let super::RemoveNode { id } = payload;
    protocol::MutationOutcome::new({
        if base.nodes.iter().any(|n| &n.id == id) {
            SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: vec![id.clone()], modified: Vec::new(), added: Vec::new() }) }
        } else {
            SemioValueTreeDiff::default()
        }
    })
}
//#endregion 🔖️Diff
