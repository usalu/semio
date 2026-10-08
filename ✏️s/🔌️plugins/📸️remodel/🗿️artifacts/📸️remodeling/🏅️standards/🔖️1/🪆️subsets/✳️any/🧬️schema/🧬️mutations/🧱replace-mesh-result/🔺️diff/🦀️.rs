//! 🔺️ Sparse diff builder for `ReplaceMeshResult` — `results.mesh` is always present (defaults to a
//! placeholder box), so there is no target-missing case; a durable content handle must name complete
//! mesh content (`mutation.target-mismatch`); identical resubmission ⇒ Warning.
use crate::diff::{RemodelingDiff, RemodelingResultsDiff};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceMeshResult, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if let Some((content_id, chunk_count)) = crate::remodeling_mesh_content_handle_parts(&payload.mesh.mesh) {
        if !crate::remodeling_content_is_complete(&base.durable_artifacts, content_id, crate::RemodelingContentKind::Mesh, chunk_count) {
            return protocol::MutationOutcome::error("mutation.target-mismatch", "The mesh names durable content that is not complete.", [payload.mesh.mesh.child_id.clone()]);
        }
    }
    let mesh = (*payload.mesh).clone();
    if mesh == base.results.mesh {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Mesh result is already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { mesh: Some(mesh), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
