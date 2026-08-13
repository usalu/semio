//! 🔺️ Sparse diff builder for `RemoveStep` — a real removal (never a whole-snapshot capture).
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveStep, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    steps.retain(|step| step.id != payload.step_id);
    diff_replace_content(base.title.as_deref(), steps)
}
//#endregion 🔖️Diff
