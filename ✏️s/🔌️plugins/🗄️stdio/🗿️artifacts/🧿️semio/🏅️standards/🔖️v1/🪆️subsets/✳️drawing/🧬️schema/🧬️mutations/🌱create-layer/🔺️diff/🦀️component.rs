//! 🔺️ `create-layer` — sparse diff construction; a FINAL-state index beyond the current length
//! clamps to append.

use super::mutation::CreateLayer;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLayer, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    let at = payload.index.min(base.layers.len());
    SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: at, item: payload.layer.clone() }] }) }
}
//#endregion 🔖️Diff
