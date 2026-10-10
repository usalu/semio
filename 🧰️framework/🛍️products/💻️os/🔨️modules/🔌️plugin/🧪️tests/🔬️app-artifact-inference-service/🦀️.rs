mod artifact_inference_service_tests {
    use super::*;

    fn completed_child_demands(_request:&ArtifactInferenceExecutionRequest<'_>,_copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
    fn contextual_completed_child_demands(_request:&ArtifactInferenceExecutionRequest<'_>,_context:&dyn std::any::Any,_copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
    fn echo(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError> {
        Ok(ArtifactInferenceExecution { retirement_progress: Default::default(), canonical_payload: Some(request.canonical_payload.to_vec()), diagnostics: Vec::new(), validity: "valid".into(), quality: "complete".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() }.into_step(true))
    }

    fn reject(_request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError> {
        Err(ArtifactInferenceExecutionError::new("test.rejected", "rejected"))
    }

    async fn metadata(artifact_kind: &'static str, inference_schema: &'static str) -> ArtifactInferenceServiceMetadata {
        ArtifactInferenceServiceMetadata {
            owner: "test",
            artifact_kind,
            artifact_schema: artifact_kind,
            artifact_schema_version: 1,
            inference_schema,
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            payload: None,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_inference_registry_is_order_independent_and_idempotent() {
        let alpha = ArtifactInferenceService::new(metadata("s.test.alpha", "s.test.alpha.inference").await, echo,completed_child_demands);
        let beta = ArtifactInferenceService::new(metadata("s.test.beta", "s.test.beta.inference").await, echo,completed_child_demands);
        let mut forward = ArtifactInferenceServiceRegistry::new();
        forward.register(alpha).unwrap();
        forward.register(beta).unwrap();
        forward.register(alpha).unwrap();
        let mut reverse = ArtifactInferenceServiceRegistry::new();
        reverse.register(beta).unwrap();
        reverse.register(alpha).unwrap();
        assert_eq!(forward.metadata(), reverse.metadata());
        let budget = WireArtifactInferenceBudget { allocation_bytes: 16, work_units: 1, recursion_depth: 1 };
        let request = ArtifactInferenceExecutionRequest { operation:17,generation:9,cancelled:false, policy: &[], budgets: &budget, retained: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 7, maximum_copy_bytes: 3, maximum_capacity_bytes: 129, maximum_release_bytes: 4096, maximum_depth: 2 }, cancellation_id: "test", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold, canonical_payload: b"pack", dependencies: &[] };
        assert_eq!(forward.infer("s.test.alpha", "s.test.alpha.inference", &request).unwrap().execution.unwrap().canonical_payload, Some(b"pack".to_vec()));
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_inference_registry_rejects_any_conflicting_duplicate() {
        let identity = metadata("s.test.conflict", "s.test.conflict.inference").await;
        let first = ArtifactInferenceService::new(identity, echo,completed_child_demands);
        assert_eq!(first.executable_identity(), ArtifactInferenceService::new(identity, echo,completed_child_demands).executable_identity());
        assert_ne!(first.executable_identity(), ArtifactInferenceService::new(identity, reject,completed_child_demands).executable_identity());
        let mut changed_metadata = metadata("s.test.conflict", "s.test.conflict.inference").await;
        changed_metadata.algorithm_version = 2;
        let mut registry = ArtifactInferenceServiceRegistry::new();
        registry.register(first).unwrap();
        let executable_error = registry.register(ArtifactInferenceService::new(identity, reject,completed_child_demands)).unwrap_err();
        let ArtifactInferenceRegistrationError::Conflict(conflict) = executable_error else {
            panic!("in-memory registry must report a registration conflict");
        };
        assert_eq!(conflict.existing, conflict.incoming);
        let error = registry.register(ArtifactInferenceService::new(changed_metadata, reject,completed_child_demands)).unwrap_err();
        let ArtifactInferenceRegistrationError::Conflict(conflict) = error else {
            panic!("in-memory registry must report a registration conflict");
        };
        assert_eq!(conflict.key.artifact_kind, "s.test.conflict");
        assert_eq!(conflict.existing.algorithm_version, 1);
        assert_eq!(conflict.incoming.algorithm_version, 2);
    }
    fn contextual_echo(request: &ArtifactInferenceExecutionRequest<'_>, context: &dyn std::any::Any) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError> {
        let expected = context.downcast_ref::<Vec<u8>>().ok_or_else(|| ArtifactInferenceExecutionError::new("test.context", "wrong context"))?;
        assert_eq!(request.canonical_payload, expected);
        echo(request)
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_inference_context_preserves_execution_and_registry_identity() {
        let identity = metadata("s.test.context", "s.test.context.inference").await;
        let service = ArtifactInferenceService::new_contextual(identity, contextual_echo,contextual_completed_child_demands);
        let budget = WireArtifactInferenceBudget { allocation_bytes: 16, work_units: 1, recursion_depth: 1 };
        let request = ArtifactInferenceExecutionRequest { operation:17,generation:9,cancelled:false, policy: &[], budgets: &budget, retained: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 7, maximum_copy_bytes: 3, maximum_capacity_bytes: 129, maximum_release_bytes: 4096, maximum_depth: 2 }, cancellation_id: "context", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold, canonical_payload: b"pack", dependencies: &[] };
        assert_eq!(service.infer(&request).err().unwrap().code, "artifact-inference.context-required");
        assert_eq!(service.infer_with_context(&request, &b"pack".to_vec()).unwrap().execution.unwrap().canonical_payload, echo(&request).unwrap().execution.unwrap().canonical_payload);
        assert_eq!(service.executable_identity(), ArtifactInferenceService::new_contextual(identity, contextual_echo,contextual_completed_child_demands).executable_identity());
        let mut registry = ArtifactInferenceServiceRegistry::new();
        registry.register(service).unwrap();
        registry.register(service).unwrap();
        assert!(registry.register(ArtifactInferenceService::new(identity, echo,completed_child_demands)).is_err());
        assert_eq!(registry.get(identity.artifact_kind, identity.inference_schema).unwrap().infer_with_context(&request, &b"pack".to_vec()).unwrap().execution.unwrap().canonical_payload, Some(b"pack".to_vec()));
    }

    /// 🪆️ LAW (design §20.15): an owned-child dependency key round-trips through its grammar, a malformed one is refused, and a
    /// request carrying a child dependency reaches the inference with its child decoded by `inference_child`.
    #[test]
    fn owned_child_dependencies_travel_with_the_request() {
        let key = inference_child_dependency("content", "flow-content-sha256-00ff");
        assert_eq!(key, "child:content/flow-content-sha256-00ff");
        assert_eq!(inference_child_dependency_parts(&key), Some(("content", "flow-content-sha256-00ff")));
        for malformed in ["child:", "child:content", "child:/id", "child:content/", "child:con tent/id", "content/id"] {
            assert_eq!(inference_child_dependency_parts(malformed), None, "{malformed}");
        }
        let child = NoConfig::default();
        let pack = <NoConfig as ArtifactPack>::encode_pack(&child);
        let budgets = WireArtifactInferenceBudget { allocation_bytes: 1 << 20, work_units: 8, recursion_depth: 4 };
        let dependencies = vec![(key.clone(), pack)];
        let request = ArtifactInferenceExecutionRequest { operation:17,generation:9,cancelled:false, policy: &[], budgets: &budgets, retained: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 7, maximum_copy_bytes: 3, maximum_capacity_bytes: 129, maximum_release_bytes: 4096, maximum_depth: 2 }, cancellation_id: "child-dependency", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold, canonical_payload: &[], dependencies: &dependencies };
        assert_eq!(inference_child::<NoConfig>(&request, "content", "flow-content-sha256-00ff").expect("the owned child decodes"), child);
        assert_eq!(inference_child::<NoConfig>(&request, "content", "absent").map_err(|error| error.code).unwrap_err(), "artifact-inference.child-missing");
    }
    /// 🪪️ The demand recipient borrows the exact original child and refuses equal replacement bytes.
    #[test]
    fn original_inference_service_demand_keeps_same_child_and_actual_source(){
        struct Child{operation:u64,generation:u64,source:Vec<u8>}
        fn pending(_request:&ArtifactInferenceExecutionRequest<'_>,_context:&dyn std::any::Any)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:Default::default(),terminal:false})}
        fn demand(request:&ArtifactInferenceExecutionRequest<'_>,context:&dyn std::any::Any,_copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{use semio_framework_value::{ValueError,ValueRefusalKind,retirement::RetireOwned};let child=context.downcast_ref::<Child>().ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original child demand lost its owner"))?;if child.operation!=request.operation||child.generation!=request.generation||child.source.as_ptr()!=request.canonical_payload.as_ptr()||child.source.len()!=request.canonical_payload.len(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original child demand source differs"))}Ok(semio_framework_value::RetirementDemand{capacity_bytes:child.source.retirement_birth_bytes().unwrap(),depth:1,..Default::default()})}
        let child=Child{operation:71,generation:3,source:b"actual retained child".to_vec()};let metadata=ArtifactInferenceServiceMetadata{owner:"test",artifact_kind:"test.child",artifact_schema:"test.child",artifact_schema_version:1,inference_schema:"test.child.inference",inference_schema_version:1,algorithm_version:1,policy_version:1,payload:None};let service=ArtifactInferenceService::new_contextual(metadata,pending,demand);let budget=WireArtifactInferenceBudget{allocation_bytes:1048576,work_units:1,recursion_depth:128};
        let mut request=ArtifactInferenceExecutionRequest{operation:71,generation:3,cancelled:true,policy:&[],budgets:&budget,retained:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128},cancellation_id:"child-71",previous_state:None,requested_cache_mode:WireArtifactInferenceCacheMode::Cold,canonical_payload:&child.source,dependencies:&[]};
        for copy in [1,3,64]{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||service.retirement_demands(&request,Some(&child),copy));let quoted=result.unwrap();assert!(quoted.capacity_bytes>0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(request.canonical_payload.as_ptr(),child.source.as_ptr());}
        let replacement=child.source.clone();request.canonical_payload=&replacement;assert!(service.retirement_demands(&request,Some(&child),3).is_err());eprintln!("[DEBUG] Registered inference demand borrows exact retained child and refuses equal replacement source; actual System0");
    }

}
