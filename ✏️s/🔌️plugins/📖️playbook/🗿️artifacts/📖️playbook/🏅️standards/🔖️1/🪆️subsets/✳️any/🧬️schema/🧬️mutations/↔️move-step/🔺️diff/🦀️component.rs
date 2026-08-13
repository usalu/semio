//! 🔺️ Sparse diff builder for `MoveStep` — a real reordering of the step list (never a
//! whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveStep, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    if let Some(position) = steps.iter().position(|step| step.id == payload.step_id) {
        let entry = steps.remove(position);
        let at = payload.index.min(steps.len());
        steps.insert(at, entry);
    }
    diff_replace_content(base.title.as_deref(), steps)
}
//#endregion 🔖️Diff
