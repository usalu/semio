//! 🔺️ `create-shell` — sparse diff construction; if a shell with this `id` already exists in
//! `base`, this is a no-op (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateShell;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepShell, SemioBrepSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateShell, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if base.shells.iter().any(|x| x.id == payload.id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        shells: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepShell { id: payload.id.clone(), faces: payload.faces.clone() }] }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
