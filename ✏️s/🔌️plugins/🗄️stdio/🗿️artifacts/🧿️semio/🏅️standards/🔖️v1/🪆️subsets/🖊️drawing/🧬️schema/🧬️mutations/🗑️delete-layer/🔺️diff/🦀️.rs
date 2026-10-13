//! 🔺️ Diff for `DeleteLayer`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::SemioDrawingDiff;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteLayer, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    match base.layers.iter().position(|l| l.id == payload.id) {
        Some(index) => protocol::MutationOutcome::new(SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }),
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.id), [payload.id.clone()]),
    }
}
//#endregion 🔖️Diff
