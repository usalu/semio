//! 🔺️ Diff for `AddBlockEntity`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddBlockEntity, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_block(base, &payload.block_name).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.block_name), [payload.block_name.clone()]);
    }
    if find_block_entity(base, &payload.block_name, &payload.entity.handle).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Block \"{}\" already has an entity \"{}\".", payload.block_name, payload.entity.handle), [payload.block_name.clone(), payload.entity.handle.clone()]);
    }
    let super::AddBlockEntity { block_name, entity, at } = payload;
    let block_len = base.blocks.iter().find(|block| block.name == *block_name).map_or(0, |block| block.entities.len());
    protocol::MutationOutcome::new({
        wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(block_len, |at| at.min(block_len)), item: entity.clone() }] }) })
    })
}
//#endregion 🔖️Diff
