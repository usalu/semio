//! 🔺️ Diff for `RemoveMaster`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveMaster, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::RemoveMaster { id } = payload;
    protocol::MutationOutcome::new(diff_remove_master(id))
}
//#endregion 🔖️Diff
