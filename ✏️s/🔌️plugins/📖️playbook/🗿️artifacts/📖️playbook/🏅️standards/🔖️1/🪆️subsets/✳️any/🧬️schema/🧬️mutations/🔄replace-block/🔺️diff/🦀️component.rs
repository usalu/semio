//! 🔺️ Sparse diff builder for `ReplaceBlock` — a real whole-block patch entry (never a
//! whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceBlock, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let Some(existing) = step.blocks.iter().find(|block| block.id == payload.block.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist in step \"{}\".", payload.block.id, payload.step_id), [payload.step_id.clone(), payload.block.id.clone()]);
    };
    if existing == &payload.block {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" is already unchanged.", payload.block.id));
    }
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.step_id) {
        if let Some(block) = step.blocks.iter_mut().find(|block| block.id == payload.block.id) {
            *block = payload.block.clone();
        }
    }
    protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps))
}
//#endregion 🔖️Diff
