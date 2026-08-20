//! 🔺️ `delete-layer` — sparse diff construction; an absent `id` is `mutation.target-missing`
//! (Error, empty diff — `layers` is a real id-keyed collection; a spurious `removed` entry for an
//! absent id would make the diff lie).

use super::mutation::DeleteLayer;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::IndexedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteLayer, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    match base.layers.iter().position(|l| l.id == payload.id) {
        Some(index) => protocol::MutationOutcome::new(SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }).await,
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.id), [payload.id.clone()]).await,
    }
}
//#endregion 🔖️Diff
