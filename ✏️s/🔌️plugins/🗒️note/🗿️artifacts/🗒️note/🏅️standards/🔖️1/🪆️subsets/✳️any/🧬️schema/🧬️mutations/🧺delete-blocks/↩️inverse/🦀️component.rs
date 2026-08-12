//! ↩️ Inverse for `DeleteBlocks`.
use super::mutation::DeleteBlocks;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteBlocks, base: &NoteSnapshot) -> Vec<NoteMutation> {
    let mut entries: Vec<(Option<String>, usize, crate::artifacts::note::NoteBlockNode)> = payload.ids.iter().filter_map(|id| {
        let block = crate::artifacts::note::engine::find_block(&base.blocks, id)?.clone();
        let (parent_id, index) = crate::artifacts::note::engine::find_block_location(&base.blocks, id)?;
        Some((parent_id, index, block))
    }).collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    entries.into_iter().map(|(parent_id, index, block)| NoteMutation::CreateBlock(CreateBlock { block, parent_id, index: Some(index) })).collect()
}
//#endregion 🔖️Inverse
