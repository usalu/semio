//! 🔺️ Sparse diff builder for `AddBlock` — a real ordered insert into the owning step's block
//! list (never a whole-snapshot capture).
use crate::artifacts::playbook::{PlaybookBlocksDelta, PlaybookDiff, PlaybookSnapshot, PlaybookStepPatch, PlaybookStepPatchEntry, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddBlock, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut order: Vec<String> = base.steps.iter().find(|step| step.id == payload.step_id).map(|step| step.blocks.iter().map(|block| block.id.clone()).collect()).unwrap_or_default();
    let at = payload.index.unwrap_or(order.len()).min(order.len());
    order.insert(at, payload.block.id.clone());
    PlaybookDiff {
        steps: Some(PlaybookStepsDelta {
            patched: vec![PlaybookStepPatchEntry {
                id: payload.step_id.clone(),
                patch: PlaybookStepPatch { blocks: Some(PlaybookBlocksDelta { added: vec![payload.block.clone()], reordered: Some(order), ..Default::default() }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
