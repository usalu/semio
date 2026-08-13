//! 🔺️ Sparse diff builder for `MoveBlock` — a real same-step reorder OR cross-step relocation
//! (remove from source, insert into target at `index`), never a whole-snapshot capture.
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveBlock, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    if payload.from_step_id == payload.to_step_id {
        if let Some(step) = steps.iter_mut().find(|step| step.id == payload.from_step_id) {
            if let Some(position) = step.blocks.iter().position(|block| block.id == payload.block_id) {
                let block = step.blocks.remove(position);
                let at = payload.index.min(step.blocks.len());
                step.blocks.insert(at, block);
            }
        }
        return diff_replace_content(base.title.as_deref(), steps);
    }
    let Some(block) = steps.iter().find(|step| step.id == payload.from_step_id).and_then(|step| step.blocks.iter().find(|block| block.id == payload.block_id)).cloned() else {
        return PlaybookDiff::default();
    };
    if let Some(from_step) = steps.iter_mut().find(|step| step.id == payload.from_step_id) {
        from_step.blocks.retain(|entry| entry.id != payload.block_id);
    }
    if let Some(to_step) = steps.iter_mut().find(|step| step.id == payload.to_step_id) {
        let at = payload.index.min(to_step.blocks.len());
        to_step.blocks.insert(at, block);
    }
    diff_replace_content(base.title.as_deref(), steps)
}
//#endregion 🔖️Diff
