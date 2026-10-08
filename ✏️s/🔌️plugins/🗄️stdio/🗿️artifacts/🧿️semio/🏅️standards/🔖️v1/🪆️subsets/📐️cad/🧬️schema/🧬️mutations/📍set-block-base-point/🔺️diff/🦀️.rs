//! 🔺️ Diff for `SetBlockBasePoint`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetBlockBasePoint, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::SetBlockBasePoint { name, base_point } = payload;
    protocol::MutationOutcome::new(wrap_block_diff(name, CadBlockDiff { base_point: Some(*base_point), entities: None }))
}
//#endregion 🔖️Diff
