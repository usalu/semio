//! 🔺️ Diff for `RemoveMetadataEntry`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveMetadataEntry, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::RemoveMetadataEntry { key } = payload;
    if !base.metadata.iter().any(|e| &e.key == key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Metadata entry \"{key}\" does not exist."), [key.clone()]);
    }
    protocol::MutationOutcome::new(SemioImageDiff { metadata: Some(SemioImageMetadataDiff { removed: vec![key.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
