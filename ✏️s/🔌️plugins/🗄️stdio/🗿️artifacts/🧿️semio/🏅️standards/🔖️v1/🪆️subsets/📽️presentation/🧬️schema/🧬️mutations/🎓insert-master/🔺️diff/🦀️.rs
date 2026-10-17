//! 🔺️ Diff for `InsertMaster`.

#[allow(unused_imports)]
use crate::schema::diff::SemioPresentationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::diff_insert_master;
#[allow(unused_imports)]
use crate::schema::mutations::support::master_at;

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
