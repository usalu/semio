//! 💡️ `semio.infer` cold-job bridge. Exact ActionBus routes such as `s.wfc.wfc3d.solve`
//! decode through their factory-owned schema and retain one persistent `InteractiveJob` session.
//! Every guest continuation admits exactly one bounded step to the shared WorkerPool; previews
//! coalesce, checkpoints and commits remain lossless under explicit item/byte bounds, and
//! diagnostics use a bounded ring. Inferences without an ActionBus route retain the synchronous
//! two-phase registry path.

use super::{BoundedJob, JobBudget, JobStep, TwoPhaseBoundedJob, WORK_UNITS_EXECUTE, WORK_UNITS_PUMP, WORK_UNITS_RETIRE};
use semio_framework_job::{Generation, Operation, OperationId, RevisionId, StepOutcome};
use semio_framework_value_derive::ToValue;
use std::collections::VecDeque;

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

/// 🎟️ Ceiling on the bounded retirement actions one `cancel`/`fail` may drive in a single call.
/// A retirement that needs more than this leaves the owner deep, which `step_job`/`cancel_job`
/// report as `job.bounded-false-terminal` rather than silently dropping a live worker session.
const RETIRE_STEP_CEILING: usize = 4_096;

type InferenceSession = semio_framework_job::MountedWorkerJobSession<semio_framework::action_bus::ErasedToolJob>;
type InferenceRejected = semio_framework_job::WorkerJobSessionAdmissionRejected<semio_framework::action_bus::ErasedToolJob>;

// 🚫️async: E4 fn-pointer slot — registered into `BoundedJobFactory` (see
// `⚛️reactor/💼️jobs/🦀️.rs`'s `builtin_registry`); the admission body routes and constructs only.
pub(super) fn job_infer(job: u64, input: &[u8], restored: Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    let request = match decode_request(input) {
        Ok(request) => request,
        Err(error) => return Err(dsl::encode_fault_bytes(&error)),
    };
    let key = semio_framework::ToolFactoryKey::new(super::JOB_KIND_INFER, request.inference_schema.clone());
    if semio_framework::ActionBus::production().contains(&key) {
        return InteractiveInferenceJob::admit(job, request, restored).map(|owner| Box::new(owner) as Box<dyn BoundedJob>);
    }
    Ok(Box::new(TwoPhaseBoundedJob::admit("job.infer", input, restored, decode_phase, execute_phase)))
}

// 🚫️async: E4 phase slot — `BuiltinPhaseFn` is synchronous by contract; `decode` has no suspension
// point, so `settle_in_step` resolves it inside this state action.
fn decode_phase(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    super::settle_in_step("job.infer", decode(input))
}

// 🚫️async: E4 phase slot — see `decode_phase`; `wire_artifact_infer` is the unchunked native call
// this state action declares `WORK_UNITS_EXECUTE` for, and its own error type is translated here.
fn execute_phase(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    super::settle_in_step("job.infer", async move { crate::app::wire_artifact_infer(input).await.map_err(|error| super::fault(error.code, error.message.clone())) })
}

// 🚫️async: E1 pure parse consumed by the sync factory above and by `decode`'s own body.
fn decode_request(input: &[u8]) -> Result<crate::app::WireArtifactInferenceRequest, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
    dsl::os_pack::json::from_json_str(input_text).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))
}

/// 🚦️ The explicit states of one ActionBus-routed inference. Each is exactly one `step-job`
/// opportunity: `Dispatch` admits the worker session, `Pump` advances it by one bounded worker
/// step, `OutcomeClose` retires one page of the checked-out outcome, `SessionClose` retires one
/// page of the session after a terminal outcome, `RejectedClose` retires an admission rejection,
/// and `Complete` has no action left. The state walk is the literal translation of the former
/// `run_interactive_inference` future: every `ctx.tick().await` in that body is one state boundary
/// here, so the sliceability the async shape had is preserved without an executor to park on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum InteractivePhase {
    Dispatch,
    Pump,
    OutcomeClose,
    SessionClose,
    RejectedClose,
    Complete,
}

