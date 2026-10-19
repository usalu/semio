//! ↩️ Inverse for `SetMapEntry`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use crate::path::resolve;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetMapEntry, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::remove_map_entry::RemoveMapEntry> + From<crate::set_map_entry::SetMapEntry>,
{
    let super::SetMapEntry { path, key, .. } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
            Some(existing) => vec![Op::from(crate::set_map_entry::SetMapEntry { path: path.clone(), key: key.clone(), value: existing.value.clone(), at: None })],
            None => vec![Op::from(crate::remove_map_entry::RemoveMapEntry { path: path.clone(), key: key.clone() })],
        },
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
