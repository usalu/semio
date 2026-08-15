//! 🔺️ `create-vertex` — sparse diff construction; if a vertex with this `id` already exists in
//! `base`, this is a no-op (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateVertex;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepVertex, SemioBrepSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateVertex, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if base.vertices.iter().any(|x| x.id == payload.id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff { vertices: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepVertex { id: payload.id.clone(), point: payload.point }] }), ..Default::default() }
}
//#endregion 🔖️Diff
