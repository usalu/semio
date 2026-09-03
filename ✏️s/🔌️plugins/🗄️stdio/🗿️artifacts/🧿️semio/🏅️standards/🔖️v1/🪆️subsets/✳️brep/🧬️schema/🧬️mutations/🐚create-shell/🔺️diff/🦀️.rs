//! 🔺️ Diff for `CreateShell`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepShell, BrepShellFace, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateShell, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.shells.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A shell with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { shells: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepShell { id: payload.id.clone(), faces: payload.faces.clone() }] }), ..Default::default() })
}
//#endregion 🔖️Diff
