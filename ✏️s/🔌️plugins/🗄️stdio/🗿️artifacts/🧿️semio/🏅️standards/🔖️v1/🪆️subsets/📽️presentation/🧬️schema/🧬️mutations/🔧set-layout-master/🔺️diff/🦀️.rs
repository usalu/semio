//! 🔺️ Diff for `SetLayoutMaster`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetLayoutMaster, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::SetLayoutMaster { id, master_id } = payload;
    protocol::MutationOutcome::new(diff_set_layout_master(id, master_id))
}
//#endregion 🔖️Diff
