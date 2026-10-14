//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof).
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceKindCatalogs>>(_payload: &super::ReplaceKindCatalogs, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::replace_kind_catalogs(base.meta.kind_catalogs.clone())]

    })())
}
//#endregion 🔖️Inverse
