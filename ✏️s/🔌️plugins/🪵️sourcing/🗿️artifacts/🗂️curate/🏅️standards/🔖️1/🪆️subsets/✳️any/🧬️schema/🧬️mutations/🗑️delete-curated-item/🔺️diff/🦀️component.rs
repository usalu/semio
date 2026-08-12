//! 🔺 Sparse diff builder for `DeleteCuratedItem` — a real removal (never a whole-snapshot
//! capture). Missing target ⇒ empty diff.
use crate::artifacts::curate::diff::{CurateCuratedDelta, CurateDiff};
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteCuratedItem, base: &CurateSnapshot) -> CurateDiff {
    if !base.curated.iter().any(|item| item.object_id == payload.object_id) {
        return CurateDiff::default();
    }
    CurateDiff { curated: Some(CurateCuratedDelta { removed: vec![payload.object_id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
