mod artifact_inference_wire_tests {
    use super::*;

    async fn metadata(inference_schema: &'static str) -> ArtifactInferenceServiceMetadata {
        ArtifactInferenceServiceMetadata {
            owner: "s.test",
            artifact_kind: "s.test",
            artifact_schema: "s.test.schema",
            artifact_schema_version: 1,
            document_schema: "s.test.document",
            document_schema_version: 1,
            inference_schema,
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
        }
    }

    fn echo(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
        Ok(ArtifactInferenceExecution { canonical_payload: request.canonical_payload.to_vec(), diagnostics: Vec::new(), validity: "valid".into(), quality: "complete".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
    }

    fn cancel(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
        cancel_artifact_inference(request.cancellation_id)?;
        echo(request)
    }

    async fn request(inference_schema: &str) -> WireArtifactInferenceRequest {
        WireArtifactInferenceRequest {
            wire_version: ARTIFACT_INFERENCE_WIRE_VERSION,
            owner: "s.test".into(),
            artifact_kind: "s.test".into(),
            artifact_schema: "s.test.schema".into(),
            artifact_schema_version: 1,
            document_schema: "s.test.document".into(),
            document_schema_version: 1,
            inference_schema: inference_schema.into(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            revision: 3,
            generation: 4,
            source_dialect: "s.test.standard.v1.dialect.canonical".into(),
            policy: vec![1],
            budgets: WireArtifactInferenceBudget { allocation_bytes: 128, work_units: 2, recursion_depth: 1 },
            cancellation_id: format!("cancel-{inference_schema}"),
            previous_state: None,
            requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
            canonical_payload: vec![2],
            dependencies: vec![("s.dependency".into(), vec![3])],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn request_rejects_unknown_wire_version_before_registry_lookup() {
        let request = WireArtifactInferenceRequest {
            wire_version: 1,
            owner: "test".into(),
            artifact_kind: "test".into(),
            artifact_schema: "test".into(),
            artifact_schema_version: 1,
            document_schema: "test".into(),
            document_schema_version: 1,
            inference_schema: "test.inference".into(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            revision: 3,
            generation: 4,
            source_dialect: "s.stdio.test.standard.v1.dialect.canonical".into(),
            policy: Vec::new(),
            budgets: WireArtifactInferenceBudget { allocation_bytes: 1, work_units: 1, recursion_depth: 1 },
            cancellation_id: "test-cancel".into(),
            previous_state: None,
            requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
            canonical_payload: Vec::new(),
            dependencies: Vec::new(),
        };
        let error = wire_artifact_infer(protocol::json::to_json_string(&request).as_bytes()).await.unwrap_err();
        assert_eq!(error.code, "artifact-inference.wire-version");
    }

    #[semio_framework_async_macros::async_test]
    async fn wire_execution_preserves_every_echoed_request_fact() {
        let mut registry = ArtifactInferenceServiceRegistry::new();
        registry.register(ArtifactInferenceService::new(metadata("s.test.inference.echo").await, echo)).unwrap();
        let request = request("s.test.inference.echo").await;
        let bytes = wire_artifact_infer_from(&registry, protocol::json::to_json_string(&request).as_bytes()).unwrap();
        let result: WireArtifactInferenceResult = protocol::json::from_json_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
        assert_eq!(result.policy, request.policy);
        assert_eq!(result.budgets, request.budgets);
        assert_eq!(result.previous_state, request.previous_state);
        assert_eq!(result.requested_cache_mode, request.requested_cache_mode);
        assert_eq!(result.dependencies, request.dependencies);
        assert_eq!(result.actual_cache_mode, WireArtifactInferenceCacheMode::Cold);
    }

    #[semio_framework_async_macros::async_test]
    async fn wire_execution_observes_midflight_cancellation() {
        let mut registry = ArtifactInferenceServiceRegistry::new();
        registry.register(ArtifactInferenceService::new(metadata("s.test.inference.cancel").await, cancel)).unwrap();
        let request = request("s.test.inference.cancel");
        let error = wire_artifact_infer_from(&registry, protocol::json::to_json_string(&request.await).as_bytes()).unwrap_err();
        assert_eq!(error.code, "artifact-inference.cancelled");
    }
}
