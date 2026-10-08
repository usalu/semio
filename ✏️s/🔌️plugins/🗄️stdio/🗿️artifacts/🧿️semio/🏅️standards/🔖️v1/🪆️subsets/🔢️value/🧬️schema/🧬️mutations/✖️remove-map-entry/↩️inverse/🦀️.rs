//! ↩️ Inverse for `RemoveMapEntry`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveMapEntry, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::RemoveMapEntry { path, key } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::Map { entries }) => match entries.iter().position(|e| &e.key == key) {
            Some(pos) => {
                let tail: Vec<SemioValueEntry> = entries[pos + 1..].to_vec();
                let mut steps: Vec<SemioValueMutation> = tail.iter().rev().map(|e| SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: path.clone(), key: e.key.clone() })).collect();
                steps.push(SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: key.clone(), value: entries[pos].value.clone() }));
                steps.extend(tail.into_iter().map(|e| SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: e.key, value: e.value })));
                steps
            }
            None => Vec::new(),
        },
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
