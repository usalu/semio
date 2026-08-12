//! 🔺️ `delete-layer` — sparse diff construction; an absent id is a no-op.

use super::mutation::DeleteLayer;
use crate::artifacts::semio::standards::v1::engine::triples::IndexedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLayer, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match base.layers.iter().position(|l| l.id == payload.id) {
        Some(index) => SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) },
        None => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
