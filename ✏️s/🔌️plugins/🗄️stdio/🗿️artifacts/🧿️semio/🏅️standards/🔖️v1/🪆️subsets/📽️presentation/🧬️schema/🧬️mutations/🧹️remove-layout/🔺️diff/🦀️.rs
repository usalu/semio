//! 🔺️ Diff for `RemoveLayout`.

#[allow(unused_imports)]
use crate::schema::diff::SemioPresentationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::diff_remove_layout;
#[allow(unused_imports)]
use crate::schema::mutations::support::layout_at;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveLayout, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if layout_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layout \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::RemoveLayout { id } = payload;
    protocol::MutationOutcome::new(diff_remove_layout(id))
}
//#endregion 🔖️Diff
