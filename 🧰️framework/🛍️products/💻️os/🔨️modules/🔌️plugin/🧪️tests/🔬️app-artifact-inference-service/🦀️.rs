mod artifact_inference_service_tests {
    use super::*;

    fn echo(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
        Ok(ArtifactInferenceExecution { canonical_payload: request.canonical_payload.to_vec(), diagnostics: Vec::new(), validity: "valid".into(), quality: "complete".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
    }

    fn reject(_request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
        Err(ArtifactInferenceExecutionError::new("test.rejected", "rejected"))
    }

    async fn metadata(artifact_kind: &'static str, inference_schema: &'static str) -> ArtifactInferenceServiceMetadata {
        ArtifactInferenceServiceMetadata {
            owner: "test",
            artifact_kind,
            artifact_schema: artifact_kind,
            artifact_schema_version: 1,
            document_schema: "test.document",
            document_schema_version: 1,
            inference_schema,
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_inference_registry_is_order_independent_and_idempotent() {
        let alpha = ArtifactInferenceService::new(metadata("s.test.alpha", "s.test.alpha.inference").await, echo);
        let beta = ArtifactInferenceService::new(metadata("s.test.beta", "s.test.beta.inference").await, echo);
        let mut forward = ArtifactInferenceServiceRegistry::new();
        forward.register(alpha).unwrap();
        forward.register(beta).unwrap();
        forward.register(alpha).unwrap();
        let mut reverse = ArtifactInferenceServiceRegistry::new();
        reverse.register(beta).unwrap();
        reverse.register(alpha).unwrap();
        assert_eq!(forward.metadata(), reverse.metadata());
        let budget = WireArtifactInferenceBudget { allocation_bytes: 16, work_units: 1, recursion_depth: 1 };
        let request = ArtifactInferenceExecutionRequest { policy: &[], budgets: &budget, cancellation_id: "test", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold, canonical_payload: b"pack", dependencies: &[] };
        assert_eq!(forward.infer("s.test.alpha", "s.test.alpha.inference", &request).unwrap().canonical_payload, b"pack");
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_inference_registry_rejects_any_conflicting_duplicate() {
        let identity = metadata("s.test.conflict", "s.test.conflict.inference").await;
        let first = ArtifactInferenceService::new(identity, echo);
        assert_eq!(first.executable_identity(), ArtifactInferenceService::new(identity, echo).executable_identity());
        assert_ne!(first.executable_identity(), ArtifactInferenceService::new(identity, reject).executable_identity());
        let mut changed_metadata = metadata("s.test.conflict", "s.test.conflict.inference").await;
        changed_metadata.algorithm_version = 2;
        let mut registry = ArtifactInferenceServiceRegistry::new();
        registry.register(first).unwrap();
        let executable_error = registry.register(ArtifactInferenceService::new(identity, reject)).unwrap_err();
        let ArtifactInferenceRegistrationError::Conflict(conflict) = executable_error else {
            panic!("in-memory registry must report a registration conflict");
        };
        assert_eq!(conflict.existing, conflict.incoming);
        let error = registry.register(ArtifactInferenceService::new(changed_metadata, reject)).unwrap_err();
        let ArtifactInferenceRegistrationError::Conflict(conflict) = error else {
            panic!("in-memory registry must report a registration conflict");
        };
        assert_eq!(conflict.key.artifact_kind, "s.test.conflict");
        assert_eq!(conflict.existing.algorithm_version, 1);
        assert_eq!(conflict.incoming.algorithm_version, 2);
    }
}
