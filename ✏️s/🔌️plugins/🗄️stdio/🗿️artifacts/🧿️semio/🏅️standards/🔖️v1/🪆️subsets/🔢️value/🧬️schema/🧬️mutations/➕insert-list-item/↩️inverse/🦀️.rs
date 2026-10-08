//! ↩️ Inverse for `InsertListItem`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertListItem, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::InsertListItem { path, index, .. } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::List { items }) => vec![SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: path.clone(), index: (*index).min(items.len()) })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
