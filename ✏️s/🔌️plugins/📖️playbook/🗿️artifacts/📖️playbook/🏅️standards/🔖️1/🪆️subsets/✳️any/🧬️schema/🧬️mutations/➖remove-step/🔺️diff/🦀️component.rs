//! 🔺️ Sparse diff builder for `RemoveStep` — a real removal (never a whole-snapshot capture).
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveStep, _base: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff { steps: Some(PlaybookStepsDelta { removed: vec![payload.step_id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
