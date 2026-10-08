//! 🔺️ Diff for `AddBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddBlock, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_block(base, &payload.block.name).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Block \"{}\" already exists.", payload.block.name), [payload.block.name.clone()]);
    }
    let super::AddBlock { block, at } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.blocks.len(), |at| at.min(base.blocks.len())), item: block.clone() }] }), entities: None })
}
//#endregion 🔖️Diff