struct InteractiveInferenceJob {
    request: crate::app::WireArtifactInferenceRequest,
    restored: Option<Vec<u8>>,
    operation: Operation,
    bridge: InferenceBridge,
    /// 🛑️ Drop guard: releases this inference's cancellation slot when the machine is dropped,
    /// exactly as the former future's own `let _cancellation = …` binding did for its whole run.
    _cancellation: crate::app::ArtifactInferenceCancellationGuard,
    cancel: semio_framework_job::CancelToken,
    session: Option<InferenceSession>,
    rejected: Option<InferenceRejected>,
    outcome: Option<StepOutcome>,
    result: Option<Result<Vec<u8>, semio_framework::Fault>>,
    terminal: bool,
    checkpoint: Option<Vec<u8>>,
    phase: InteractivePhase,
    cancelled: bool,
}

impl InteractiveInferenceJob {
    /// 🎟️ Admission: validates the request's declared resources and claims its cancellation slot
    /// before any worker capacity is touched, so a refused inference never reaches the pool.
    fn admit(job: u64, request: crate::app::WireArtifactInferenceRequest, restored: Option<&[u8]>) -> Result<Self, Vec<u8>> {
        crate::app::validate_wire_request_resources(&request).map_err(|error| dsl::encode_fault_bytes(&super::fault(error.code, error.message)))?;
        let cancellation = crate::app::begin_artifact_inference(&request.cancellation_id).map_err(|error| dsl::encode_fault_bytes(&super::fault(error.code, error.message)))?;
        let operation = Operation::new(OperationId(job), RevisionId(request.revision), Generation(request.generation), 0);
        let bridge = InferenceBridge::new(operation);
        Ok(Self {
            request,
            restored: restored.map(<[u8]>::to_vec),
            operation,
            bridge,
            _cancellation: cancellation,
            cancel: semio_framework_job::root_cancel_token(),
            session: None,
            rejected: None,
            outcome: None,
            result: None,
            terminal: false,
            checkpoint: None,
            phase: InteractivePhase::Dispatch,
            cancelled: false,
        })
    }

    // 🚫️async: E1 pure price table consumed by `step`'s sync budget gate.
    fn price(&self) -> u64 {
        match self.phase {
            InteractivePhase::Dispatch => WORK_UNITS_EXECUTE,
            InteractivePhase::Pump => WORK_UNITS_PUMP,
            InteractivePhase::OutcomeClose | InteractivePhase::SessionClose | InteractivePhase::RejectedClose => WORK_UNITS_RETIRE,
            InteractivePhase::Complete => 0,
        }
    }

    /// 🧹️ Drives every deep owner this machine still holds through its own bounded close protocol
    /// so the wrapper left behind is shallow — the property `step_job`/`cancel_job` assert on.
    // 🚫️async: E1 retirement driver consumed by `cancel` (an externally-declared sync trait method)
    // and by `fail`; every close protocol it drives is itself synchronous.
    fn retire(&mut self) {
        if let Some(outcome) = self.outcome.as_mut() {
            for _ in 0..RETIRE_STEP_CEILING {
                if matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) && outcome.terminal_is_empty() {
                    break;
                }
            }
            if outcome.terminal_is_empty() {
                self.outcome = None;
            }
        }
        if let Some(session) = self.session.as_mut() {
            session.begin_close();
            for _ in 0..RETIRE_STEP_CEILING {
                if matches!(session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::WorkerJobCloseStep::Complete) && session.terminal_is_empty() {
                    break;
                }
            }
            if session.terminal_is_empty() {
                self.session = None;
            }
        }
        if let Some(rejected) = self.rejected.as_mut() {
            for _ in 0..RETIRE_STEP_CEILING {
                if matches!(rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && rejected.terminal_is_empty() {
                    break;
                }
            }
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
        }
    }

    // 🚫️async: E1 pure terminal constructor consumed by every sync state action below.
    fn fail(&mut self, error: semio_framework::Fault) -> JobStep {
        self.retire();
        self.phase = InteractivePhase::Complete;
        JobStep::Failed(dsl::encode_fault_bytes(&error))
    }

