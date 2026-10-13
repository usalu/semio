//! 🔺️ Diff for `AddDesign`.

use semio_s_artifact_stdio_semio_base::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::schema::diff::SemioKitDiff;
use crate::schema::snapshot::{SemioKitSnapshot, SemioKitDesign};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddDesign, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.designs.iter().any(|d| d.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A design with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let item = SemioKitDesign { id: payload.id.clone(), name: payload.name.clone(), pieces: Vec::new(), connections: Vec::new() };
    protocol::MutationOutcome::new(SemioKitDiff { designs: Some(IndexedTripleDiff { added: vec![IndexAdded { index: payload.at.map_or(base.designs.len(), |at| at.min(base.designs.len())), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
