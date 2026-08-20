//! 💡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `semio.infer` — the cold job kind
//! design-abi.md §2 names as the replacement for the deleted `contributor.artifact-infer` guest
//! export. `input`/the result are the SAME JSON `WireArtifactInferenceRequest`/`WireArtifactInferenceResult`
//! bytes the deleted export used (`🖥️host/🦀️component.rs`'s `PluginInstanceHandle::infer` already
//! passes them straight through, no tuple wrapping — see that method's own doc comment); dispatch
//! goes through `crate::app::wire_artifact_infer`, the SAME process-registered
//! `ArtifactInferenceServiceRegistry` lookup `job_io_run`/`job_io_sniff` (this crate's other two
//! builtin kinds) already use for their own process-global registries — not the per-instance
//! `crate::plugin_runtime::plugin_wire_artifact_infer`, which is scoped to a `PLUGIN` bundle
//! installed through a completely different mechanism (`install_plugin_bundle`) that a native
//! inference-service registration never requires.
//!
//! Sliced across `super::run_two_phase`'s two ticks: slice 1 decodes+validates `input` (reporting
//! `(artifact_kind, inference_schema)` as progress, matching what a caller most wants to see
//! mid-flight) and checkpoints; slice 2 runs the real `wire_artifact_infer` dispatch. The dormant
//! 10,930-LOC WFC solve this unblocks (`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`'s own blocker
//! comment) still needs its OWN internal `ctx.tick()` calls to be genuinely preemptible mid-solve —
//! that migration is explicitly out of this packet's scope (a W7 flagship owns it) — but the cold
//! job kind it will call into now exists and is real, not a stub.

use super::{run_two_phase, JobCtx};
use std::future::Future;
use std::pin::Pin;

// 🚫️async: E4 fn-pointer slot — see `job_mutation_plan`'s own comment in the sibling `🧬️mutation-plan`
// module for the full explanation; same `JobFn` registry shape.
pub(super) fn job_infer(ctx: JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move {
        let decode_input = input.clone();
        let execute_input = input;
        run_two_phase(
            ctx,
            restored,
            move || async move { decode(&decode_input).await },
            move || async move { crate::app::wire_artifact_infer(&execute_input).await.map_err(|error| super::fault(error.code, error.message.clone())) },
        )
        .await
    })
}

/// 🔎️ Validates `input` decodes as a `WireArtifactInferenceRequest` and reports its
/// `(artifact_kind, inference_schema)` identity as the first slice's progress bytes — a REAL
/// decode (not a placeholder), since a malformed request should fail on slice 1, before ever
/// touching the inference-service registry on slice 2.
async fn decode(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let request: crate::app::WireArtifactInferenceRequest = serde_json::from_slice(input).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
    serde_json::to_vec(&(request.artifact_kind, request.inference_schema)).map_err(|error| super::fault("job.infer.decode", error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::super::*;
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

    async fn echo_infer(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, crate::app::ArtifactInferenceExecutionError> {
        Ok(ArtifactInferenceExecution { canonical_payload: request.canonical_payload.to_vec(), diagnostics: Vec::new(), validity: "valid".into(), quality: "exact".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
    }

    async fn request_bytes() -> Vec<u8> {
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
            source_dialect: "s.jobtest.widget@1/*".into(),
            policy: Vec::new(),
            budgets: WireArtifactInferenceBudget { allocation_bytes: 1 << 20, work_units: 1000, recursion_depth: 4 },
            cancellation_id: "jobtest-cancel-1".into(),
            previous_state: None,
            requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
            canonical_payload: vec![9, 8, 7],
            dependencies: Vec::new(),
        };
        serde_json::to_vec(&request).expect("test request encodes")
    }

    /// 💡️ Registers a real native inference service (not mocked away) and drives `semio.infer`
    /// through two real `step_job` slices to `Done`, proving `job_infer` really reaches
    /// `crate::app::wire_artifact_infer` and not just `job.unknown-kind`.
    #[semio_framework_async_macros::async_test]
    async fn a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service() {
        let _ = crate::app::register_artifact_inference_service(ArtifactInferenceService::new(TEST_METADATA, echo_infer));
        start_job(200, JOB_KIND_INFER, &request_bytes());

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
                let result: crate::app::WireArtifactInferenceResult = serde_json::from_slice(&bytes).expect("slice 2 result decodes");
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

        start_job(201, JOB_KIND_INFER, &input);
        step_job(201, JobBudget::default());
        let baseline = match step_job(201, JobBudget::default()).await {
            JobStep::Done(bytes) => bytes,
            _ => panic!("uninterrupted run must finish Done within 2 slices"),
        };

        start_job(202, JOB_KIND_INFER, &input);
        step_job(202, JobBudget::default());
        let entries = checkpoint_jobs();
        let entry = entries.await.iter().find(|entry| entry.job == 202).expect("job 202 must appear in checkpoint_jobs()");
        assert_eq!(entry.checkpoint.as_deref(), Some(PHASE_DECODED), "slice 1 must have checkpointed PHASE_DECODED");
        let checkpoint = entry.checkpoint.clone();
        cancel_job(202);

        restore_job(202, JOB_KIND_INFER, &input, checkpoint);
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
        start_job(203, JOB_KIND_INFER, b"not json");
        match step_job(203, JobBudget::default()).await {
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                assert_eq!(fault.code.0, "job.infer.decode");
            }
            _ => panic!("garbage infer input must fail on slice 1, before ever reaching the registry"),
        }
    }
}
