//! ↩️ Inverse for `RemoveMetadataEntry`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveMetadataEntry, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    let super::RemoveMetadataEntry { key } = payload;
    Ok(vec![match base.metadata.iter().position(|e| &e.key == key) {
        Some(at) => SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: base.metadata[at].value.clone(), at: Some(at) }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
