//! 🔺 Sparse diff builder for `ChangeCuratedItemCount` — a real single-field patch (never a whole-
//! snapshot capture). Missing target ⇒ empty diff.
use crate::artifacts::curate::diff::{CurateCuratedDelta, CurateCuratedPatchEntry, CurateDiff};
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeCuratedItemCount, base: &CurateSnapshot) -> CurateDiff {
    if !base.curated.iter().any(|item| item.object_id == payload.object_id) {
        return CurateDiff::default();
    }
    CurateDiff {
        curated: Some(CurateCuratedDelta {
            patched: vec![CurateCuratedPatchEntry { object_id: payload.object_id.clone(), count: Some(payload.new_count) }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
