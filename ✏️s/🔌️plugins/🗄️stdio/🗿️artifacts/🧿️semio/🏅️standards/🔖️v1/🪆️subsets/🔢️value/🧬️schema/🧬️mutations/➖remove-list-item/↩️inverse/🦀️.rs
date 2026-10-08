//! ↩️ Inverse for `RemoveListItem`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveListItem, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::RemoveListItem { path, index } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::List { items }) => match items.get(*index) {
            Some(item) => vec![SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: path.clone(), index: *index, value: item.clone() })],
            None => Vec::new(),
        },
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
