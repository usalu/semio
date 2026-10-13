//! 🔺️ Diff for `CreateShell`.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use crate::schema::diff::SemioBrepDiff;
use crate::schema::snapshot::{BrepShell, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateShell, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.shells.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A shell with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { shells: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![semio_s_artifact_stdio_semio_base::schema::triples::NamedAdded { index: payload.at.map_or(base.shells.len(), |at| at.min(base.shells.len())), item: BrepShell { id: payload.id.clone(), faces: payload.faces.clone() } }] }), ..Default::default() })
}
//#endregion 🔖️Diff
