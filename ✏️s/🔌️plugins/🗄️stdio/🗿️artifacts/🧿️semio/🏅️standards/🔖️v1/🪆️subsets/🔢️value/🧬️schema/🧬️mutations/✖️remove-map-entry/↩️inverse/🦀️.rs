//! ↩️ Inverse for `RemoveMapEntry`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveMapEntry, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::RemoveMapEntry { path, key } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::Map { entries }) => match entries.iter().position(|e| &e.key == key) {
            Some(pos) => {
                vec![SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: key.clone(), value: entries[pos].value.clone(), at: Some(pos) })]
            }
            None => Vec::new(),
        },
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
