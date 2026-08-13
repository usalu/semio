//! 🔺️ Sparse diff builder for `ReplaceBlock` — a real whole-block patch entry (never a
//! whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceBlock, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.step_id) {
        if let Some(block) = step.blocks.iter_mut().find(|block| block.id == payload.block.id) {
            *block = payload.block.clone();
        }
    }
    diff_replace_content(base.title.as_deref(), steps)
}
//#endregion 🔖️Diff
