use crate::BoundedJob;
use semio_framework_value_derive::ToValue;
use semio_framework_value_derive::FromValue;
use std::future::Future;
use semio_framework_value::ValueError;
use crate::infer::{DIAGNOSTIC_MAX_BYTES, DIAGNOSTIC_MAX_ITEMS, InferenceBridge, InferenceBridgeError, LOSSLESS_MAX_BYTES, OriginalInferenceJob, PREVIEW_MAX_BYTES, decode_request, encode_result, job_infer};
use semio_framework_job::{Generation, Operation, OperationId, RevisionId};
#[cfg(test)]
use crate::infer::LosslessInferenceItem;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_decl_registry::{ArtifactInferenceExecution, ArtifactInferenceExecutionRequest, ArtifactInferenceServiceMetadata, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};
use semio_plugin_plugin_program::WireArtifactInferenceRequest;

const TEST_METADATA: ArtifactInferenceServiceMetadata = ArtifactInferenceServiceMetadata {
    owner: "s.jobtest",
    artifact_kind: "s.jobtest.widget",
    artifact_schema: "widget.doc",
    artifact_schema_version: 1,
    inference_schema: "jobtest.echo",
    inference_schema_version: 1,
    algorithm_version: 1,
    policy_version: 1,
    payload: None,
};

fn echo_infer(request: &ArtifactInferenceExecutionRequest<'_>, cx:&mut semio_plugin_decl_registry::ArtifactInferenceStepContext<'_>) -> Result<semio_plugin_decl_registry::ArtifactInferenceExecutionStep, semio_plugin_decl_registry::ArtifactInferenceExecutionError> {
    Ok(ArtifactInferenceExecution { retirement_progress: Default::default(), canonical_payload: Some(request.canonical_payload.to_vec()), diagnostics: Vec::new(), validity: "valid".into(), quality: "exact".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() }.into_step(true))
}

/// 🪪️ Every fixture request carries its OWN `cancellation_id`: the in-flight inference registry is
/// process-global, so two tests reusing one id race each other for the same slot.
#[doc(hidden)]
pub fn request_bytes(cancellation_id: &str) -> Vec<u8> {
    let request = WireArtifactInferenceRequest {
        wire_version: semio_plugin_plugin_program::ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: TEST_METADATA.owner.into(),
        artifact_kind: TEST_METADATA.artifact_kind.into(),
        artifact_schema: TEST_METADATA.artifact_schema.into(),
        artifact_schema_version: TEST_METADATA.artifact_schema_version,
        inference_schema: TEST_METADATA.inference_schema.into(),
        inference_schema_version: TEST_METADATA.inference_schema_version,
        algorithm_version: TEST_METADATA.algorithm_version,
        policy_version: TEST_METADATA.policy_version,
        revision: 1,
        generation: 1,
        source_dialect: "s.jobtest.widget.standard.v1.dialect.canonical".into(),
        policy: Vec::new(),
        budgets: WireArtifactInferenceBudget { allocation_bytes: 1 << 20, work_units: 1000, recursion_depth: 4 },
        retained: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 7, maximum_copy_bytes: 3, maximum_capacity_bytes: 129, maximum_release_bytes: 4096, maximum_depth: 2 },
        cancellation_id: cancellation_id.to_string(),
        previous_state: None,
        requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
        canonical_payload: vec![9, 8, 7],
        dependencies: Vec::new(),
    };
    semio_framework_pack_json::to_json_string(&request).into_bytes()
}

