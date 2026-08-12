//! 🔺 Sparse diff builder for `CreateCuratedItem` — a real append-only insert (never a whole-
//! snapshot capture).
use crate::artifacts::curate::diff::{CurateCuratedDelta, CurateDiff};
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateCuratedItem, _base: &CurateSnapshot) -> CurateDiff {
    CurateDiff { curated: Some(CurateCuratedDelta { added: vec![payload.item.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
