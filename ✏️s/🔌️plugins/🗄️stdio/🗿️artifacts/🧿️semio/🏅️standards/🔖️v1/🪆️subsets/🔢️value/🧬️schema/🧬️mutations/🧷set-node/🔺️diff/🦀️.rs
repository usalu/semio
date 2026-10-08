//! 🔺️ Diff for `SetNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetNode, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    let super::SetNode { id, value } = payload;
    protocol::MutationOutcome::new(match base.nodes.iter().find(|n| &n.id == id) {
        Some(existing) => match value_diff_between(&existing.value, value) {
            Some(diff) => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: id.clone(), diff }] }) },
            None => SemioValueTreeDiff::default(),
        },
        None => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: base.nodes.len(), item: SemioValueNode { id: id.clone(), value: value.clone() } }] }) },
    })
}
//#endregion 🔖️Diff
