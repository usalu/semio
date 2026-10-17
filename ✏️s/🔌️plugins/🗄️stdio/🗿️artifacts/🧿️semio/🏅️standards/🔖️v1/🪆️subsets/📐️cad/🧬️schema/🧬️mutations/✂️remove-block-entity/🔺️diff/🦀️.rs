//! 🔺️ Diff for `RemoveBlockEntity`.

#[allow(unused_imports)]
use crate::schema::diff::CadBlockDiff;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_block_entity;
#[allow(unused_imports)]
use crate::schema::diff::wrap_block_diff;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveBlockEntity, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_block_entity(base, &payload.block_name, &payload.handle).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" has no entity \"{}\".", payload.block_name, payload.handle), [payload.block_name.clone(), payload.handle.clone()]);
    }
    let super::RemoveBlockEntity { block_name, handle } = payload;
    protocol::MutationOutcome::new({
        wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }) })
    })
}
//#endregion 🔖️Diff
