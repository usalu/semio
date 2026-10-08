//! 🔺️ Diff for `SetBlockEntityLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetBlockEntityLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::SetBlockEntityLayer { block_name, handle, layer } = payload;
    protocol::MutationOutcome::new(wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }))
}
//#endregion 🔖️Diff
