//! 🔺️ `delete-edge` — sparse diff construction; removes the edge with this `id` from
//! `edges`. No cascade into any other collection (see the payload leaf's doc comment for
//! why). Absent `id` in `base` is `mutation.target-missing` (Error, empty diff — id-keyed
//! collections are unordered sets; a spurious `removed` entry for an absent id would make
//! `SemioBrepDiff::is_empty()` lie).

use super::mutation::DeleteEdge;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteEdge, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if !base.edges.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]).await;
    }
    protocol::MutationOutcome::new(SemioBrepDiff { edges: Some(NamedTripleDiff { removed: vec![payload.id.clone()], modified: vec![], added: vec![] }), ..Default::default() }).await
}
//#endregion 🔖️Diff
