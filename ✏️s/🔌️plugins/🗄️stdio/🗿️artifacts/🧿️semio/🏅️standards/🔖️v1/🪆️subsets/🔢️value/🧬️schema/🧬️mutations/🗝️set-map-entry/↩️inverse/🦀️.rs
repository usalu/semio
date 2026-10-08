//! ↩️ Inverse for `SetMapEntry`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetMapEntry, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::SetMapEntry { path, key, .. } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
            Some(existing) => vec![SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: key.clone(), value: existing.value.clone() })],
            None => vec![SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: path.clone(), key: key.clone() })],
        },
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
