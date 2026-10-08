//! 🔺️ Diff for `SetEntityGeometry`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEntityGeometry, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::SetEntityGeometry { handle, entity } = payload;
    protocol::MutationOutcome::new(wrap_entity_diff(handle, CadEntityRecordDiff { layer: None, entity: Some(entity.clone()) }))
}
//#endregion 🔖️Diff
