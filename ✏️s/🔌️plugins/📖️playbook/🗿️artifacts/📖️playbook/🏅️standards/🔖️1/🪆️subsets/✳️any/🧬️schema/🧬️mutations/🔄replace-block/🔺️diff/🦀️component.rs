//! 🔺️ Sparse diff builder for `ReplaceBlock` — a real whole-block patch entry (never a
//! whole-snapshot capture).
use crate::artifacts::playbook::{PlaybookBlockPatch, PlaybookBlockPatchEntry, PlaybookBlocksDelta, PlaybookDiff, PlaybookSnapshot, PlaybookStepPatch, PlaybookStepPatchEntry, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceBlock, _base: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff {
        steps: Some(PlaybookStepsDelta {
            patched: vec![PlaybookStepPatchEntry {
                id: payload.step_id.clone(),
                patch: PlaybookStepPatch {
                    blocks: Some(PlaybookBlocksDelta { patched: vec![PlaybookBlockPatchEntry { id: payload.block.id.clone(), patch: PlaybookBlockPatch { block: Some(payload.block.clone()) } }], ..Default::default() }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
