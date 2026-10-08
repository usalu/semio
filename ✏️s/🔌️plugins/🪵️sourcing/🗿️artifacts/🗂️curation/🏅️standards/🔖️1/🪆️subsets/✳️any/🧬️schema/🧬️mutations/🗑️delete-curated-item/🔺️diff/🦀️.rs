//! 🔺 Sparse diff builder for `DeleteCuratedItem` — a real removal (never a whole-snapshot
//! capture). Error `target-missing` when absent.
use crate::diff::{CurationCuratedDelta, CurationDiff};
use crate::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteCuratedItem, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    let Some(index) = base.curated.iter().position(|item| item.object_id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("\"{}\" is not curated.", payload.object_id), [payload.object_id.clone()]);
    };
    protocol::MutationOutcome::new(CurationDiff { curated: Some(CurationCuratedDelta::removal(&base.curated, index)), ..Default::default() })
}
//#endregion 🔖️Diff
