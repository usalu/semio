//! ↩️ Inverse for `SetMetadataEntry`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetMetadataEntry, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    let super::SetMetadataEntry { key, .. } = payload;
    Ok(vec![match base.metadata.iter().find(|e| &e.key == key) {
        Some(entry) => SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: entry.value.clone(), at: None }),
        None => SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: key.clone() }),
    }])
}
//#endregion 🔖️Inverse
