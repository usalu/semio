//! 🔺️ Sparse diff builder for `AddStep` — a real ordered insert (never a whole-snapshot capture).
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot, PlaybookStepsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddStep, base: &PlaybookSnapshot) -> PlaybookDiff {
    let mut order: Vec<String> = base.steps.iter().map(|step| step.id.clone()).collect();
    let at = payload.index.unwrap_or(order.len()).min(order.len());
    order.insert(at, payload.step.id.clone());
    PlaybookDiff { steps: Some(PlaybookStepsDelta { added: vec![payload.step.clone()], reordered: Some(order), ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
