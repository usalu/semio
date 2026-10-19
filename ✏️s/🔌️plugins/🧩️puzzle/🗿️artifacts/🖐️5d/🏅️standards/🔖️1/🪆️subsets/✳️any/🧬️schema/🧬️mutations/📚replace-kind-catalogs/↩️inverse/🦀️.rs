//! `base`'s composed `kind_catalogs` handle + `kind_catalogs_extra` overflow back into the payload's
//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof). Reassembles
//! original `Option<Puzzle5dKindCatalogs>` shape via `kind_catalogs_of` (working-scene cache read).
use crate::standards::v1::subsets::any::schema::kind_catalogs::kind_catalogs_of;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::ReplaceKindCatalogs, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::ReplaceKindCatalogs>,
{
    Ok((|| {
    let restored = kind_catalogs_of(&base.kind_catalogs, &base.kind_catalogs_extra);
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::replace_kind_catalogs(restored)]

    })())
}
//#endregion 🔖️Inverse
