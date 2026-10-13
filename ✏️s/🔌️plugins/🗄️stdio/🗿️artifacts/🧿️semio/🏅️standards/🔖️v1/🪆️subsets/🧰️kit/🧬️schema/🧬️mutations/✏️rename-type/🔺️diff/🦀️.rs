//! 🔺️ Diff for `RenameType`.

use semio_s_artifact_stdio_semio_base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::schema::diff::{SemioKitDiff, SemioKitTypeDiff};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RenameType, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    let Some(existing) = base.types.iter().find(|t| t.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Type \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    let at = base.types.iter().position(|t| t.id == payload.id).expect("checked above");
    protocol::MutationOutcome::new(SemioKitDiff { types: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioKitTypeDiff { name: Some(payload.new_name.clone()), category: None } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
