//! 💡️ `semio.infer` cold-job bridge. Exact ActionBus routes such as `s.assembly.solve`
//! decode through their factory-owned schema and retain one persistent `InteractiveJob` session.
//! Every guest continuation admits exactly one bounded step to the shared WorkerPool; previews
//! coalesce, checkpoints and commits remain lossless under explicit item/byte bounds, and
//! diagnostics use a bounded ring. Inferences without an ActionBus route retain the synchronous
//! two-phase registry path.

use super::{run_two_phase, JobCtx};
use semio_framework_job::{CommitCandidate, Generation, Operation, OperationId, RevisionId, StepOutcome};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

const PREVIEW_MAX_BYTES: usize = 1 << 20;
const LOSSLESS_MAX_ITEMS: usize = 2;
const LOSSLESS_MAX_BYTES: usize = 2 << 20;
const DIAGNOSTIC_MAX_ITEMS: usize = 32;
const DIAGNOSTIC_MAX_BYTES: usize = 64 << 10;

//#region 🌉️Channels
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum InferenceBridgeKind {
    Scheduled,
    Preview,
    Diagnostic,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct InferenceBridgeItem {
    kind: InferenceBridgeKind,
    operation: u64,
    generation: u64,
    sequence: u64,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
enum LosslessInferenceItem {
    Checkpoint(semio_framework_job::Checkpoint),
    Commit(CommitCandidate),
    #[cfg(test)]
    TestBytes(usize),
}

impl LosslessInferenceItem {
    fn byte_len(&self) -> usize {
        match self {
            Self::Checkpoint(checkpoint) => checkpoint.state.len(),
            Self::Commit(candidate) => candidate.state.len().saturating_add(candidate.output.len()),
            #[cfg(test)]
            Self::TestBytes(bytes) => *bytes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum InferenceBridgeError {
    Oversized { channel: &'static str, bytes: usize, max_bytes: usize },
    Saturated { channel: &'static str, items: usize, bytes: usize },
}

impl std::fmt::Display for InferenceBridgeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Oversized { channel, bytes, max_bytes } => write!(formatter, "{channel} item has {bytes} bytes, above {max_bytes}"),
            Self::Saturated { channel, items, bytes } => write!(formatter, "{channel} is saturated at {items} items/{bytes} bytes"),
        }
    }
}

struct InferenceBridge {
    operation: Operation,
    sequence: u64,
    preview: Option<InferenceBridgeItem>,
    lossless: [Option<LosslessInferenceItem>; LOSSLESS_MAX_ITEMS],
    lossless_len: usize,
    lossless_bytes: usize,
    diagnostics: VecDeque<InferenceBridgeItem>,
    diagnostic_bytes: usize,
}

impl InferenceBridge {
    fn new(operation: Operation) -> Self {
        Self { operation, sequence: 0, preview: None, lossless: std::array::from_fn(|_| None), lossless_len: 0, lossless_bytes: 0, diagnostics: VecDeque::new(), diagnostic_bytes: 0 }
    }

    fn item(&mut self, kind: InferenceBridgeKind, payload: Vec<u8>) -> InferenceBridgeItem {
        let item = InferenceBridgeItem { kind, operation: self.operation.operation.0, generation: self.operation.generation.0, sequence: self.sequence, payload };
        self.sequence = self.sequence.saturating_add(1);
        item
    }

    fn publish_preview(&mut self, payload: Vec<u8>) -> Result<(), InferenceBridgeError> {
        if payload.len() > PREVIEW_MAX_BYTES {
            return Err(InferenceBridgeError::Oversized { channel: "preview", bytes: payload.len(), max_bytes: PREVIEW_MAX_BYTES });
        }
        let item = self.item(InferenceBridgeKind::Preview, payload);
        self.preview = Some(item);
        Ok(())
    }

    fn take_preview(&mut self) -> Option<InferenceBridgeItem> {
        self.preview.take()
    }

    fn publish_lossless(&mut self, item: LosslessInferenceItem) -> Result<(), InferenceBridgeError> {
        let bytes = item.byte_len();
        if bytes > LOSSLESS_MAX_BYTES {
            return Err(InferenceBridgeError::Oversized { channel: "checkpoint-commit", bytes, max_bytes: LOSSLESS_MAX_BYTES });
        }
        let total_bytes = self.lossless_bytes.checked_add(bytes).ok_or(InferenceBridgeError::Saturated { channel: "checkpoint-commit", items: self.lossless_len, bytes: self.lossless_bytes })?;
        if self.lossless_len >= LOSSLESS_MAX_ITEMS || total_bytes > LOSSLESS_MAX_BYTES {
            return Err(InferenceBridgeError::Saturated { channel: "checkpoint-commit", items: self.lossless_len, bytes: self.lossless_bytes });
        }
        self.lossless[self.lossless_len] = Some(item);
        self.lossless_len += 1;
        self.lossless_bytes = total_bytes;
        Ok(())
    }

    fn take_lossless(&mut self) -> Option<LosslessInferenceItem> {
        let item = self.lossless[0].take()?;
        for index in 1..self.lossless_len {
            self.lossless[index - 1] = self.lossless[index].take();
        }
        self.lossless_len -= 1;
        self.lossless_bytes = self.lossless_bytes.saturating_sub(item.byte_len());
        Some(item)
    }

    fn publish_diagnostic(&mut self, payload: Vec<u8>) {
        if payload.len() > DIAGNOSTIC_MAX_BYTES {
            return;
        }
        while self.diagnostics.len() >= DIAGNOSTIC_MAX_ITEMS || self.diagnostic_bytes.saturating_add(payload.len()) > DIAGNOSTIC_MAX_BYTES {
            let Some(removed) = self.diagnostics.pop_front() else {
                break;
            };
            self.diagnostic_bytes = self.diagnostic_bytes.saturating_sub(removed.payload.len());
        }
        self.diagnostic_bytes = self.diagnostic_bytes.saturating_add(payload.len());
        let item = self.item(InferenceBridgeKind::Diagnostic, payload);
        self.diagnostics.push_back(item);
    }

    fn scheduled(&mut self) -> InferenceBridgeItem {
        self.item(InferenceBridgeKind::Scheduled, Vec::new())
    }

    fn latest_diagnostic(&self) -> Option<&InferenceBridgeItem> {
        self.diagnostics.back()
    }
}

fn encode_bridge_item(item: &InferenceBridgeItem) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(1 + 8 * 3 + 4 + item.payload.len());
    bytes.push(match item.kind {
        InferenceBridgeKind::Scheduled => 0,
        InferenceBridgeKind::Preview => 1,
        InferenceBridgeKind::Diagnostic => 2,
    });
    bytes.extend_from_slice(&item.operation.to_le_bytes());
    bytes.extend_from_slice(&item.generation.to_le_bytes());
    bytes.extend_from_slice(&item.sequence.to_le_bytes());
    bytes.extend_from_slice(&(item.payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&item.payload);
    bytes
}
//#endregion 🌉️Channels

// 🚫️async: E4 fn-pointer slot — see `job_mutation_plan`'s own comment in the sibling `🧬️mutation-plan`
// module for the full explanation; same `JobFn` registry shape.
pub(super) fn job_infer(ctx: JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move {
        let request: crate::app::WireArtifactInferenceRequest = serde_json::from_slice(&input).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
        let key = semio_framework::ToolFactoryKey::new(super::JOB_KIND_INFER, request.inference_schema.clone());
        if semio_framework::ActionBus::production().contains(&key) {
            return run_interactive_inference(ctx, request, restored).await;
        }
        let decode_input = input.clone();
        let execute_input = input;
        run_two_phase(ctx, restored, move || async move { decode(&decode_input).await }, move || async move { crate::app::wire_artifact_infer(&execute_input).await.map_err(|error| super::fault(error.code, error.message.clone())) }).await
    })
}

async fn run_interactive_inference(ctx: JobCtx, request: crate::app::WireArtifactInferenceRequest, restored: Option<Vec<u8>>) -> Result<Vec<u8>, semio_framework::Fault> {
    crate::app::validate_wire_request_resources(&request).map_err(|error| super::fault(error.code, error.message))?;
    let _cancellation = crate::app::begin_artifact_inference(&request.cancellation_id).map_err(|error| super::fault(error.code, error.message))?;
    let operation = Operation::new(OperationId(ctx.id().await), RevisionId(request.revision), Generation(request.generation), 0);
    let mut bridge = InferenceBridge::new(operation);
    ctx.tick().await;
    bridge.publish_preview(serde_json::to_vec(&(request.artifact_kind.clone(), request.inference_schema.clone())).map_err(|error| super::fault("job.infer.progress", error.to_string()))?).map_err(bridge_fault)?;
    if let Some(item) = bridge.take_preview() {
        ctx.progress(encode_bridge_item(&item)).await;
    }

    let bus = semio_framework::ActionBus::production();
    let key = semio_framework::ToolFactoryKey::new(super::JOB_KIND_INFER, request.inference_schema.clone());
    let schema_id = bus.payload_schema_id(&key).ok_or_else(|| super::fault("job.infer.dispatch", "interactive inference factory disappeared before admission"))?;
    let dispatch = bus.dispatch_wire(super::JOB_KIND_INFER, request.inference_schema.clone(), schema_id, &request.canonical_payload, restored, operation).map_err(|error| super::fault("job.infer.dispatch", error.to_string()))?;
    let cancel = semio_framework_job::root_cancel_token();
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: cancel.clone(),
        config: semio_framework_job::BatchDriveConfig {
            site: "semio.infer.action-bus",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: request.budgets.work_units.min(semio_framework_job::USER_VISIBLE_LANE_FUEL).max(1),
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    let cores = std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1);
    let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores));
    let mut session = match semio_framework_job::MountedWorkerJobSession::try_new(dispatch.job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            loop {
                ctx.tick().await;
                match rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                    semio_framework_job::InteractiveJobCloseStep::Pending { .. } | semio_framework_job::InteractiveJobCloseStep::Blocked => {}
                    semio_framework_job::InteractiveJobCloseStep::Complete if rejected.terminal_is_empty() => break,
                    semio_framework_job::InteractiveJobCloseStep::Complete => return Err(super::fault("job.infer.admission-false-terminal", "interactive inference admission rejection did not reach terminal-empty authority")),
                }
            }
            return Err(super::fault("job.infer.admission", "interactive inference worker session capacity is exhausted"));
        }
    };

    loop {
        if crate::app::inference_cancelled(&request.cancellation_id).map_err(|error| super::fault(error.code, error.message))? {
            cancel.cancel_now();
        }
        ctx.tick().await;
        let scheduled = bridge.scheduled();
        ctx.progress(encode_bridge_item(&scheduled)).await;
        let poll = session.pump_one(&pool, semio_framework_async::Lane::UserVisible).map_err(|_| super::fault("job.infer.worker-pump", "interactive inference mounted worker transition was rejected"))?;
        if !matches!(poll, semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) {
            continue;
        }
        let mut outcome = session.take_checked_out_outcome().ok_or_else(|| super::fault("job.infer.outcome-missing", "interactive inference mounted worker checkout lost its exact outcome"))?;
        let terminal = outcome.is_terminal();
        let result = match &outcome {
            StepOutcome::Yield => None,
            StepOutcome::PreviewReady(payload) => {
                let bytes = copy_retained_payload(payload, PREVIEW_MAX_BYTES)?;
                bridge.publish_preview(bytes).map_err(bridge_fault)?;
                if let Some(item) = bridge.take_preview() {
                    ctx.progress(encode_bridge_item(&item)).await;
                }
                None
            }
            StepOutcome::CheckpointReady(checkpoint) => {
                ctx.checkpoint(copy_retained_payload(&checkpoint.state, LOSSLESS_MAX_BYTES)?).await;
                None
            }
            StepOutcome::Complete(candidate) => {
                let output = copy_retained_payload(&candidate.output, LOSSLESS_MAX_BYTES)?;
                Some(encode_result(request.clone(), output))
            }
            StepOutcome::Cancelled => Some(Err(super::fault("job.infer.cancelled", "interactive inference was cancelled"))),
            StepOutcome::Fault(fault) => {
                bridge.publish_diagnostic(copy_retained_payload(&fault.detail, DIAGNOSTIC_MAX_BYTES)?);
                if let Some(item) = bridge.latest_diagnostic() {
                    ctx.progress(encode_bridge_item(item)).await;
                }
                let detail = bridge.latest_diagnostic().map(|item| String::from_utf8_lossy(&item.payload).into_owned()).unwrap_or_else(|| "interactive inference failed without retained diagnostic bytes".to_string());
                Some(Err(super::fault("job.infer.interactive", detail)))
            }
        };
        loop {
            ctx.tick().await;
            match outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                semio_framework_job::JobPayloadCloseStep::Pending { .. } => {}
                semio_framework_job::JobPayloadCloseStep::Complete if outcome.terminal_is_empty() => break,
                semio_framework_job::JobPayloadCloseStep::Complete => return Err(super::fault("job.infer.outcome-false-terminal", "interactive inference outcome did not reach terminal-empty payload authority")),
            }
        }
        if terminal {
            session.begin_close();
            loop {
                ctx.tick().await;
                match session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                    semio_framework_job::WorkerJobCloseStep::Pending { .. } | semio_framework_job::WorkerJobCloseStep::Blocked => {}
                    semio_framework_job::WorkerJobCloseStep::Complete if session.terminal_is_empty() => break,
                    semio_framework_job::WorkerJobCloseStep::Complete => return Err(super::fault("job.infer.session-false-terminal", "interactive inference session did not reach terminal-empty authority")),
                }
            }
            return result.unwrap_or_else(|| Err(super::fault("job.infer.terminal-result", "terminal interactive inference produced no result")));
        }
        session.resume().map_err(|_| super::fault("job.infer.resume", "interactive inference outcome lost its exact resume authority"))?;
    }
}

