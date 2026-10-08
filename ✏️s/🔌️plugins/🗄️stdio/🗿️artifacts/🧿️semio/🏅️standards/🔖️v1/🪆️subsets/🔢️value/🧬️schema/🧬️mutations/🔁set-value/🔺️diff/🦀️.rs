//! 🔺️ Diff for `SetValue`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetValue, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    let super::SetValue { path, value } = payload;
    protocol::MutationOutcome::new(match resolve(&base.root, path) {
        Some(old) if old != value => diff_at_path(path, Some(SemioValueDiff::Replace { value: value.clone() })),
        _ => SemioValueTreeDiff::default(),
    })
}
//#endregion 🔖️Diff
