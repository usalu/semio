//! 🔺️ Sparse diff builder for `CreateStep` — a real append-only insert at the payload's
//! `path_ref` (never a whole-snapshot capture).
use crate::artifacts::imperative::diff::{ImperativeDiff, ImperativePathDelta, ImperativeStepsDelta};
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateStep, _base: &ImperativeSnapshot) -> ImperativeDiff {
    ImperativeDiff {
        path: Some(ImperativePathDelta {
            path_ref: payload.path_ref.clone(),
            steps: ImperativeStepsDelta { added: vec![payload.step.clone()], ..Default::default() },
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