fn copy_retained_payload(payload: &semio_framework_job::RetainedJobPayload, maximum_bytes: usize) -> Result<Vec<u8>, semio_framework::Fault> {
    if payload.len() > maximum_bytes {
        return Err(super::fault("job.infer.payload-limit", format!("interactive inference retained payload has {} bytes, above {maximum_bytes}", payload.len())));
    }
    let mut bytes = Vec::with_capacity(payload.len());
    let mut reader = payload.reader();
    while let Some(page) = reader.read_page(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
        bytes.extend_from_slice(page);
    }
    if !reader.terminal_is_empty() {
        return Err(super::fault("job.infer.payload-page", "interactive inference retained payload page exceeded the fixed codec grant"));
    }
    Ok(bytes)
}

fn bridge_fault(error: InferenceBridgeError) -> semio_framework::Fault {
    super::fault("job.infer.bridge", error.to_string())
}

fn encode_result(request: crate::app::WireArtifactInferenceRequest, canonical_payload: Vec<u8>) -> Result<Vec<u8>, semio_framework::Fault> {
    let allocation = usize::try_from(request.budgets.allocation_bytes).map_err(|_| super::fault("job.infer.result", "allocation budget exceeds this runtime's address space"))?;
    if canonical_payload.len() > allocation {
        return Err(super::fault("job.infer.result", format!("interactive inference result has {} bytes, above allocation budget {allocation}", canonical_payload.len())));
    }
    let provenance = crate::app::WireArtifactInferenceProvenance {
        owner: request.owner.clone(),
        inference_schema: request.inference_schema.clone(),
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        source_dialect: request.source_dialect.clone(),
    };
    let result = crate::app::WireArtifactInferenceResult {
        wire_version: crate::app::ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: request.owner,
        artifact_kind: request.artifact_kind,
        artifact_schema: request.artifact_schema,
        artifact_schema_version: request.artifact_schema_version,
        document_schema: request.document_schema,
        document_schema_version: request.document_schema_version,
        inference_schema: request.inference_schema,
        inference_schema_version: request.inference_schema_version,
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        revision: request.revision,
        generation: request.generation,
        source_dialect: request.source_dialect,
        policy: request.policy,
        budgets: request.budgets,
        previous_state: request.previous_state,
        requested_cache_mode: request.requested_cache_mode.clone(),
        canonical_payload,
        dependencies: request.dependencies,
        diagnostics: Vec::new(),
        provenance,
        validity: "valid".into(),
        quality: "exact".into(),
        complete: true,
        actual_cache_mode: request.requested_cache_mode,
        cancellation_id: request.cancellation_id,
    };
    serde_json::to_vec(&result).map_err(|error| super::fault("job.infer.result-encode", error.to_string()))
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
        serde_json::to_vec(&request).expect("test request encodes")
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
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
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
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
        let mut bridge = InferenceBridge::new(operation);
        for index in 0..(DIAGNOSTIC_MAX_ITEMS + 9) {
            bridge.publish_diagnostic(vec![index as u8; 8]);
        }
        assert_eq!(bridge.diagnostics.len(), DIAGNOSTIC_MAX_ITEMS);
        assert!(bridge.diagnostic_bytes <= DIAGNOSTIC_MAX_BYTES);
        assert_eq!(bridge.diagnostics.front().expect("ring head").payload, vec![9; 8]);
    }
}
