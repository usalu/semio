//! 🔺️ Diff for `RemoveMapEntry`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveMapEntry, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    let super::RemoveMapEntry { path, key } = payload;
    protocol::MutationOutcome::new(match resolve(&base.root, path) {
        Some(SemioValue::Map { entries }) if entries.iter().any(|e| &e.key == key) => diff_at_path(path, Some(SemioValueDiff::Map { diff: NamedTripleDiff { removed: vec![key.clone()], modified: Vec::new(), added: Vec::new() } })),
        _ => SemioValueTreeDiff::default(),
    })
}
//#endregion 🔖️Diff
