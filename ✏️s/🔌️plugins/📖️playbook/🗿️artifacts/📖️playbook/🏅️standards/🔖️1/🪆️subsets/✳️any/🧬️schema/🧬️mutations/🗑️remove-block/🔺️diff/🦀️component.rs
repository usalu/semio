//! 🔺️ Sparse diff builder for `RemoveBlock` — a real removal from the owning step's block list
//! (never a whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RemoveBlock, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    if !step.blocks.iter().any(|block| block.id == payload.block_id) {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Block \"{}\" does not exist in step \"{}\".", payload.block_id, payload.step_id),
            [payload.step_id.clone(), payload.block_id.clone()],
        );
    }
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.step_id) {
        step.blocks.retain(|block| block.id != payload.block_id);
    }
    protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps))
}
//#endregion 🔖️Diff
