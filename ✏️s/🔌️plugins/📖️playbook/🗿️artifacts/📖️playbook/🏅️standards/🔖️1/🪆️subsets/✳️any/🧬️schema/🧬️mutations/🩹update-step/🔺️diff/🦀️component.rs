//! 🔺️ Sparse diff builder for `UpdateStep` — a real title/description patch entry, `blocks`
//! untouched (never a whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateStep, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.step_id) {
        step.title = payload.title.clone();
        step.description = payload.description.clone();
    }
    diff_replace_content(base.title.as_deref(), steps)
}
//#endregion 🔖️Diff
