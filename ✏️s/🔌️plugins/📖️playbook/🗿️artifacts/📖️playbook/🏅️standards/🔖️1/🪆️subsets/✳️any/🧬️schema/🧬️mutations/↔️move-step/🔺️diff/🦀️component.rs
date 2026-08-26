//! 🔺️ Sparse diff builder for `MoveStep` — a real reordering of the step list (never a
//! whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveStep, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(position) = steps.iter().position(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let entry = steps.remove(position);
    let at = payload.index.min(steps.len());
    if at == position {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already at index {at}.", payload.step_id));
    }
    steps.insert(at, entry);
    protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps))
}
//#endregion 🔖️Diff
