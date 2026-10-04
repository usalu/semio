use super::*;

#[semio_framework_async_macros::async_test]
async fn only_exactly_echoed_guest_results_are_publishable() {
    let request = InferenceRouteRequest {
        wire_version: 2,
        owner: "s.test".into(),
        artifact_kind: "s.test".into(),
        artifact_schema: "s.test".into(),
        artifact_schema_version: 1,
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

/// 🪆️ LAW (design §20.15): a composed document's owned children travel with its inference — the requester's `child:` entries stay
/// first on the routed request, the `depends_on` results follow, and every dependency request carries the same children.
#[semio_framework_async_macros::async_test]
async fn a_composed_documents_children_reach_every_routed_inference_request() {
    let child = |slot: &str, id: &str, pack: u8| (format!("child:{slot}/{id}"), vec![pack]);
    let requested = vec![child("content", "c1", 1), child("content", "c2", 2)];
    let base = InferenceRouteRequest {
        wire_version: 2,
        owner: "s.test".into(),
        artifact_kind: "s.test".into(),
        artifact_schema: "s.test".into(),
        artifact_schema_version: 1,
        inference_schema: "s.test.summary".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        revision: 7,
        generation: 9,
        source_dialect: "s.test.standard.v1.dialect.canonical".into(),
        policy: Vec::new(),
        budgets: InferenceRouteBudget { allocation_bytes: 128, work_units: 4, recursion_depth: 2 },
        cancellation_id: "cancel-1".into(),
        previous_state: None,
        requested_cache_mode: InferenceRouteCacheMode::Cold,
        canonical_payload: vec![9],
        dependencies: requested.clone(),
    };
    let dependency = GuestArtifactInferenceMetadata {
        owner: "s.test".into(),
        artifact_kind: "s.test".into(),
        artifact_schema: "s.test".into(),
        artifact_schema_version: 1,
        inference_schema: "s.test.outline".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: None,
        depends_on: Vec::new(),
        payload: None,
    };
    let dependency_request = build_dependency_inference_request(&base, &dependency).await;
    assert_eq!(dependency_request.dependencies, requested, "a dependency inference reads the same owned children");
    let routed = routed_inference_dependencies(base.dependencies.clone(), vec![("s.test.outline".into(), vec![7])]);
    assert_eq!(routed, vec![child("content", "c1", 1), child("content", "c2", 2), ("s.test.outline".into(), vec![7])]);
    assert_eq!(routed_inference_dependencies(Vec::new(), vec![("s.test.outline".into(), vec![7])]), vec![("s.test.outline".to_string(), vec![7])], "a non-composed request is unchanged");
}
