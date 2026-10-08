//! 🔺️ Diff for `AddBlockEntity`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddBlockEntity, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::AddBlockEntity { block_name, entity } = payload;
    protocol::MutationOutcome::new({
        wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![entity.clone()] }) })
    })
}
//#endregion 🔖️Diff
