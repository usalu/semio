//! 🔺️ Diff for `SetEntityLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEntityLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_entity(base, &payload.handle).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Entity \"{}\" does not exist.", payload.handle), [payload.handle.clone()]);
    }
    if find_entity(base, &payload.handle).is_some_and(|e| e.layer == payload.layer) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The entity is already on that layer.");
    }
    let super::SetEntityLayer { handle, layer } = payload;
    protocol::MutationOutcome::new(wrap_entity_diff(handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }))
}
//#endregion 🔖️Diff