    /// 🚀️ `Dispatch`: publishes the request's identity preview, resolves the ActionBus factory for
    /// `semio.infer/<schema>`, and mounts one worker session for it.
    // 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn dispatch(&mut self) -> JobStep {
        if let Err(error) = self.bridge.publish_preview(dsl::os_pack::json::to_json_string(&(self.request.artifact_kind.clone(), self.request.inference_schema.clone())).into_bytes()) {
            return self.fail(bridge_fault(&error));
        }
        let progress = self.bridge.take_preview().map(|item| encode_bridge_item(&item));
        let bus = semio_framework::ActionBus::production();
        let key = semio_framework::ToolFactoryKey::new(super::JOB_KIND_INFER, self.request.inference_schema.clone());
        let Some(schema_id) = bus.payload_schema_id(&key) else {
            return self.fail(super::fault("job.infer.dispatch", "interactive inference factory disappeared before admission"));
        };
        let dispatch = match bus.dispatch_wire(super::JOB_KIND_INFER, self.request.inference_schema.clone(), schema_id, &self.request.canonical_payload, self.restored.clone(), self.operation) {
            Ok(dispatch) => dispatch,
            Err(error) => return self.fail(super::fault("job.infer.dispatch", error.to_string())),
        };
        let params = semio_framework_job::BatchJobParams {
            operation: self.operation.operation,
            generation: self.operation.generation,
            cancel: self.cancel.clone(),
            config: semio_framework_job::BatchDriveConfig {
                site: "semio.infer.action-bus",
                stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
                fuel_per_step: self.request.budgets.work_units.clamp(1, semio_framework_job::USER_VISIBLE_LANE_FUEL),
                step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
            },
            now_us: semio_framework_job::default_now_us,
        };
        match InferenceSession::try_new(dispatch.job, params) {
            Ok(session) => {
                self.session = Some(session);
                self.phase = InteractivePhase::Pump;
            }
            Err(rejected) => {
                self.rejected = Some(rejected);
                self.phase = InteractivePhase::RejectedClose;
            }
        }
        JobStep::Running(progress)
    }

    /// ⚙️ `Pump`: one bounded worker transition. An outcome that is not ready keeps the machine in
    /// this state with a fresh scheduled progress item, so the stall guard sees real movement.
    // 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn pump(&mut self) -> JobStep {
        match crate::app::inference_cancelled(&self.request.cancellation_id) {
            Ok(true) => self.cancel.cancel_now(),
            Ok(false) => {}
            Err(error) => return self.fail(super::fault(error.code, error.message)),
        }
        let scheduled = self.bridge.scheduled();
        let mut progress = encode_bridge_item(&scheduled);
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores));

        // 🧵️ ONE transition, and then the pool that owes it its work. `pump_one` only SUBMITS a step
        // to `semio_framework_async::process_worker_pool`; on wasm that pool has no threads and runs
        // a submitted step only inside `WorkerPool::pump`, which the reactor turn calls — and NO
        // reactor turn runs during a `step-job` crossing. So the session answered `Submitted`
        // forever, the machine stayed in `Pump`, and the host reissued `step-job` for as long as the
        // client waited: measured on `s.wfc.bitmap.solve` at 907 s and again at 423 s with a
        // 6 000 000-unit grant (`🗑️generated/gj1-solve-probe-{1,2}.txt`) — the grant is irrelevant to
        // a step that never runs. This is `📓️…project-wasm-pool-pump-starves-interactive-jobs`'s own
        // law ("any loop that waits on a cooperative-pool step on wasm must pump the pool itself")
        // applied to the one lane that still had no pump. Natively the pool has real workers and
        // `pump_process_worker_pool` is a no-op, so both hosts run the identical code.
        let poll = match self.session.as_mut() {
            Some(session) => session.pump_one(&pool, semio_framework_async::Lane::UserVisible),
            None => return self.fail(super::fault("job.infer.session-missing", "interactive inference lost its mounted worker session before pumping")),
        };
        let poll = match poll {
            Ok(poll) => poll,
            Err(_) => return self.fail(super::fault("job.infer.worker-pump", "interactive inference mounted worker transition was rejected")),
        };
        let poll = if matches!(poll, semio_framework_job::WorkerJobPoll::Submitted) {
            crate::reactor::turn::pump_process_worker_pool();
            let repoll = match self.session.as_mut() {
                Some(session) => session.pump_one(&pool, semio_framework_async::Lane::UserVisible),
                None => return self.fail(super::fault("job.infer.session-missing", "interactive inference lost its mounted worker session before pumping")),
            };
            match repoll {
                Ok(poll) => poll,
                Err(_) => return self.fail(super::fault("job.infer.worker-pump", "interactive inference mounted worker transition was rejected")),
            }
        } else {
            poll
        };
        if !matches!(poll, semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) {
            return JobStep::Running(Some(progress));
        }
        let Some(outcome) = self.session.as_mut().and_then(InferenceSession::take_checked_out_outcome) else {
            return self.fail(super::fault("job.infer.outcome-missing", "interactive inference mounted worker checkout lost its exact outcome"));
        };
        self.terminal = outcome.is_terminal();
        let result = match &outcome {
            StepOutcome::Yield => None,
            StepOutcome::PreviewReady(payload) => {
                let bytes = match copy_retained_payload(payload, PREVIEW_MAX_BYTES) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        self.outcome = Some(outcome);
                        return self.fail(error);
                    }
                };
                if let Err(error) = self.bridge.publish_preview(bytes) {
                    self.outcome = Some(outcome);
                    return self.fail(bridge_fault(&error));
                }
                if let Some(item) = self.bridge.take_preview() {
                    progress = encode_bridge_item(&item);
                }
                None
            }
            StepOutcome::CheckpointReady(checkpoint) => {
                match copy_retained_payload(&checkpoint.state, LOSSLESS_MAX_BYTES) {
                    Ok(bytes) => self.checkpoint = Some(bytes),
                    Err(error) => {
                        self.outcome = Some(outcome);
                        return self.fail(error);
                    }
                }
                None
            }
            StepOutcome::Complete(candidate) => match copy_retained_payload(&candidate.output, LOSSLESS_MAX_BYTES) {
                Ok(output) => Some(encode_result(self.request.clone(), output)),
                Err(error) => {
                    self.outcome = Some(outcome);
                    return self.fail(error);
                }
            },
            StepOutcome::Cancelled => Some(Err(super::fault("job.infer.cancelled", "interactive inference was cancelled"))),
            StepOutcome::Fault(fault) => {
                match copy_retained_payload(&fault.detail, DIAGNOSTIC_MAX_BYTES) {
                    Ok(bytes) => self.bridge.publish_diagnostic(bytes),
                    Err(error) => {
                        self.outcome = Some(outcome);
                        return self.fail(error);
                    }
                }
                if let Some(item) = self.bridge.latest_diagnostic() {
                    progress = encode_bridge_item(item);
                }
                let detail = self.bridge.latest_diagnostic().map_or_else(|| "interactive inference failed without retained diagnostic bytes".to_string(), |item| String::from_utf8_lossy(&item.payload).into_owned());
                Some(Err(super::fault("job.infer.interactive", detail)))
            }
        };
        self.result = result;
        self.outcome = Some(outcome);
        self.phase = InteractivePhase::OutcomeClose;
        JobStep::Running(Some(progress))
    }

    /// 🧾️ `OutcomeClose`: one page of the checked-out outcome's retained payload authority. On
    /// completion the machine either closes a terminal session or resumes the worker for the next
    /// pump, exactly as the former future's inner close loop did.
    // 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn close_outcome(&mut self) -> JobStep {
        let Some(outcome) = self.outcome.as_mut() else {
            return self.fail(super::fault("job.infer.outcome-missing", "interactive inference lost the outcome it was retiring"));
        };
        match outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            semio_framework_job::JobPayloadCloseStep::Pending { .. } => JobStep::Running(Some(self.retirement_progress())),
            semio_framework_job::JobPayloadCloseStep::Complete if outcome.terminal_is_empty() => {
                self.outcome = None;
                if self.terminal {
                    if let Some(session) = self.session.as_mut() {
                        session.begin_close();
                    }
                    self.phase = InteractivePhase::SessionClose;
                    return JobStep::Running(Some(self.retirement_progress()));
                }
                match self.session.as_mut().map(InferenceSession::resume) {
                    Some(Ok(())) => {
                        self.phase = InteractivePhase::Pump;
                        JobStep::Running(Some(self.retirement_progress()))
                    }
                    _ => self.fail(super::fault("job.infer.resume", "interactive inference outcome lost its exact resume authority")),
                }
            }
            semio_framework_job::JobPayloadCloseStep::Complete => self.fail(super::fault("job.infer.outcome-false-terminal", "interactive inference outcome did not reach terminal-empty payload authority")),
        }
    }

    /// 🧾️ `SessionClose`: one page of the mounted session's retirement after a terminal outcome.
    // 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn close_session(&mut self) -> JobStep {
        let Some(session) = self.session.as_mut() else {
            return self.fail(super::fault("job.infer.session-missing", "interactive inference lost the session it was retiring"));
        };
        match session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            semio_framework_job::WorkerJobCloseStep::Pending { .. } | semio_framework_job::WorkerJobCloseStep::Blocked => JobStep::Running(Some(self.retirement_progress())),
            semio_framework_job::WorkerJobCloseStep::Complete if session.terminal_is_empty() => {
                self.session = None;
                self.phase = InteractivePhase::Complete;
                match self.result.take() {
                    Some(Ok(bytes)) => JobStep::Done(bytes),
                    Some(Err(error)) => JobStep::Failed(dsl::encode_fault_bytes(&error)),
                    None => JobStep::Failed(dsl::encode_fault_bytes(&super::fault("job.infer.terminal-result", "terminal interactive inference produced no result"))),
                }
            }
            semio_framework_job::WorkerJobCloseStep::Complete => self.fail(super::fault("job.infer.session-false-terminal", "interactive inference session did not reach terminal-empty authority")),
        }
    }

    /// 🧾️ `RejectedClose`: one page of an admission rejection's retirement, after which the
    /// capacity refusal is reported — the former future's own rejection loop, state by state.
    // 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn close_rejected(&mut self) -> JobStep {
        let Some(rejected) = self.rejected.as_mut() else {
            return self.fail(super::fault("job.infer.admission", "interactive inference lost the rejection it was retiring"));
        };
        match rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            semio_framework_job::InteractiveJobCloseStep::Pending { .. } | semio_framework_job::InteractiveJobCloseStep::Blocked => JobStep::Running(Some(self.retirement_progress())),
            semio_framework_job::InteractiveJobCloseStep::Complete if rejected.terminal_is_empty() => {
                self.rejected = None;
                self.phase = InteractivePhase::Complete;
                JobStep::Failed(dsl::encode_fault_bytes(&super::fault("job.infer.admission", "interactive inference worker session capacity is exhausted")))
            }
            semio_framework_job::InteractiveJobCloseStep::Complete => self.fail(super::fault("job.infer.admission-false-terminal", "interactive inference admission rejection did not reach terminal-empty authority")),
        }
    }

    /// 📈️ A fresh scheduled bridge item for a retirement state, so every `Running` this machine
    /// reports carries monotonic progress bytes and the stall guard never mistakes a bounded close
    /// walk for a wedged job.
    // 🚫️async: E1 pure bridge read consumed by the sync state actions above.
    fn retirement_progress(&mut self) -> Vec<u8> {
        let item = self.bridge.scheduled();
        encode_bridge_item(&item)
    }
}

