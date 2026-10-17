//! 🔺️ Diff for `SetLayoutMaster`.

#[allow(unused_imports)]
use crate::schema::diff::SemioPresentationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::diff_set_layout_master;
#[allow(unused_imports)]
use crate::schema::mutations::support::layout_at;
#[allow(unused_imports)]
use crate::schema::mutations::support::master_at;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetLayoutMaster, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if layout_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layout \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if master_at(base, &payload.master_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Master \"{}\" does not exist.", payload.master_id), [payload.master_id.clone()]);
    }
    let super::SetLayoutMaster { id, master_id } = payload;
    protocol::MutationOutcome::new(diff_set_layout_master(id, master_id))
}
//#endregion 🔖️Diff
