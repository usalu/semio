//! 🔺️ Diff for `RemoveListItem`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveListItem, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    let super::RemoveListItem { path, index } = payload;
    protocol::MutationOutcome::new(match resolve(&base.root, path) {
        Some(SemioValue::List { items }) if *index < items.len() => {
            diff_at_path(path, Some(SemioValueDiff::List { diff: crate::standards::v1::subsets::base::schema::triples::IndexedTripleDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() } }))
        }
        _ => SemioValueTreeDiff::default(),
    })
}
//#endregion 🔖️Diff
