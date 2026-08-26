//! 🔺️ Sparse diff builder for `UpdateStep` — a real title/description patch entry, `blocks`
//! untouched (never a whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateStep, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(existing) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    if existing.title == payload.title && existing.description == payload.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already unchanged.", payload.step_id));
    }
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.step_id) {
        step.title = payload.title.clone();
        step.description = payload.description.clone();
    }
    protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps))
}
//#endregion 🔖️Diff
