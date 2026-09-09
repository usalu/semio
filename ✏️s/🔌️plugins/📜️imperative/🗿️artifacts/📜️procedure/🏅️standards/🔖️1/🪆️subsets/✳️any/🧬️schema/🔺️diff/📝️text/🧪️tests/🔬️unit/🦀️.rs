use super::*;
use crate::schema::default_snapshot;

#[semio_framework_async_macros::async_test]
async fn imperative_diff_absorb_whole_artifact_wins() {
    let mut diff = ProcedureDiff { flow: Some(crate::procedure_flow_child_with_owner(&crate::Path::new())), ..Default::default() };
    let replacement = ProcedureDiff { artifact: Some(Box::new(ProcedureArtifact::default())), ..Default::default() };
    diff.absorb(replacement);
    assert!(diff.artifact.is_some());
    assert!(diff.flow.is_none());
}

/// 🔁 Replaces the retired `path_delta_remove_round_trips_via_apply` — whole-list
/// `ProcedurePathDelta` deltas no longer exist, since composed children are opaque and a diff
/// only ever whole-handle-replaces `flow` — with the equivalent real-behavior law: a `flow`
/// handle minted from an edited working scene applies as a clean whole-handle replace.
#[semio_framework_async_macros::async_test]
async fn flow_handle_replace_round_trips_via_apply() {
    let base = default_snapshot();
    let mut path = crate::procedure_working_scene(&base).path;
    assert!(path.steps.iter().any(|step| step.id == "step-1"));
    path.steps.retain(|step| step.id != "step-1");
    let diff = crate::diff_replace_flow(&path);
    let next = diff.apply(&base).expect("valid mutation diff");
    let next_path = crate::procedure_working_scene(&next).path;
    assert!(next_path.steps.iter().all(|step| step.id != "step-1"));
}
