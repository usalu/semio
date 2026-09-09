//! 🔺️ Diff fragment yielded by `EditBlockMath`. Error `target-missing` when the block is absent
//! or not a math block, Warning `no-op` when the TeX source is unchanged.
use super::EditBlockMath;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &EditBlockMath, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let crate::NoteBlockNode::Math { tex, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a math block.", payload.id), [payload.id.clone()]);
    };
    if tex == &payload.new_tex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" math is unchanged.", payload.id));
    }
    let mut updated = block.clone();
    if let crate::NoteBlockNode::Math { tex, .. } = &mut updated {
        *tex = payload.new_tex.clone();
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, &updated))
}
//#endregion 🔖️Diff
