//! 🔺️ Sparse diff builder for `AddBlock` — a real ordered insert into the owning step's block
//! list (never a whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::AddBlock, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    if step.blocks.iter().any(|block| block.id == payload.block.id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" already exists in step \"{}\".", payload.block.id, payload.step_id));
    }
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.step_id) {
        let at = payload.index.unwrap_or(step.blocks.len()).min(step.blocks.len());
        step.blocks.insert(at, payload.block.clone());
    }
    protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps))
}
//#endregion 🔖️Diff
