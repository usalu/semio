//! 🔺️ Sparse diff builder for `RemoveBlock` — a real removal from the owning step's block list
//! (never a whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveBlock, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.step_id) {
        step.blocks.retain(|block| block.id != payload.block_id);
    }
    diff_replace_content(base.title.as_deref(), steps)
}
//#endregion 🔖️Diff
