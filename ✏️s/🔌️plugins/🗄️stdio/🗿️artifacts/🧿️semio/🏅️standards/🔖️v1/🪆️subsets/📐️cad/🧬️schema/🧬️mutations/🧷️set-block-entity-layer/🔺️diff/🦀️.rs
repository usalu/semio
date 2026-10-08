//! 🔺️ Diff for `SetBlockEntityLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetBlockEntityLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_block_entity(base, &payload.block_name, &payload.handle).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" has no entity \"{}\".", payload.block_name, payload.handle), [payload.block_name.clone(), payload.handle.clone()]);
    }
    let super::SetBlockEntityLayer { block_name, handle, layer } = payload;
    protocol::MutationOutcome::new(wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }))
}
//#endregion 🔖️Diff
