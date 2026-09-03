//! 🔺️ Diff for `CreateVertex`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepVertex, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateVertex, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.vertices.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A vertex with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { vertices: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepVertex { id: payload.id.clone(), point: payload.point }] }), ..Default::default() })
}
//#endregion 🔖️Diff
