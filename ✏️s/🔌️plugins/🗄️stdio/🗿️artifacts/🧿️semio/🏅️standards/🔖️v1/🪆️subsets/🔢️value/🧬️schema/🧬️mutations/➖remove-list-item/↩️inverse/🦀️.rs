//! ↩️ Inverse for `RemoveListItem`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use crate::path::resolve;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveListItem, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::insert_list_item::InsertListItem>,
{
    let super::RemoveListItem { path, index } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::List { items }) => match items.get(*index) {
            Some(item) => vec![Op::from(crate::insert_list_item::InsertListItem { path: path.clone(), index: *index, value: item.clone() })],
            None => Vec::new(),
        },
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
