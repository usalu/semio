//! ↩️ Inverse for `ChangeCatalogGeneration` — the OLD counter value looked up from BASE (never a
//! structural inversion of the diff).
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeCatalogGeneration, base: &SHomeSnapshot) -> Result<Vec<SHomeMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::change_catalog_generation(base.catalog_generation)]

    })())
}
//#endregion 🔖️Inverse
