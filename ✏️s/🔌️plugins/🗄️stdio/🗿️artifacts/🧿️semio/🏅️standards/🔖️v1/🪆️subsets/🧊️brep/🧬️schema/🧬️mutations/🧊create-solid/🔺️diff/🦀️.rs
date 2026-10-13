//! 🔺️ Diff for `CreateSolid`.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use crate::schema::diff::SemioBrepDiff;
use crate::schema::snapshot::{BrepSolid, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateSolid, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.solids.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A solid with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { solids: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![semio_s_artifact_stdio_semio_base::schema::triples::NamedAdded { index: payload.at.map_or(base.solids.len(), |at| at.min(base.solids.len())), item: BrepSolid { id: payload.id.clone(), shells: payload.shells.clone() } }] }), ..Default::default() })
}
//#endregion 🔖️Diff
