//! 🔺️ `create-layer` — sparse diff construction; a FINAL-state index beyond the current length
//! clamps to append. A layer with this `id` already present in `base` is `mutation.duplicate-id`
//! (Fatal, empty diff — real entity-lifecycle safety, never a silent duplicate).

use super::mutation::CreateLayer;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLayer, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    if base.layers.iter().any(|l| l.id == payload.layer.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A layer with id \"{}\" already exists.", payload.layer.id), [payload.layer.id.clone()]);
    }
    let at = payload.index.min(base.layers.len());
    protocol::MutationOutcome::new(SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: at, item: payload.layer.clone() }] }) })
}
//#endregion 🔖️Diff
