//! 🔺️ `create-shell` — sparse diff construction; a shell with this `id` already present in
//! `base` is `mutation.duplicate-id` (Fatal, empty diff — real entity-lifecycle safety, never a
//! silent duplicate). `faces` referential integrity is the subset's `✅validation-report`
//! inference's job, not this diff constructor's.

use super::mutation::CreateShell;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepShell, SemioBrepSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateShell, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.shells.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A shell with id \"{}\" already exists.", payload.id), [payload.id.clone()]).await;
    }
    protocol::MutationOutcome::new(SemioBrepDiff { shells: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepShell { id: payload.id.clone(), faces: payload.faces.clone() }] }), ..Default::default() }).await
}
//#endregion 🔖️Diff
