//! 🔺️ Diff for `InsertMaster`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertMaster, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::InsertMaster { master } = payload;
    protocol::MutationOutcome::new(diff_insert_master(master.clone()))
}
//#endregion 🔖️Diff