impl BoundedJob for InteractiveInferenceJob {
    fn step(&mut self, budget: JobBudget) -> JobStep {
        if self.cancelled {
            self.phase = InteractivePhase::Complete;
            return JobStep::Failed(dsl::encode_fault_bytes(&super::fault("job.infer.cancelled", "interactive inference was cancelled before its next state action")));
        }
        let price = self.price();
        if budget.fuel < price {
            return self.fail(super::fault("job.infer.budget-exhausted", format!("interactive inference needs {price} work units for its next state action and was granted {}", budget.fuel)));
        }
        match self.phase {
            InteractivePhase::Dispatch => self.dispatch(),
            InteractivePhase::Pump => self.pump(),
            InteractivePhase::OutcomeClose => self.close_outcome(),
            InteractivePhase::SessionClose => self.close_session(),
            InteractivePhase::RejectedClose => self.close_rejected(),
            InteractivePhase::Complete => JobStep::Failed(dsl::encode_fault_bytes(&super::fault("job.infer.terminal", "interactive inference has no state action left to advance"))),
        }
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        self.cancel.cancel_now();
        self.retire();
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        self.checkpoint.clone()
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.outcome.is_none()
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
/// `(artifact_kind, inference_schema)` identity as the `Decode` state's progress bytes — a REAL
/// decode (not a placeholder), since a malformed request should fail on the first state action,
/// before ever touching the inference-service registry.
async fn decode(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let request = decode_request(input)?;
    Ok(dsl::os_pack::json::to_json_string(&(request.artifact_kind, request.inference_schema)).into_bytes())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
