//! 🔺️ Diff for `SetEntityGeometry`.

#[allow(unused_imports)]
use crate::schema::diff::CadEntityRecordDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_entity;
#[allow(unused_imports)]
use crate::schema::diff::wrap_entity_diff;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEntityGeometry, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_entity(base, &payload.handle).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Entity \"{}\" does not exist.", payload.handle), [payload.handle.clone()]);
    }
    let super::SetEntityGeometry { handle, entity } = payload;
    protocol::MutationOutcome::new(wrap_entity_diff(handle, CadEntityRecordDiff { layer: None, entity: Some(entity.clone()) }))
}
//#endregion 🔖️Diff
