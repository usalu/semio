//! 🔺️ Diff for `CreateObject`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::SemioKitDiff;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateObject, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.objects.iter().any(|o| o.child_id == payload.child_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An object child with id \"{}\" already exists.", payload.child_id), [payload.child_id.clone()]);
    }
    if let Err(message) = semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "object") {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, ["objects".to_string()]);
    }
    let item = store::os_store::artifact_child::ArtifactChild::new(payload.child_id.clone(), payload.target.clone());
    protocol::MutationOutcome::new(SemioKitDiff { objects: Some(IndexedTripleDiff { added: vec![IndexAdded { index: payload.at.map_or(base.objects.len(), |at| at.min(base.objects.len())), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
