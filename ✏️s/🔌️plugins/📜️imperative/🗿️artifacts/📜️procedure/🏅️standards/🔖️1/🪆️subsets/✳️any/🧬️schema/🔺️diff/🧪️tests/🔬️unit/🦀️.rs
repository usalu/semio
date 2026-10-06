use super::*;
use crate::schema::default_snapshot;

/// 🔁️ A whole-artifact replacement absorbs every earlier field delta.
#[semio_framework_async_macros::async_test]
async fn imperative_diff_absorb_whole_artifact_wins() {
    let mut diff = ProcedureDiff { flow: Some(crate::procedure_flow_child_handle(&crate::Path::new())), ..Default::default() };
    let replacement = ProcedureDiff { artifact: Some(Box::new(ProcedureArtifact::default())), ..Default::default() };
    diff.absorb(replacement);
    assert!(diff.artifact.is_some());
    assert!(diff.flow.is_none());
}

/// 🔁️ A `flow` handle naming another child applies as a clean whole-handle replace and leaves `text` alone.
#[semio_framework_async_macros::async_test]
async fn flow_handle_replace_round_trips_via_apply() {
    let base = default_snapshot();
    let handle = crate::procedure_flow_child_handle(&crate::Path::new());
    let next = ProcedureDiff { flow: Some(handle.clone()), ..Default::default() }.apply(&base).expect("valid mutation diff");
    assert_eq!(next.flow, handle);
    assert_eq!(next.text, base.text);
}
