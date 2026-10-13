//! 🔺️ Diff for `RemoveType`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::SemioKitDiff;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveType, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.types.iter().any(|t| t.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let at = base.types.iter().position(|t| t.id == payload.id).expect("checked above");
    protocol::MutationOutcome::new(SemioKitDiff { types: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
