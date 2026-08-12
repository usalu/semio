//! ↩️ Inverse for `ChangeCatalogGeneration` — the OLD counter value looked up from BASE (never a
//! structural inversion of the diff).
use crate::artifacts::home::mutations::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeCatalogGeneration, base: &SHomeSnapshot) -> Vec<SHomeMutation> {
    vec![super::mutation::change_catalog_generation(base.catalog_generation)]
}
//#endregion 🔖️Inverse
