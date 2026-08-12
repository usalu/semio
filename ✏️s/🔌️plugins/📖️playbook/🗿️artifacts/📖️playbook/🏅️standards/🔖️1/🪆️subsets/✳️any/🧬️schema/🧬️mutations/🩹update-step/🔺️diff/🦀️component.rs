//! 🔺️ Sparse diff builder for `UpdateStep` — a real title/description patch entry, `blocks`
//! untouched (never a whole-snapshot capture).
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot, PlaybookStepPatch, PlaybookStepPatchEntry, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateStep, _base: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff {
        steps: Some(PlaybookStepsDelta {
            patched: vec![PlaybookStepPatchEntry { id: payload.step_id.clone(), patch: PlaybookStepPatch { title: Some(payload.title.clone()), description: Some(payload.description.clone()), blocks: None } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
