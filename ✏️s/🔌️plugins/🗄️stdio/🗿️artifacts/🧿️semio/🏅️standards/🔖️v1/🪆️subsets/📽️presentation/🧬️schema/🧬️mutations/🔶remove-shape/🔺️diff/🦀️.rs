//! 🔺️ Diff for `RemoveShape`.

#[allow(unused_imports)]
use crate::schema::diff::SemioPresentationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::diff_remove_shape;
#[allow(unused_imports)]
use crate::schema::mutations::support::shape_at;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveShape, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if shape_at(base, payload.slide_index, payload.shape_index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No shape exists at index #{}.", payload.shape_index), [payload.shape_index.to_string()]);
    }
    let super::RemoveShape { slide_index, shape_index } = payload;
    protocol::MutationOutcome::new(diff_remove_shape(*slide_index, *shape_index))
}
//#endregion 🔖️Diff
