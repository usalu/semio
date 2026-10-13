//! 🔺️ Diff for `AddType`.

use semio_s_artifact_stdio_semio_base::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::schema::diff::SemioKitDiff;
use crate::schema::snapshot::{SemioKitSnapshot, SemioKitType};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddType, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.types.iter().any(|t| t.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A type with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let item = SemioKitType { id: payload.id.clone(), name: payload.name.clone(), category: payload.category.clone() };
    protocol::MutationOutcome::new(SemioKitDiff { types: Some(IndexedTripleDiff { added: vec![IndexAdded { index: payload.at.map_or(base.types.len(), |at| at.min(base.types.len())), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
