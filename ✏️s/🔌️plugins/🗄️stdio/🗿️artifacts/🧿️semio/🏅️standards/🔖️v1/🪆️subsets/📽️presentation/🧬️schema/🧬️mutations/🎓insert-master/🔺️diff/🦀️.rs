//! 🔺️ Diff for `InsertMaster`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertMaster, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if master_at(base, &payload.master.id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Master \"{}\" already exists.", payload.master.id), [payload.master.id.clone()]);
    }
    let super::InsertMaster { master, at } = payload;
    protocol::MutationOutcome::new(diff_insert_master(base, master.clone(), *at))
}
//#endregion 🔖️Diff
