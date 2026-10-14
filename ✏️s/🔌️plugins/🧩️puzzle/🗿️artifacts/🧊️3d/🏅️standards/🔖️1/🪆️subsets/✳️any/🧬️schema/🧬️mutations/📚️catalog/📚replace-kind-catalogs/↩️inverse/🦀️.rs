//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof).
use super::mutation::ReplaceKindCatalogs;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceKindCatalogs>>(_payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::replace_kind_catalogs::mutation::replace_kind_catalogs(base.meta.kind_catalogs.clone())]

    })())
}
//#endregion 🔖️Inverse
