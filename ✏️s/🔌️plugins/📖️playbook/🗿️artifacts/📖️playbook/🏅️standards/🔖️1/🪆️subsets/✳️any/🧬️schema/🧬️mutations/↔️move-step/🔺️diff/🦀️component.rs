//! 🔺️ Sparse diff builder for `MoveStep` — a real reordering of the step-id list (never a
//! whole-snapshot capture).
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveStep, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut order: Vec<String> = base.steps.iter().map(|step| step.id.clone()).collect();
    if let Some(position) = order.iter().position(|id| *id == payload.step_id) {
        let entry = order.remove(position);
        let at = payload.index.min(order.len());
        order.insert(at, entry);
    }
    PlaybookDiff { steps: Some(PlaybookStepsDelta { reordered: Some(order), ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
