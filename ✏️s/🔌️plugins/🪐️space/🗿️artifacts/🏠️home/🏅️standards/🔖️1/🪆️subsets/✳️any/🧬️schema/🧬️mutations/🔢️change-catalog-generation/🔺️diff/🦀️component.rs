//! 🔺️ Sparse diff builder for `ChangeCatalogGeneration` — a real single-field delta, built
//! directly from the payload (never apply-then-capture).
use crate::artifacts::home::diff::SHomeDiff;
use crate::artifacts::home::SHomeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeCatalogGeneration, _base: &SHomeSnapshot) -> SHomeDiff {
    SHomeDiff { catalog_generation: Some(payload.new_catalog_generation), ..Default::default() }
}
//#endregion 🔖️Diff
