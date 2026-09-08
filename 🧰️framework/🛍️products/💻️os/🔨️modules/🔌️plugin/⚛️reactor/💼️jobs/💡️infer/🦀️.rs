//! 💡️ `semio.infer` cold-job bridge. Exact ActionBus routes such as `s.assembly.solve`
//! decode through their factory-owned schema and retain one persistent `InteractiveJob` session.
//! Every guest continuation admits exactly one bounded step to the shared WorkerPool; previews
//! coalesce, checkpoints and commits remain lossless under explicit item/byte bounds, and
//! diagnostics use a bounded ring. Inferences without an ActionBus route retain the synchronous
//! two-phase registry path.

use super::{run_two_phase, JobCtx};
use semio_framework_job::{Generation, Operation, OperationId, RevisionId, StepOutcome};
use semio_framework_value_derive::ToValue;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

const PREVIEW_MAX_BYTES: usize = 1 << 20;
#[cfg(test)]
const LOSSLESS_MAX_ITEMS: usize = 2;
const LOSSLESS_MAX_BYTES: usize = 2 << 20;
const DIAGNOSTIC_MAX_ITEMS: usize = 32;
const DIAGNOSTIC_MAX_BYTES: usize = 64 << 10;

//#region 🌉️Channels
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
enum InferenceBridgeKind {
    Scheduled,
    Preview,
    Diagnostic,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceBridgeItem {
    kind: InferenceBridgeKind,
    operation: u64,
    generation: u64,
    sequence: u64,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
enum LosslessInferenceItem {
    Checkpoint(semio_framework_job::Checkpoint),
    #[cfg(test)]
    TestBytes(usize),
}

#[cfg(test)]
impl LosslessInferenceItem {
    fn byte_len(&self) -> usize {
        match self {
            Self::Checkpoint(checkpoint) => checkpoint.state.len(),
            #[cfg(test)]
            Self::TestBytes(bytes) => *bytes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum InferenceBridgeError {
    Oversized {
        channel: &'static str,
        bytes: usize,
        max_bytes: usize,
    },
    #[cfg(test)]
    Saturated {
        channel: &'static str,
        items: usize,
        bytes: usize,
    },
}

impl std::fmt::Display for InferenceBridgeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Oversized { channel, bytes, max_bytes } => write!(formatter, "{channel} item has {bytes} bytes, above {max_bytes}"),
            #[cfg(test)]
            Self::Saturated { channel, items, bytes } => write!(formatter, "{channel} is saturated at {items} items/{bytes} bytes"),
        }
    }
}

struct InferenceBridge {
    operation: Operation,
    sequence: u64,
    preview: Option<InferenceBridgeItem>,
    #[cfg(test)]
    lossless: [Option<LosslessInferenceItem>; LOSSLESS_MAX_ITEMS],
    #[cfg(test)]
    lossless_len: usize,
    #[cfg(test)]
    lossless_bytes: usize,
    diagnostics: VecDeque<InferenceBridgeItem>,
    diagnostic_bytes: usize,
}

impl InferenceBridge {
    fn new(operation: Operation) -> Self {
        Self {
            operation,
            sequence: 0,
            preview: None,
            #[cfg(test)]
            lossless: std::array::from_fn(|_| None),
            #[cfg(test)]
            lossless_len: 0,
            #[cfg(test)]
            lossless_bytes: 0,
            diagnostics: VecDeque::new(),
            diagnostic_bytes: 0,
        }
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

    #[cfg(test)]
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

    #[cfg(test)]
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
        let input_text = std::str::from_utf8(&input).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
        let request: crate::app::WireArtifactInferenceRequest = dsl::os_pack::json::from_json_str(input_text).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
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
    bridge.publish_preview(dsl::os_pack::json::to_json_string(&(request.artifact_kind.clone(), request.inference_schema.clone())).into_bytes()).map_err(|error| bridge_fault(&error))?;
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
            fuel_per_step: request.budgets.work_units.clamp(1, semio_framework_job::USER_VISIBLE_LANE_FUEL),
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
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
                bridge.publish_preview(bytes).map_err(|error| bridge_fault(&error))?;
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
                let detail = bridge.latest_diagnostic().map_or_else(|| "interactive inference failed without retained diagnostic bytes".to_string(), |item| String::from_utf8_lossy(&item.payload).into_owned());
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

fn bridge_fault(error: &InferenceBridgeError) -> semio_framework::Fault {
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
    Ok(dsl::os_pack::json::to_json_string(&result).into_bytes())
}

/// 🔎️ Validates `input` decodes as a `WireArtifactInferenceRequest` and reports its
/// `(artifact_kind, inference_schema)` identity as the first slice's progress bytes — a REAL
/// decode (not a placeholder), since a malformed request should fail on slice 1, before ever
/// touching the inference-service registry on slice 2.
async fn decode(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
    let request: crate::app::WireArtifactInferenceRequest = dsl::os_pack::json::from_json_str(input_text).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
    Ok(dsl::os_pack::json::to_json_string(&(request.artifact_kind, request.inference_schema)).into_bytes())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
