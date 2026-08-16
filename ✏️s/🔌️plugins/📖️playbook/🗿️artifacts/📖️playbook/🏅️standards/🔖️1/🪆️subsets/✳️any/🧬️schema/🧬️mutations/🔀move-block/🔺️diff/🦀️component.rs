//! 🔺️ Sparse diff builder for `MoveBlock` — a real same-step reorder OR cross-step relocation
//! (remove from source, insert into target at `index`), never a whole-snapshot capture.
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveBlock, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(from_step) = steps.iter().find(|step| step.id == payload.from_step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.from_step_id), [payload.from_step_id.clone()]);
    };
    let Some(position) = from_step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Block \"{}\" does not exist in step \"{}\".", payload.block_id, payload.from_step_id),
            [payload.from_step_id.clone(), payload.block_id.clone()],
        );
    };
    if payload.from_step_id == payload.to_step_id {
        let step = steps.iter_mut().find(|step| step.id == payload.from_step_id).expect("from_step_id already located above");
        let at = payload.index.min(step.blocks.len() - 1);
        if at == position {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" is already at index {at}.", payload.block_id));
        }
        let block = step.blocks.remove(position);
        step.blocks.insert(at, block);
        return protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps));
    }
    if !steps.iter().any(|step| step.id == payload.to_step_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.to_step_id), [payload.to_step_id.clone()]);
    }
    let block = from_step.blocks[position].clone();
    if let Some(from_step) = steps.iter_mut().find(|step| step.id == payload.from_step_id) {
        from_step.blocks.retain(|entry| entry.id != payload.block_id);
    }
    if let Some(to_step) = steps.iter_mut().find(|step| step.id == payload.to_step_id) {
        let at = payload.index.min(to_step.blocks.len());
        to_step.blocks.insert(at, block);
    }
    protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps))
}
//#endregion 🔖️Diff
