//! 🔺️ Diff fragment yielded by `EditBlockMath`. Error `target-missing` when the block is absent
//! or not a math block, Warning `no-op` when the TeX source is unchanged.
use super::EditBlockMath;
use crate::schema::diff::NoteBlockPatch;
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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" math is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { tex: Some(payload.new_tex.clone()), ..Default::default() })]))
}
//#endregion 🔖️Diff
