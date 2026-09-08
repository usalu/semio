
use super::*;

#[semio_framework_async_macros::async_test]
async fn only_exactly_echoed_guest_results_are_publishable() {
    let request = InferenceRouteRequest {
        wire_version: 2,
        owner: "s.test".into(),
        artifact_kind: "s.test".into(),
        artifact_schema: "s.test".into(),
        artifact_schema_version: 1,
        document_schema: "s.test.document".into(),
        document_schema_version: 1,
        inference_schema: "s.test.inference".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        revision: 7,
        generation: 9,
        source_dialect: "s.test.standard.v1.dialect.canonical".into(),
        policy: vec![1],
        budgets: InferenceRouteBudget { allocation_bytes: 128, work_units: 1, recursion_depth: 1 },
        cancellation_id: "cancel-1".into(),
        previous_state: None,
        requested_cache_mode: InferenceRouteCacheMode::Cold,
        canonical_payload: vec![1],
        dependencies: vec![("s.dependency".into(), vec![2])],
    };
    let valid = InferenceRouteResult {
        wire_version: request.wire_version,
        owner: request.owner.clone(),
        artifact_kind: request.artifact_kind.clone(),
        artifact_schema: request.artifact_schema.clone(),
        artifact_schema_version: request.artifact_schema_version,
        document_schema: request.document_schema.clone(),
        document_schema_version: request.document_schema_version,
        inference_schema: request.inference_schema.clone(),
        inference_schema_version: request.inference_schema_version,
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        revision: request.revision,
        generation: request.generation,
        source_dialect: request.source_dialect.clone(),
        policy: request.policy.clone(),
        budgets: request.budgets.clone(),
        cancellation_id: request.cancellation_id.clone(),
        previous_state: request.previous_state.clone(),
        requested_cache_mode: request.requested_cache_mode.clone(),
        canonical_payload: request.canonical_payload.clone(),
        dependencies: request.dependencies.clone(),
        complete: true,
        actual_cache_mode: request.requested_cache_mode.clone(),
    };
    assert!(validate_inference_echo(&request, &valid).await.is_ok());
    let stale = InferenceRouteResult { generation: 8, ..valid };
    assert!(matches!(validate_inference_echo(&request, &stale).await, Err(PluginHostError::Plugin(_))));
}

#[semio_framework_async_macros::async_test]
async fn live_revision_and_generation_change_rejects_the_terminal_candidate() {
    let router = ArtifactInferenceRouter::new();
    router.live_commits.lock().expect("live authority").insert("assembly-stale".into(), (7, 9));
    router.set_live_revision_generation("assembly-stale", 8, 10).await.expect("model actor freshness update");
    let live = router.live_commits.lock().expect("live authority").remove("assembly-stale").expect("active authority");
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(7), semio_framework_job::Generation(9), 0);
    assert!(matches!(
        semio_framework_job::validate_commit(&operation, semio_framework_job::RevisionId(live.0), semio_framework_job::Generation(live.1)),
        semio_framework_job::CommitValidation::Stale { live_revision: semio_framework_job::RevisionId(8), live_generation: semio_framework_job::Generation(10) }
    ));
}
