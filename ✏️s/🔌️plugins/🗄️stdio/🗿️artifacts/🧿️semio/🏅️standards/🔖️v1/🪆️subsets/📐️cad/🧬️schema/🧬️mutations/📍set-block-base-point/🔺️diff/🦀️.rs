//! 🔺️ Diff for `SetBlockBasePoint`.

#[allow(unused_imports)]
use crate::schema::diff::CadBlockDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_block;
#[allow(unused_imports)]
use crate::schema::diff::wrap_block_diff;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetBlockBasePoint, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_block(base, &payload.name).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    }
    let super::SetBlockBasePoint { name, base_point } = payload;
    protocol::MutationOutcome::new(wrap_block_diff(name, CadBlockDiff { base_point: Some(*base_point), entities: None }))
}
//#endregion 🔖️Diff
