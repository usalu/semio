//! ↩️ Inverse for `InsertListItem`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use crate::path::resolve;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertListItem, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::remove_list_item::RemoveListItem>,
{
    let super::InsertListItem { path, index, .. } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::List { items }) => vec![Op::from(crate::remove_list_item::RemoveListItem { path: path.clone(), index: (*index).min(items.len()) })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
