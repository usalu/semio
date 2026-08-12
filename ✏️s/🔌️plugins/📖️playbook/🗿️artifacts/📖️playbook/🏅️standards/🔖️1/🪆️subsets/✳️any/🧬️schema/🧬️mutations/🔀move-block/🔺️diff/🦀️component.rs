//! 🔺️ Sparse diff builder for `MoveBlock` — a real same-step reorder OR cross-step relocation
//! (remove from source, insert into target at `index`), never a whole-snapshot capture. Cross-step
//! relocation used to fall back to a whole-artifact replacement in the pre-migration kernel
//! translator (`playbook_diff_from_mutation`) — this is the real per-field replacement.
use crate::artifacts::playbook::{PlaybookBlocksDelta, PlaybookDiff, PlaybookSnapshot, PlaybookStepPatch, PlaybookStepPatchEntry, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveBlock, base: &PlaybookSnapshot) -> PlaybookDiff {
    if payload.from_step_id == payload.to_step_id {
        let mut order: Vec<String> = base.steps.iter().find(|step| step.id == payload.from_step_id).map(|step| step.blocks.iter().map(|block| block.id.clone()).collect()).unwrap_or_default();
        if let Some(position) = order.iter().position(|id| *id == payload.block_id) {
            order.remove(position);
        }
        let at = payload.index.min(order.len());
        order.insert(at, payload.block_id.clone());
        return PlaybookDiff {
            steps: Some(PlaybookStepsDelta {
                patched: vec![PlaybookStepPatchEntry { id: payload.from_step_id.clone(), patch: PlaybookStepPatch { blocks: Some(PlaybookBlocksDelta { reordered: Some(order), ..Default::default() }), ..Default::default() } }],
                ..Default::default()
            }),
            ..Default::default()
        };
    }
    let Some(block) = base.steps.iter().find(|step| step.id == payload.from_step_id).and_then(|step| step.blocks.iter().find(|block| block.id == payload.block_id)).cloned() else {
        return PlaybookDiff::default();
    };
    let mut target_order: Vec<String> = base.steps.iter().find(|step| step.id == payload.to_step_id).map(|step| step.blocks.iter().map(|entry| entry.id.clone()).collect()).unwrap_or_default();
    let at = payload.index.min(target_order.len());
    target_order.insert(at, payload.block_id.clone());
    PlaybookDiff {
        steps: Some(PlaybookStepsDelta {
            patched: vec![
                PlaybookStepPatchEntry { id: payload.from_step_id.clone(), patch: PlaybookStepPatch { blocks: Some(PlaybookBlocksDelta { removed: vec![payload.block_id.clone()], ..Default::default() }), ..Default::default() } },
                PlaybookStepPatchEntry { id: payload.to_step_id.clone(), patch: PlaybookStepPatch { blocks: Some(PlaybookBlocksDelta { added: vec![block], reordered: Some(target_order), ..Default::default() }), ..Default::default() } },
            ],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