/// 🚪️ The exact original raw loan is preserved before an admitted boxed inference birth.
#[test]
fn original_inference_job_admission_preserves_owned_input_and_checkpoint_before_birth(){
 use semio_framework_job::{StepContext,StepBudget,root_cancel_token};
 let mut input=Some(request_bytes("original-admission"));let mut checkpoint=Some(b"original checkpoint".to_vec());let input_pointer=input.as_ref().unwrap().as_ptr();let checkpoint_pointer=checkpoint.as_ref().unwrap().as_ptr();let mut sequence=0;let cancel=root_cancel_token();let mut receipt=Default::default();
 let grant=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:128};let mut cx=StepContext::new(OperationId(71),Generation(3),StepBudget::new(1,u64::MAX,grant),cancel,||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job_infer(71,&mut input,&mut checkpoint,&mut cx));assert!(result.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(input.as_ref().unwrap().as_ptr(),input_pointer);assert_eq!(checkpoint.as_ref().unwrap().as_ptr(),checkpoint_pointer);
}

#[test]
fn interactive_bridge_coalesces_preview_but_backpressures_lossless_items() {
    let operation = Operation::new(OperationId(7), RevisionId(11), Generation(3), 0);
    let mut bridge = InferenceBridge::new(operation);
    bridge.publish_preview(vec![1]).expect("first preview");
    bridge.publish_preview(vec![2, 3]).expect("latest preview");
    assert_eq!(bridge.take_preview().expect("coalesced preview").payload, vec![2, 3]);

    bridge
        .publish_lossless(LosslessInferenceItem::Checkpoint(semio_framework_job::Checkpoint { state: semio_framework_job_payload::RetainedJobPayload::empty(semio_framework_job_payload::JobPayloadStream::CheckpointState), applied_progress: 1 }))
        .expect("first checkpoint");
    bridge
        .publish_lossless(LosslessInferenceItem::Checkpoint(semio_framework_job::Checkpoint { state: semio_framework_job_payload::RetainedJobPayload::empty(semio_framework_job_payload::JobPayloadStream::CheckpointState), applied_progress: 2 }))
        .expect("second checkpoint");
    assert!(matches!(bridge.publish_lossless(LosslessInferenceItem::TestBytes(1)), Err(InferenceBridgeError::Saturated { .. })));
    assert!(matches!(bridge.take_lossless(), Some(LosslessInferenceItem::Checkpoint(checkpoint)) if checkpoint.applied_progress == 1));
    assert!(matches!(bridge.take_lossless(), Some(LosslessInferenceItem::Checkpoint(checkpoint)) if checkpoint.applied_progress == 2));
    assert!(matches!(bridge.publish_preview(vec![0; PREVIEW_MAX_BYTES + 1]), Err(InferenceBridgeError::Oversized { channel: "preview", .. })));
    bridge.publish_lossless(LosslessInferenceItem::TestBytes(LOSSLESS_MAX_BYTES)).expect("exact byte maximum");
    assert_eq!(bridge.lossless_len, 1);
    assert_eq!(bridge.lossless_bytes, LOSSLESS_MAX_BYTES);
    assert!(matches!(bridge.take_lossless(), Some(LosslessInferenceItem::TestBytes(LOSSLESS_MAX_BYTES))));
    assert!(matches!(bridge.publish_lossless(LosslessInferenceItem::TestBytes(LOSSLESS_MAX_BYTES + 1)), Err(InferenceBridgeError::Oversized { channel: "checkpoint-commit", .. })));
}

#[test]
fn interactive_bridge_diagnostic_ring_is_item_and_byte_bounded() {
    let operation = Operation::new(OperationId(8), RevisionId(11), Generation(3), 0);
    let mut bridge = InferenceBridge::new(operation);
    for index in 0..(DIAGNOSTIC_MAX_ITEMS + 9) {
        bridge.publish_diagnostic(vec![index as u8; 8]);
    }
    assert_eq!(bridge.diagnostics.len(), DIAGNOSTIC_MAX_ITEMS);
    assert!(bridge.diagnostic_bytes <= DIAGNOSTIC_MAX_BYTES);
    assert_eq!(bridge.diagnostics.front().expect("ring head").payload, vec![9; 8]);
}

