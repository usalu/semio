//! 🔺️ Sparse diff builder for `AddStep` — a real ordered insert (never a whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddStep, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    if steps.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" already exists.", payload.step.id));
    }
    let at = payload.index.unwrap_or(steps.len()).min(steps.len());
    steps.insert(at, payload.step.clone());
    protocol::MutationOutcome::new(diff_replace_content(base.title.as_deref(), steps))
}
//#endregion 🔖️Diff
