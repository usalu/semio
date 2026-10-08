//! 🔺️ Diff for `SetMapEntry`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetMapEntry, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    if !matches!(resolve(&base.root, &payload.path), Some(SemioValue::Map { .. })) {
        return protocol::MutationOutcome::error("mutation.target-missing", "The path does not address a map.".to_string(), [payload.key.clone()]);
    }
    let super::SetMapEntry { path, key, value, at } = payload;
    protocol::MutationOutcome::new(match resolve(&base.root, path) {
        Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
            Some(_) => diff_at_path(path, Some(SemioValueDiff::Map { diff: NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: key.clone(), diff: SemioValueDiff::Replace { value: value.clone() } }] } })),
            None => diff_at_path(
                path,
                Some(SemioValueDiff::Map { diff: NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: at.map_or(entries.len(), |at| at.min(entries.len())), item: SemioValueEntry { key: key.clone(), value: value.clone() } }] } }),
            ),
        },
        _ => SemioValueTreeDiff::default(),
    })
}
//#endregion 🔖️Diff
