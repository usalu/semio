//! 🔺️ Diff fragment yielded by `DragBlocks`.
use super::mutation::DragBlocks;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DragBlocks, base: &NoteSnapshot) -> NoteDiff {
    let mut delta = crate::artifacts::note::schema::diff::NoteBlocksDelta::default();
    for id in &payload.ids {
        let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, id) else { continue };
        let mut moved = block.clone();
        crate::artifacts::note::schema::offset_block_tree(&mut moved, payload.dx, payload.dy);
        delta.patched.push(crate::artifacts::note::schema::diff::NoteBlockPatchEntry { id: id.clone(), patch: crate::artifacts::note::schema::diff::NoteBlockPatch { block_json: Some(serde_json::to_string(&moved).expect("json")) } });
    }
    NoteDiff { blocks: Some(delta), ..Default::default() }
}
//#endregion 🔖️Diff
