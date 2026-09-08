use super::super::*;
use super::*;
use crate::app::{ArtifactInferenceExecution, ArtifactInferenceExecutionRequest, ArtifactInferenceService, ArtifactInferenceServiceMetadata, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode, WireArtifactInferenceRequest};

const TEST_METADATA: ArtifactInferenceServiceMetadata = ArtifactInferenceServiceMetadata {
    owner: "s.jobtest",
    artifact_kind: "s.jobtest.widget",
    artifact_schema: "widget.doc",
    artifact_schema_version: 1,
    document_schema: "widget.doc",
    document_schema_version: 1,
    inference_schema: "jobtest.echo",
    inference_schema_version: 1,
    algorithm_version: 1,
    policy_version: 1,
};

fn echo_infer(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, crate::app::ArtifactInferenceExecutionError> {
    Ok(ArtifactInferenceExecution { canonical_payload: request.canonical_payload.to_vec(), diagnostics: Vec::new(), validity: "valid".into(), quality: "exact".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
}

fn request_bytes() -> Vec<u8> {
    let request = WireArtifactInferenceRequest {
        wire_version: crate::app::ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: TEST_METADATA.owner.into(),
        artifact_kind: TEST_METADATA.artifact_kind.into(),
        artifact_schema: TEST_METADATA.artifact_schema.into(),
        artifact_schema_version: TEST_METADATA.artifact_schema_version,
        document_schema: TEST_METADATA.document_schema.into(),
        document_schema_version: TEST_METADATA.document_schema_version,
        inference_schema: TEST_METADATA.inference_schema.into(),
        inference_schema_version: TEST_METADATA.inference_schema_version,
        algorithm_version: TEST_METADATA.algorithm_version,
        policy_version: TEST_METADATA.policy_version,
        revision: 1,
        generation: 1,
        source_dialect: "s.jobtest.widget.standard.v1.dialect.canonical".into(),
        policy: Vec::new(),
        budgets: WireArtifactInferenceBudget { allocation_bytes: 1 << 20, work_units: 1000, recursion_depth: 4 },
        cancellation_id: "jobtest-cancel-1".into(),
        previous_state: None,
        requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
        canonical_payload: vec![9, 8, 7],
        dependencies: Vec::new(),
    };
    protocol::json::to_json_string(&request).into_bytes()
}

/// 💡️ Registers a real native inference service (not mocked away) and drives `semio.infer`
/// through two real `step_job` slices to `Done`, proving `job_infer` really reaches
/// `crate::app::wire_artifact_infer` and not just `job.unknown-kind`.
#[semio_framework_async_macros::async_test]
async fn a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service() {
    let _ = crate::app::register_artifact_inference_service(ArtifactInferenceService::new(TEST_METADATA, echo_infer));
    start_job(200, JOB_KIND_INFER, &request_bytes()).await;

    match step_job(200, JobBudget { fuel: 1, deadline_ms: 1 }).await {
        JobStep::Running(Some(progress)) => {
            let (artifact_kind, inference_schema): (String, String) = serde_json::from_slice(&progress).expect("slice 1 progress decodes");
            assert_eq!(artifact_kind, TEST_METADATA.artifact_kind);
            assert_eq!(inference_schema, TEST_METADATA.inference_schema);
        }
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            panic!("slice 1 must be Running(Some(identity)), not fail before ever calling the registry: {} {}", fault.code.0, fault.message);
        }
        _ => panic!("slice 1 must be Running(Some(identity))"),
    }
    match step_job(200, JobBudget { fuel: 1, deadline_ms: 1 }).await {
        JobStep::Done(bytes) => {
            let result: crate::app::WireArtifactInferenceResult = protocol::json::from_json_str(std::str::from_utf8(&bytes).expect("result UTF-8")).expect("slice 2 result decodes");
            assert_eq!(result.canonical_payload, vec![9, 8, 7]);
            assert!(result.complete);
        }
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            panic!("slice 2 must dispatch to the registered service, not fail: {} {}", fault.code.0, fault.message);
        }
        JobStep::Running(_) => panic!("slice 2 must finish Done, the native inference call is atomic"),
    }
}

/// 📸️ Interrupts after slice 1 (decode only), checkpoints, cancels (simulating a trap), restores,
/// and confirms the resumed run reaches the SAME `Done` output as an uninterrupted run — the
/// mission's checkpoint/restore round-trip requirement, exercised against the real dispatch, not
/// a synthetic counter.
#[semio_framework_async_macros::async_test]
async fn infer_job_checkpoint_restore_matches_an_uninterrupted_run() {
    let _ = crate::app::register_artifact_inference_service(ArtifactInferenceService::new(TEST_METADATA, echo_infer));
    let input = request_bytes();

    start_job(201, JOB_KIND_INFER, &input).await;
    step_job(201, JobBudget::default()).await;
    let baseline = match step_job(201, JobBudget::default()).await {
        JobStep::Done(bytes) => bytes,
        _ => panic!("uninterrupted run must finish Done within 2 slices"),
    };

    start_job(202, JOB_KIND_INFER, &input).await;
    step_job(202, JobBudget::default()).await;
    let entries = checkpoint_jobs().await;
    let entry = entries.iter().find(|entry| entry.job == 202).expect("job 202 must appear in checkpoint_jobs()");
    assert_eq!(entry.checkpoint.as_deref(), Some(PHASE_DECODED), "slice 1 must have checkpointed PHASE_DECODED");
    let checkpoint = entry.checkpoint.clone();
    cancel_job(202).await;

    restore_job(202, JOB_KIND_INFER, &input, checkpoint).await;
    let restored_final = match step_job(202, JobBudget::default()).await {
        JobStep::Done(bytes) => bytes,
        JobStep::Running(_) => panic!("a restore from PHASE_DECODED must finish Done on its FIRST step_job call (only the execute tick remains)"),
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            panic!("restored run must not fail: {} {}", fault.code.0, fault.message);
        }
    };
    assert_eq!(restored_final, baseline, "checkpoint/restore must produce the identical final output");
}

#[semio_framework_async_macros::async_test]
async fn infer_job_reports_a_named_decode_fault_on_garbage_input() {
    start_job(203, JOB_KIND_INFER, b"not json").await;
    match step_job(203, JobBudget::default()).await {
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            assert_eq!(fault.code.0, "job.infer.decode");
        }
        _ => panic!("garbage infer input must fail on slice 1, before ever reaching the registry"),
    }
}

#[test]
fn interactive_bridge_coalesces_preview_but_backpressures_lossless_items() {
    let operation = Operation::new(OperationId(7), RevisionId(11), Generation(3), 0);
    let mut bridge = InferenceBridge::new(operation);
    bridge.publish_preview(vec![1]).expect("first preview");
    bridge.publish_preview(vec![2, 3]).expect("latest preview");
    assert_eq!(bridge.take_preview().expect("coalesced preview").payload, vec![2, 3]);

    bridge
        .publish_lossless(LosslessInferenceItem::Checkpoint(semio_framework_job::Checkpoint { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), applied_progress: 1 }))
        .expect("first checkpoint");
    bridge
        .publish_lossless(LosslessInferenceItem::Checkpoint(semio_framework_job::Checkpoint { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), applied_progress: 2 }))
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
