//! 🔺️ `create-solid` — sparse diff construction; a solid with this `id` already present in
//! `base` is `mutation.duplicate-id` (Fatal, empty diff — real entity-lifecycle safety, never a
//! silent duplicate). `shells` referential integrity is the subset's `✅validation-report`
//! inference's job, not this diff constructor's.

use super::mutation::CreateSolid;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepSolid, SemioBrepSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateSolid, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.solids.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A solid with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { solids: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepSolid { id: payload.id.clone(), shells: payload.shells.clone() }] }), ..Default::default() })
}
//#endregion 🔖️Diff