/// 🧾️ G3: an interactive result states what the host observed, not what a request echo or a constant claims.
#[test]
fn an_interactive_result_forwards_the_jobs_resume_state_and_claims_no_fidelity_it_was_not_told() {
    let request = decode_request(&request_bytes("jobtest-cancel-encode")).expect("the fixture request decodes");
    let decoded = |bytes: Vec<u8>| -> semio_plugin_plugin_program::WireArtifactInferenceResult { semio_framework_pack_json::from_json_str(std::str::from_utf8(&bytes).expect("result UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the result decodes") };

    let resumable = decoded(encode_result(request.clone(), vec![1, 2, 3], Some(vec![4, 5]),RetainedCloneProgress{copied_items:7,copied_bytes:23,retained_capacity_bytes:0,released_bytes:65536}).expect("encodes"));
    assert_eq!((resumable.canonical_payload, resumable.previous_state, resumable.complete), (Some(vec![1, 2, 3]), Some(vec![4, 5]), true));
    assert_eq!((resumable.validity.as_str(), resumable.quality.as_str()), ("valid", "unreported"));
    assert!(resumable.diagnostics.is_empty());
    assert_eq!(resumable.retirement_progress,RetainedCloneProgress{copied_items:7,copied_bytes:23,retained_capacity_bytes:0,released_bytes:65536});

    let mut resumed = request;
    resumed.previous_state = Some(vec![9, 9]);
    let stateless = decoded(encode_result(resumed, vec![1], None,RetainedCloneProgress::default()).expect("encodes"));
    assert_eq!(stateless.previous_state, None, "the request's own previous state is not the job's resume state");
}

#[doc(hidden)]
pub fn completed_child_demands(_request:&ArtifactInferenceExecutionRequest<'_>,_copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}

/// ⚙️ One paid inference turn advances multiple slices and the same raw source closes on cancellation.
#[test]
fn original_inference_job_batches_paid_slices_and_preserves_exact_wire(){
    use semio_framework_actor::OriginalJobStep as JobStep;
    use semio_plugin_decl_registry::artifact_inference::ArtifactInferenceService;
    use semio_plugin_testing_laws::OriginalFixtureCaller;

    semio_plugin_extension::artifact_inference::register_artifact_inference_service(
        ArtifactInferenceService::new(TEST_METADATA,echo_infer,completed_child_demands)
    ).unwrap();
    let expected=serde_json::json!([0,1,9,127,128,255,0,42]);
    let grant=RetainedCloneGrant{
        maximum_items:96,maximum_copy_bytes:8192,maximum_capacity_bytes:1048576,
        maximum_release_bytes:1048576,maximum_depth:128,
    };
    for cancelled in [false,true]{
        let mut input:serde_json::Value=serde_json::from_slice(&request_bytes("original-paid-slices")).unwrap();
        input["canonicalPayload"]=expected.clone();
        let raw=serde_json::to_vec(&input).unwrap();
        let pointer=raw.as_ptr();
        let mut caller=OriginalFixtureCaller::admit().unwrap();
        let mut cx=caller.context();
        let mut job=OriginalInferenceJob::new(raw,None,cx.operation().0,cx.generation().0);
        let before=cx.retained_progress();
        let withheld=RetainedCloneGrant{maximum_items:0,..grant};
        cx.with_retained_loan(withheld,|turn|{
            assert!(matches!(job.step_original(turn),JobStep::Running(None)));
            Ok(())
        }).unwrap();
        assert_eq!(cx.retained_progress(),before);
        cx.with_retained_loan(grant,|turn|{
            assert!(matches!(job.step_original(turn),JobStep::Running(None)));
            Ok(())
        }).unwrap();
        let after=cx.retained_progress();
        assert!(after.copied_items-before.copied_items>1);
        assert!(after.copied_items-before.copied_items<=grant.maximum_items);
        assert_eq!(job.input.as_ref().unwrap().as_ptr(),pointer);
        if cancelled{
            cx.cancel_token().cancel_now();
            for _ in 0..OriginalFixtureCaller::maximum_turns(){
                let demand=job.retirement_demands(grant.maximum_copy_bytes).unwrap();
                let selected=RetainedCloneGrant{
                    maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,
                    maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1),
                };
                if cx.with_retained_loan(selected,|turn|job.close_step(turn)).unwrap(){break;}
            }
            assert!(job.terminal_drop_is_shallow());
        }else{
            let mut output=None;
            for _ in 0..OriginalFixtureCaller::maximum_turns(){
                cx.with_retained_loan(grant,|turn|{
                    match job.step_original(turn){
                        JobStep::Done(bytes)=>output=Some(bytes),
                        JobStep::Failed(bytes)=>panic!("original inference refused: {}",String::from_utf8_lossy(&bytes)),
                        _=>{},
                    }
                    Ok(())
                }).unwrap();
                if output.is_some(){break;}
            }
            let result:serde_json::Value=serde_json::from_slice(&output.expect("paid inference finishes")).unwrap();
            assert_eq!(result["canonicalPayload"],expected);
            assert_eq!(result["complete"],true);
            assert!(job.terminal_drop_is_shallow());
        }
        let fuel=cx.fuel_remaining();
        drop(cx);
        caller.retire(fuel).unwrap();
    }
}

/// ♻️ The original caller closes both raw sources under selected grants and observes each physical release.
#[test]
fn original_inference_job_close_keeps_raw_checkpoint_system_receipts_and_drop_zero(){
    use semio_plugin_testing_laws::OriginalFixtureCaller;
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    for copy in [1,3,64]{
        let input=b"same raw original loan".to_vec();
        let checkpoint=b"same original checkpoint".to_vec();
        let (input_ptr,checkpoint_ptr)=(input.as_ptr(),checkpoint.as_ptr());
        let original=input.capacity()+checkpoint.capacity();
        let mut caller=OriginalFixtureCaller::admit().unwrap();
        let mut cx=caller.context();
        let (mut owner,birth)=observe(||OriginalInferenceJob::new(input,Some(checkpoint),cx.operation().0,cx.generation().0));
        assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));
        let zero=RetainedCloneGrant{maximum_copy_bytes:copy,maximum_depth:128,..Default::default()};
        let before=cx.retained_progress();
        let (result,heap)=observe(||cx.with_retained_loan(zero,|turn|owner.close_step(turn)));
        assert!(!result.unwrap());
        assert_eq!(cx.retained_progress(),before);
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(owner.input.as_ref().unwrap().as_ptr(),input_ptr);
        assert_eq!(owner.restored.as_ref().unwrap().as_ptr(),checkpoint_ptr);
        let (mut born,mut freed)=(0,0);
        for _ in 0..OriginalFixtureCaller::maximum_close_turns(){
            let demand=owner.retirement_demands(copy).unwrap();
            let grant=RetainedCloneGrant{
                maximum_items:1,maximum_copy_bytes:copy.max(demand.copy_bytes),
                maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,
                maximum_depth:demand.depth.max(1),
            };
            let before=cx.retained_progress();
            let (result,heap)=observe(||cx.with_retained_loan(grant,|turn|owner.close_step(turn)));
            let after=cx.retained_progress();
            let progress=RetainedCloneProgress{
                copied_items:after.copied_items-before.copied_items,
                copied_bytes:after.copied_bytes-before.copied_bytes,
                retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,
                released_bytes:after.released_bytes-before.released_bytes,
            };
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            born+=heap.requested_bytes;
            freed+=heap.released_bytes;
            if let Some(source)=&owner.input{assert_eq!(source.as_ptr(),input_ptr);}
            if let Some(source)=&owner.restored{assert_eq!(source.as_ptr(),checkpoint_ptr);}
            if result.unwrap(){break;}
            assert_ne!(progress,RetainedCloneProgress::default(),"funded close stalled for copy {copy}: {demand:?}");
        }
        assert!(owner.terminal_drop_is_shallow());
        assert_eq!(original+born,freed);
        let (_,heap)=observe(||drop(owner));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let fuel=cx.fuel_remaining();
        drop(cx);
        caller.retire(fuel).unwrap();
    }
}
