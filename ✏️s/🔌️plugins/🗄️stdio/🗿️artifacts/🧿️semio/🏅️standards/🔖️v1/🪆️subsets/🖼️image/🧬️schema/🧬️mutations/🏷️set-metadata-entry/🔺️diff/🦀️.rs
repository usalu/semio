//! 🔺️ Diff for `SetMetadataEntry`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetMetadataEntry, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::SetMetadataEntry { key, value } = payload;
    if base.metadata.iter().any(|e| &e.key == key && &e.value == value) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Metadata entry \"{key}\" already has this value."));
    }
    protocol::MutationOutcome::new({
        let metadata = if base.metadata.iter().any(|e| &e.key == key) {
            SemioImageMetadataDiff { modified: vec![NamedModified { key: key.clone(), diff: value.clone() }], ..Default::default() }
        } else {
            SemioImageMetadataDiff { added: vec![SemioImageMetadataEntry { key: key.clone(), value: value.clone() }], ..Default::default() }
        };
        SemioImageDiff { metadata: Some(metadata), ..Default::default() }
    })
}
//#endregion 🔖️Diff
