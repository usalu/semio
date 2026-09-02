//! 🔺️ Sparse diff builder for `ReplaceMeshResult` — `results.mesh` is always present (defaults to a
//! placeholder box), so there is no target-missing case; identical resubmission ⇒ Warning.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceMeshResult, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.mesh.mesh.target.artifact_id.starts_with("mesh-stage:") {
        return protocol::MutationOutcome::error("mutation.incomplete-mesh", "Private reconstruction staging handles are accepted only by CommitReconstruction.".into(), [payload.mesh.mesh.child_id.clone()]);
    }
    let mesh = (*payload.mesh).clone();
    if mesh == base.results.mesh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Mesh result is already up to date.".to_string());
    }
    let mut results = base.results.clone();
    results.mesh = mesh;
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
