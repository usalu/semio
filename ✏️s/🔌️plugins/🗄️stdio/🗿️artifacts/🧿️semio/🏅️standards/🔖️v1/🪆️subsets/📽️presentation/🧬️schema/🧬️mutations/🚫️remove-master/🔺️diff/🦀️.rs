//! 🔺️ Diff for `RemoveMaster`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveMaster, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if master_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Master \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::RemoveMaster { id } = payload;
    protocol::MutationOutcome::new(diff_remove_master(id))
}
//#endregion 🔖️Diff
