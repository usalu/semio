//! 🔺️ Sparse diff builder for `RemoveBlock` — a real removal from the owning step's block list
//! (never a whole-snapshot capture).
use crate::artifacts::playbook::{PlaybookBlocksDelta, PlaybookDiff, PlaybookSnapshot, PlaybookStepPatch, PlaybookStepPatchEntry, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveBlock, _base: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff {
        steps: Some(PlaybookStepsDelta {
            patched: vec![PlaybookStepPatchEntry {
                id: payload.step_id.clone(),
                patch: PlaybookStepPatch { blocks: Some(PlaybookBlocksDelta { removed: vec![payload.block_id.clone()], ..Default::default() }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
