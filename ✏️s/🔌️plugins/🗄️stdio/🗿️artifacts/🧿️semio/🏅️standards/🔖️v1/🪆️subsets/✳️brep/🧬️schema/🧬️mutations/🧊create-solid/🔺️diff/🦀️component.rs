//! 🔺️ `create-solid` — sparse diff construction; if a solid with this `id` already exists in
//! `base`, this is a no-op (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateSolid;
use crate::artifacts::semio::standards::v1::engine::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepSolid, SemioBrepSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateSolid, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if base.solids.iter().any(|x| x.id == payload.id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        solids: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepSolid { id: payload.id.clone(), shells: payload.shells.clone() }] }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
