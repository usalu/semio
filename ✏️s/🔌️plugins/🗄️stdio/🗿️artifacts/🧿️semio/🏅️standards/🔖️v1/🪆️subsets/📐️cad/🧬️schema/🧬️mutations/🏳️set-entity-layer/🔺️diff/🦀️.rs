//! 🔺️ Diff for `SetEntityLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEntityLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::SetEntityLayer { handle, layer } = payload;
    protocol::MutationOutcome::new(wrap_entity_diff(handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }))
}
//#endregion 🔖️Diff
