//! 🧵️ Shared retained shell for app-owned typed command reducers.

use crate::app::{AppOperationContext, ArtifactApp, ArtifactOwnedToolJobContext, ArtifactToolCompletion, Emit, EphemeralEmit, HistoryView, InteractionHoverState};
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework::Fault;
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use std::sync::Arc;

//#region 🔖️Work
pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;
const ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES: usize = 48;
const ARTIFACT_COMMAND_CHECKPOINT_MAGIC: [u8; 4] = *b"ARC1";

struct ArtifactCommandCheckpoint<'a> {
    work_phase: bool,
    raw_page_cursor: u64,
    raw_bytes: u64,
    work_progress: u64,
    context_digest: u64,
    workspace_identity: u64,
    work: &'a [u8],
}

fn encode_artifact_command_checkpoint(checkpoint: ArtifactCommandCheckpoint<'_>, target: &mut [u8]) -> Result<usize, Fault> {
    let length = ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES.checked_add(checkpoint.work.len()).ok_or_else(|| Fault::from("retained-command-checkpoint-length-overflow"))?;
    if length > target.len() {
        return Err(Fault::from("retained-command-checkpoint-capacity"));
    }
    target[..length].fill(0);
    target[..4].copy_from_slice(&ARTIFACT_COMMAND_CHECKPOINT_MAGIC);
    target[4] = 3;
    target[5] = u8::from(checkpoint.work_phase);
    target[8..16].copy_from_slice(&checkpoint.raw_page_cursor.to_le_bytes());
    target[16..24].copy_from_slice(&checkpoint.raw_bytes.to_le_bytes());
    target[24..32].copy_from_slice(&checkpoint.work_progress.to_le_bytes());
    target[32..40].copy_from_slice(&checkpoint.context_digest.to_le_bytes());
    target[40..48].copy_from_slice(&checkpoint.workspace_identity.to_le_bytes());
    target[ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES..length].copy_from_slice(checkpoint.work);
    Ok(length)
}

fn decode_artifact_command_checkpoint(bytes: &[u8], maximum_raw_bytes: usize, input_pages: usize, input_bytes: usize, current_context_digest: u64, current_workspace_identity: u64) -> Result<ArtifactCommandCheckpoint<'_>, Fault> {
    if bytes.len() < ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES || bytes[..4] != ARTIFACT_COMMAND_CHECKPOINT_MAGIC || bytes[4] != 3 || bytes[6] != 0 || bytes[7] != 0 {
        return Err(Fault::from("retained-command-checkpoint-invalid"));
    }
    let work_phase = match bytes[5] {
        0 => false,
        1 => true,
        _ => return Err(Fault::from("retained-command-checkpoint-phase-invalid")),
    };
    let raw_page_cursor = u64::from_le_bytes(bytes[8..16].try_into().map_err(|_| Fault::from("retained-command-checkpoint-cursor-invalid"))?);
    let raw_bytes = u64::from_le_bytes(bytes[16..24].try_into().map_err(|_| Fault::from("retained-command-checkpoint-raw-invalid"))?);
    let work_progress = u64::from_le_bytes(bytes[24..32].try_into().map_err(|_| Fault::from("retained-command-checkpoint-progress-invalid"))?);
    let context_digest = u64::from_le_bytes(bytes[32..40].try_into().map_err(|_| Fault::from("retained-command-checkpoint-context-invalid"))?);
    let workspace_identity = u64::from_le_bytes(bytes[40..48].try_into().map_err(|_| Fault::from("retained-command-checkpoint-workspace-invalid"))?);
    if raw_page_cursor > input_pages as u64 || raw_bytes > input_bytes as u64 || raw_bytes > maximum_raw_bytes as u64 {
        return Err(Fault::from("retained-command-checkpoint-extent-invalid"));
    }
    if context_digest != current_context_digest {
        return Err(Fault::from("retained-command-checkpoint-context-mismatch"));
    }
    if workspace_identity != current_workspace_identity {
        return Err(Fault::from("retained-command-checkpoint-workspace-mismatch"));
    }
    Ok(ArtifactCommandCheckpoint { work_phase, raw_page_cursor, raw_bytes, work_progress, context_digest, workspace_identity, work: &bytes[ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES..] })
}

pub type ArtifactCommandReducer<A> = fn(
    &<A as ArtifactApp>::Command,
    &<A as ArtifactApp>::Snapshot,
    &<A as ArtifactApp>::Config,
    &HistoryView,
    &protocol::InteractionState,
    &InteractionHoverState,
    &AppOperationContext,
) -> Result<Emit<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, Fault>;

pub type ArtifactCommandExtent<A> = fn(&<A as ArtifactApp>::Command, &<A as ArtifactApp>::Snapshot, &protocol::InteractionState) -> Option<usize>;

pub enum ArtifactCommandWorkStep<A: ArtifactApp> {
    Replay { stage: &'static str, preview: &'static [u8] },
    Progress { stage: &'static str, preview: &'static [u8] },
    Complete(Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>),
    CompleteWithEphemeral { emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, ephemeral: EphemeralEmit<A> },
}

pub trait ArtifactCommandWork<A: ArtifactApp>: Send {
    fn tool_id(&self) -> &'static str;
    /// 🧰 Stable identity of the factory-provided mutable workspace retained by this job.
    fn workspace_identity(&self) -> u64 {
        0
    }
    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState, context: Option<&ArtifactOwnedToolJobContext<A>>) -> Option<usize>;
    fn step(
        &mut self,
        command: &A::Command,
        snapshot: &A::Snapshot,
        config: &A::Config,
        history: &HistoryView,
        interaction: &protocol::InteractionState,
        hover: &InteractionHoverState,
        context: Option<&ArtifactOwnedToolJobContext<A>>,
        operation: &AppOperationContext,
    ) -> Result<ArtifactCommandWorkStep<A>, Fault>;
    fn checkpoint(&self, _target: &mut [u8]) -> Result<usize, Fault> {
        Ok(0)
    }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.is_empty() {
            Ok(())
        } else {
            Err(Fault::from("retained-command-work-checkpoint-unsupported"))
        }
    }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        true
    }
}

pub struct BoundedArtifactCommandWork<A: ArtifactApp> {
    tool_id: &'static str,
    reducer: ArtifactCommandReducer<A>,
    extent: ArtifactCommandExtent<A>,
    consumed: bool,
}

impl<A: ArtifactApp> BoundedArtifactCommandWork<A> {
    pub fn new(tool_id: &'static str, reducer: ArtifactCommandReducer<A>, extent: ArtifactCommandExtent<A>) -> Self {
        Self { tool_id, reducer, extent, consumed: false }
    }
}

impl<A: ArtifactApp> ArtifactCommandWork<A> for BoundedArtifactCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<A>>) -> Option<usize> {
        (self.extent)(command, snapshot, interaction)
    }

    fn step(
        &mut self,
        command: &A::Command,
        snapshot: &A::Snapshot,
        config: &A::Config,
        history: &HistoryView,
        interaction: &protocol::InteractionState,
        hover: &InteractionHoverState,
        _context: Option<&ArtifactOwnedToolJobContext<A>>,
        operation: &AppOperationContext,
    ) -> Result<ArtifactCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("retained-command-bounded-work-repeated"));
        }
        self.consumed = true;
        (self.reducer)(command, snapshot, config, history, interaction, hover, operation).map(ArtifactCommandWorkStep::Complete)
    }
}
//#endregion 🔖️Work

//#region 🧳️Payload
pub struct ArtifactRetainedCommandPayload<A: ArtifactApp> {
    pub command: A::Command,
    pub snapshot: Arc<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub history: Arc<HistoryView>,
    pub interaction_state: Arc<protocol::InteractionState>,
    pub interaction_hover: Arc<InteractionHoverState>,
    pub context: Option<Arc<ArtifactOwnedToolJobContext<A>>>,
    pub operation: AppOperationContext,
    pub completion: ArtifactToolCompletion<A>,
    pub command_id: fn(&A::Command) -> &'static str,
    pub maximum_raw_bytes: usize,
    pub maximum_work_items: usize,
    pub raw: Vec<u8>,
    pub work: Box<dyn ArtifactCommandWork<A>>,
}

impl<A: ArtifactApp> ArtifactRetainedCommandPayload<A> {
    pub fn try_new(
        command: A::Command,
        snapshot: Arc<A::Snapshot>,
        config: Arc<A::Config>,
        history: Arc<HistoryView>,
        interaction_state: Arc<protocol::InteractionState>,
        interaction_hover: Arc<InteractionHoverState>,
        operation: AppOperationContext,
        completion: ArtifactToolCompletion<A>,
        command_id: fn(&A::Command) -> &'static str,
        maximum_raw_bytes: usize,
        maximum_work_items: usize,
        work: Box<dyn ArtifactCommandWork<A>>,
    ) -> Result<Self, Fault> {
        if maximum_raw_bytes == 0 || maximum_work_items == 0 {
            return Err(Fault::from("retained-command-capacity-is-zero"));
        }
        let mut raw = Vec::new();
        raw.try_reserve_exact(maximum_raw_bytes).map_err(|_| Fault::from("retained-command-raw-capacity-rejected"))?;
        Ok(Self { command, snapshot, config, history, interaction_state, interaction_hover, context: None, operation, completion, command_id, maximum_raw_bytes, maximum_work_items, raw, work })
    }

    pub fn try_new_with_context(
        command: A::Command,
        snapshot: Arc<A::Snapshot>,
        config: Arc<A::Config>,
        history: Arc<HistoryView>,
        interaction_state: Arc<protocol::InteractionState>,
        interaction_hover: Arc<InteractionHoverState>,
        context: Arc<ArtifactOwnedToolJobContext<A>>,
        operation: AppOperationContext,
        completion: ArtifactToolCompletion<A>,
        command_id: fn(&A::Command) -> &'static str,
        maximum_raw_bytes: usize,
        maximum_work_items: usize,
        work: Box<dyn ArtifactCommandWork<A>>,
    ) -> Result<Self, Fault> {
        let mut payload = Self::try_new(command, snapshot, config, history, interaction_state, interaction_hover, operation, completion, command_id, maximum_raw_bytes, maximum_work_items, work)?;
        payload.context = Some(context);
        Ok(payload)
    }
}
//#endregion 🧳️Payload

//#region 🧵️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArtifactRetainedCommandPhase {
    CheckpointPages,
    CheckpointRetire,
    WirePages,
    Decode,
    Preflight,
    Work,
    Publish,
    Complete,
    Fault,
}

pub struct ArtifactRetainedCommandJob<A: ArtifactApp> {
    command: Option<A::Command>,
    snapshot: Option<Arc<A::Snapshot>>,
    config: Option<Arc<A::Config>>,
    history: Option<Arc<HistoryView>>,
    interaction_state: Option<Arc<protocol::InteractionState>>,
    interaction_hover: Option<Arc<InteractionHoverState>>,
    context: Option<Arc<ArtifactOwnedToolJobContext<A>>>,
    operation: Option<AppOperationContext>,
    completion: Option<ArtifactToolCompletion<A>>,
    command_id: fn(&A::Command) -> &'static str,
    maximum_raw_bytes: usize,
    maximum_work_items: usize,
    work: Option<Box<dyn ArtifactCommandWork<A>>>,
    checkpoint_input: Option<RetainedToolWireInput>,
    checkpoint_bytes: [u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
    checkpoint_byte_len: usize,
    checkpoint_page_cursor: usize,
    raw_input: Option<RetainedToolWireInput>,
    raw: Vec<u8>,
    raw_page_cursor: usize,
    emit: Option<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>,
    ephemeral: Option<EphemeralEmit<A>>,
    phase: ArtifactRetainedCommandPhase,
    checkpoint_pending: bool,
    work_progress: u64,
    closing: bool,
}

impl<A: ArtifactApp> ArtifactRetainedCommandJob<A> {
    pub fn new(payload: ArtifactRetainedCommandPayload<A>) -> Self {
        Self::from_payload(payload, None, None)
    }

    pub fn from_wire(payload: ArtifactRetainedCommandPayload<A>, input: RetainedToolWireInput) -> Self {
        Self::from_payload(payload, Some(input), None)
    }

    pub fn from_wire_with_checkpoint(payload: ArtifactRetainedCommandPayload<A>, input: RetainedToolWireInput, checkpoint: RetainedToolWireInput) -> Self {
        Self::from_payload(payload, Some(input), Some(checkpoint))
    }

    fn from_payload(payload: ArtifactRetainedCommandPayload<A>, raw_input: Option<RetainedToolWireInput>, checkpoint_input: Option<RetainedToolWireInput>) -> Self {
        let phase = if checkpoint_input.is_some() {
            ArtifactRetainedCommandPhase::CheckpointPages
        } else if raw_input.is_some() {
            ArtifactRetainedCommandPhase::WirePages
        } else {
            ArtifactRetainedCommandPhase::Preflight
        };
        Self {
            command: Some(payload.command),
            snapshot: Some(payload.snapshot),
            config: Some(payload.config),
            history: Some(payload.history),
            interaction_state: Some(payload.interaction_state),
            interaction_hover: Some(payload.interaction_hover),
            context: payload.context,
            operation: Some(payload.operation),
            completion: Some(payload.completion),
            command_id: payload.command_id,
            maximum_raw_bytes: payload.maximum_raw_bytes,
            maximum_work_items: payload.maximum_work_items,
            work: Some(payload.work),
            checkpoint_input,
            checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_byte_len: 0,
            checkpoint_page_cursor: 0,
            raw_input,
            raw: payload.raw,
            raw_page_cursor: 0,
            emit: None,
            ephemeral: None,
            phase,
            checkpoint_pending: false,
            work_progress: 0,
            closing: false,
        }
    }

    fn retained_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
        cx.payload_from_bytes(stream, bytes).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        })
    }

    fn checkpoint(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let mut state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES];
        let mut work_state = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES - ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES];
        let work_len = match self.work.as_ref().map(|work| work.checkpoint(&mut work_state)).transpose() {
            Ok(Some(length)) if length <= ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES - ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES => length,
            Ok(None) => 0,
            _ => return self.fault(cx, b"retained command checkpoint was rejected"),
        };
        let length = match encode_artifact_command_checkpoint(
            ArtifactCommandCheckpoint {
                work_phase: self.phase == ArtifactRetainedCommandPhase::Work,
                raw_page_cursor: self.raw_page_cursor as u64,
                raw_bytes: self.raw.len() as u64,
                work_progress: self.work_progress,
                context_digest: self.context.as_ref().map_or(0, |context| context.identity_digest()),
                workspace_identity: self.work.as_ref().map_or(0, |work| work.workspace_identity()),
                work: &work_state[..work_len],
            },
            &mut state,
        ) {
            Ok(length) => length,
            Err(_) => return self.fault(cx, b"retained command checkpoint could not be encoded"),
        };
        StepOutcome::CheckpointReady(Checkpoint { state: Self::retained_payload(cx, JobPayloadStream::CheckpointState, &state[..length]), applied_progress: self.work_progress.max(self.raw_page_cursor as u64) })
    }

    fn restore_checkpoint(&mut self) -> Result<(), Fault> {
        let bytes = &self.checkpoint_bytes[..self.checkpoint_byte_len];
        let input = self.raw_input.as_ref().ok_or_else(|| Fault::from("retained-command-checkpoint-wire-owner-absent"))?;
        let current_context_digest = self.context.as_ref().map_or(0, |context| context.identity_digest());
        let current_workspace_identity = self.work.as_ref().map_or(0, |work| work.workspace_identity());
        let checkpoint = decode_artifact_command_checkpoint(bytes, self.maximum_raw_bytes, input.page_count(), input.declared_bytes(), current_context_digest, current_workspace_identity)?;
        self.work_progress = checkpoint.work_progress;
        if checkpoint.work_phase {
            self.work.as_mut().ok_or_else(|| Fault::from("retained-command-checkpoint-work-absent"))?.restore(checkpoint.work)?;
        }
        self.raw.clear();
        self.raw_page_cursor = 0;
        Ok(())
    }

    fn preview(&self, cx: &mut StepContext<'_>, bytes: &'static [u8]) -> StepOutcome {
        StepOutcome::PreviewReady(Self::retained_payload(cx, JobPayloadStream::Preview, bytes))
    }

    fn fault(&mut self, cx: &mut StepContext<'_>, bytes: &'static [u8]) -> StepOutcome {
        self.phase = ArtifactRetainedCommandPhase::Fault;
        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, bytes) })
    }

    #[cfg(test)]
    pub(crate) fn test_pending_emit_shape(&self) -> Option<(usize, usize, Vec<String>)> {
        self.emit.as_ref().map(|emit| (emit.child_emits.len(), emit.artifact_mutations.len(), emit.child_emits.iter().map(|child| child.child_id.clone()).collect()))
    }
}

impl<A: ArtifactApp> InteractiveJob for ArtifactRetainedCommandJob<A> {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return StepOutcome::Yield;
        }
        cx.consume_fuel(1);
        if self.checkpoint_pending {
            self.checkpoint_pending = false;
            return self.checkpoint(cx);
        }
        match self.phase {
            ArtifactRetainedCommandPhase::CheckpointPages => {
                cx.set_stage("retained-command-checkpoint-page");
                let Some(checkpoint) = self.checkpoint_input.as_ref() else { return self.fault(cx, b"retained command lost checkpoint owner") };
                if let Some(page) = checkpoint.page(self.checkpoint_page_cursor) {
                    let Some(end) = self.checkpoint_byte_len.checked_add(page.len()).filter(|end| *end <= ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) else {
                        return self.fault(cx, b"retained command checkpoint exceeds capacity");
                    };
                    self.checkpoint_bytes[self.checkpoint_byte_len..end].copy_from_slice(page);
                    self.checkpoint_byte_len = end;
                    self.checkpoint_page_cursor = self.checkpoint_page_cursor.saturating_add(1);
                    return self.preview(cx, b"{\"en\":\"Reading checkpoint\",\"de\":\"Pr\xC3\xBCfpunkt wird gelesen\"}");
                }
                if self.restore_checkpoint().is_err() {
                    return self.fault(cx, b"retained command checkpoint is malformed");
                }
                self.checkpoint_input.as_mut().expect("checkpoint owner was validated above").begin_close();
                self.phase = ArtifactRetainedCommandPhase::CheckpointRetire;
                self.preview(cx, br#"{"en":"Restoring command","de":"Befehl wird wiederhergestellt"}"#)
            }
            ArtifactRetainedCommandPhase::CheckpointRetire => {
                cx.set_stage("retained-command-checkpoint-retire");
                let Some(checkpoint) = self.checkpoint_input.as_mut() else {
                    self.phase = ArtifactRetainedCommandPhase::WirePages;
                    return self.checkpoint(cx);
                };
                let step = checkpoint.close_step(1, semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES);
                if checkpoint.terminal_is_empty() {
                    self.checkpoint_input = None;
                    self.phase = ArtifactRetainedCommandPhase::WirePages;
                }
                match step {
                    InteractiveJobCloseStep::Blocked => StepOutcome::Yield,
                    InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete => self.preview(cx, b"{\"en\":\"Checkpoint restored\",\"de\":\"Pr\xC3\xBCfpunkt wiederhergestellt\"}"),
                }
            }
            ArtifactRetainedCommandPhase::WirePages => {
                cx.set_stage("retained-command-wire-page");
                let Some(input) = self.raw_input.as_ref() else { return self.fault(cx, b"retained command lost wire owner") };
                if let Some(page) = input.page(self.raw_page_cursor) {
                    let Some(end) = self.raw.len().checked_add(page.len()).filter(|end| *end <= self.maximum_raw_bytes) else {
                        return self.fault(cx, b"retained command exceeds raw capacity");
                    };
                    self.raw.extend_from_slice(page);
                    debug_assert_eq!(self.raw.len(), end);
                    self.raw_page_cursor = self.raw_page_cursor.saturating_add(1);
                    self.checkpoint_pending = true;
                    return self.preview(cx, br#"{"en":"Reading command page","de":"Befehlsseite wird gelesen"}"#);
                }
                self.phase = ArtifactRetainedCommandPhase::Decode;
                self.checkpoint(cx)
            }
            ArtifactRetainedCommandPhase::Decode => {
                cx.set_stage("retained-command-decode");
                let decoded = match <A::Command as protocol::OpBinary>::decode_op(&self.raw) {
                    Ok(command) => command,
                    Err(_) => return self.fault(cx, b"retained command wire payload is malformed"),
                };
                let Some(work) = self.work.as_ref() else { return self.fault(cx, b"retained command work owner is absent") };
                if (self.command_id)(&decoded) != work.tool_id() {
                    return self.fault(cx, b"retained command wire authority does not match tool");
                }
                self.command = Some(decoded);
                self.phase = ArtifactRetainedCommandPhase::Preflight;
                self.preview(cx, b"{\"en\":\"Validating command\",\"de\":\"Befehl wird gepr\xC3\xBCft\"}")
            }
            ArtifactRetainedCommandPhase::Preflight => {
                cx.set_stage("retained-command-preflight");
                let (Some(command), Some(snapshot), Some(interaction), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.interaction_state.as_ref(), self.work.as_ref()) else {
                    return self.fault(cx, b"retained command preflight owner is absent");
                };
                if !work.extent(command, snapshot, interaction, self.context.as_deref()).is_some_and(|extent| extent != 0 && extent <= self.maximum_work_items) {
                    return self.fault(cx, b"retained command exceeds semantic work capacity");
                }
                self.phase = ArtifactRetainedCommandPhase::Work;
                self.preview(cx, br#"{"en":"Applying command","de":"Befehl wird angewendet"}"#)
            }
            ArtifactRetainedCommandPhase::Work => {
                cx.set_stage("retained-command-work");
                let (Some(command), Some(snapshot), Some(config), Some(history), Some(interaction), Some(hover), Some(operation), Some(work)) =
                    (self.command.as_ref(), self.snapshot.as_ref(), self.config.as_ref(), self.history.as_ref(), self.interaction_state.as_ref(), self.interaction_hover.as_ref(), self.operation.as_ref(), self.work.as_mut())
                else {
                    return self.fault(cx, b"retained command reducer owner is absent");
                };
                match work.step(command, snapshot, config, history, interaction, hover, self.context.as_deref(), operation) {
                    Ok(ArtifactCommandWorkStep::Replay { stage, preview }) => {
                        cx.set_stage(stage);
                        self.checkpoint_pending = true;
                        self.preview(cx, preview)
                    }
                    Ok(ArtifactCommandWorkStep::Progress { stage, preview }) => {
                        cx.set_stage(stage);
                        self.work_progress = self.work_progress.saturating_add(1);
                        self.checkpoint_pending = true;
                        self.preview(cx, preview)
                    }
                    Ok(ArtifactCommandWorkStep::Complete(emit)) => {
                        self.emit = Some(emit);
                        self.ephemeral = Some(EphemeralEmit::default());
                        self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(cx, b"{\"en\":\"Publishing result\",\"de\":\"Ergebnis wird ver\xC3\xB6ffentlicht\"}")
                    }
                    Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral }) => {
                        self.emit = Some(emit);
                        self.ephemeral = Some(ephemeral);
                        self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(cx, b"{\"en\":\"Publishing result\",\"de\":\"Ergebnis wird ver\xC3\xB6ffentlicht\"}")
                    }
                    Err(_) => self.fault(cx, b"retained command reducer rejected operation"),
                }
            }
            ArtifactRetainedCommandPhase::Publish => {
                cx.set_stage("retained-command-publish");
                let Some(completion) = self.completion.as_ref() else { return self.fault(cx, b"retained command completion owner is absent") };
                if !completion.has_mounted_consumer() {
                    return self.fault(cx, b"retained command completion consumer is absent");
                }
                let Some(emit) = self.emit.take() else { return self.fault(cx, b"retained command result owner is absent") };
                let Some(ephemeral) = self.ephemeral.take() else { return self.fault(cx, b"retained command ephemeral result owner is absent") };
                if let Err(rejected) = completion.complete(Ok(emit), ephemeral) {
                    self.emit = rejected.emit.ok();
                    self.ephemeral = Some(rejected.ephemeral);
                    drop(rejected.fault);
                    return self.fault(cx, b"retained command result publication was rejected");
                }
                self.phase = ArtifactRetainedCommandPhase::Complete;
                StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
            }
            ArtifactRetainedCommandPhase::Complete => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
            ArtifactRetainedCommandPhase::Fault => self.fault(cx, b"retained command remains faulted"),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(checkpoint) = self.checkpoint_input.as_mut() {
            checkpoint.begin_close();
        }
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
        if let Some(work) = self.work.as_mut() {
            work.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if !self.raw.is_empty() {
            if maximum_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released = self.raw.len().min(maximum_bytes);
            self.raw.truncate(self.raw.len() - released);
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: released };
        }
        if self.raw.capacity() != 0 {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.raw = Vec::new();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(checkpoint) = self.checkpoint_input.as_mut() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let step = checkpoint.close_step(maximum_items.min(1), maximum_bytes);
            if checkpoint.terminal_is_empty() {
                self.checkpoint_input = None;
                return match step {
                    InteractiveJobCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    InteractiveJobCloseStep::Blocked => InteractiveJobCloseStep::Blocked,
                    InteractiveJobCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            return step;
        }
        if let Some(input) = self.raw_input.as_mut() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let step = input.close_step(maximum_items.min(1), maximum_bytes);
            if input.terminal_is_empty() {
                self.raw_input = None;
                return match step {
                    InteractiveJobCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    InteractiveJobCloseStep::Blocked => InteractiveJobCloseStep::Blocked,
                    InteractiveJobCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            return step;
        }
        macro_rules! retire_one {
            ($field:ident) => {
                if self.$field.is_some() {
                    if maximum_items == 0 {
                        return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    drop(self.$field.take());
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            };
        }
        if let Some(emit) = self.emit.as_mut() {
            if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                return match step {
                    crate::app::PluginCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    crate::app::PluginCloseStep::Blocked { .. } | crate::app::PluginCloseStep::AwaitingInput { .. } => InteractiveJobCloseStep::Blocked,
                    crate::app::PluginCloseStep::Complete => unreachable!("child close helper consumes completed children"),
                };
            }
        }
        retire_one!(emit);
        retire_one!(ephemeral);
        if let Some(work) = self.work.as_mut() {
            let step = work.close_step(maximum_items.min(1), maximum_bytes);
            if work.terminal_is_empty() {
                if maximum_items == 0 {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                self.work = None;
                return match step {
                    InteractiveJobCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items: released_items.max(1), released_bytes },
                    InteractiveJobCloseStep::Blocked => InteractiveJobCloseStep::Blocked,
                    InteractiveJobCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            return step;
        }
        retire_one!(command);
        retire_one!(snapshot);
        retire_one!(config);
        retire_one!(history);
        retire_one!(interaction_state);
        retire_one!(interaction_hover);
        retire_one!(context);
        retire_one!(operation);
        if let Some(completion) = self.completion.as_ref() {
            if maximum_items == 0 || !completion.has_mounted_consumer() {
                return if maximum_items == 0 { InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 } } else { InteractiveJobCloseStep::Blocked };
            }
            self.completion = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.raw.is_empty()
            && self.raw.capacity() == 0
            && self.checkpoint_input.is_none()
            && self.raw_input.is_none()
            && self.emit.is_none()
            && self.ephemeral.is_none()
            && self.work.is_none()
            && self.command.is_none()
            && self.snapshot.is_none()
            && self.config.is_none()
            && self.history.is_none()
            && self.interaction_state.is_none()
            && self.interaction_hover.is_none()
            && self.context.is_none()
            && self.operation.is_none()
            && self.completion.is_none()
    }
}
//#endregion 🧵️Job

#[cfg(test)]
pub(crate) fn test_raw_allocation_close<A: ArtifactApp>() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🚪️raw-allocation-close.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut raw = Vec::with_capacity(case["capacity"].as_u64().unwrap() as usize);
        raw.resize(case["initializedBytes"].as_u64().unwrap() as usize, 42);
        let mut job = ArtifactRetainedCommandJob::<A> {
            command: None, snapshot: None, config: None, history: None, interaction_state: None, interaction_hover: None, context: None, operation: None, completion: None,
            command_id: |_| "fixture", maximum_raw_bytes: raw.capacity(), maximum_work_items: 1, work: None, checkpoint_input: None,
            checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES], checkpoint_byte_len: 0, checkpoint_page_cursor: 0, raw_input: None, raw, raw_page_cursor: 0,
            emit: None, ephemeral: None, phase: ArtifactRetainedCommandPhase::Complete, checkpoint_pending: false, work_progress: 0, closing: false,
        };
        job.begin_close();
        let mut items = 0;
        let mut bytes = 0;
        for _ in 0..8 {
            match job.close_step(1, 4096) {
                InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= 4096);
                    items += released_items;
                    bytes += released_bytes;
                }
                InteractiveJobCloseStep::Complete => break,
                InteractiveJobCloseStep::Blocked => panic!("empty raw allocation must not require capacity-sized byte authority"),
            }
        }
        assert!(job.terminal_is_empty());
        assert_eq!(serde_json::json!({ "items": items, "bytes": bytes }), serde_json::json!({ "items": case["expectedAllocationRelease"], "bytes": case["expectedByteRelease"] }));
        eprintln!("[DEBUG] retained-command raw allocation {} released initialized bytes in4096-byte pages then one empty allocation", case["id"]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHECKPOINT_FIXTURE_JSON: &str = include_str!("🧪️fixtures/📸️artifact-command-checkpoint.json");

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CheckpointFixture {
        format: String,
        version: u8,
        maximum_bytes: usize,
        header_bytes: usize,
        cases: Vec<CheckpointCase>,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CheckpointCase {
        name: String,
        work_phase: bool,
        raw_page_cursor: u64,
        raw_bytes: u64,
        work_progress: u64,
        context_digest: u64,
        workspace_identity: u64,
        work_state: CheckpointWorkState,
        outcome: String,
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum CheckpointWorkState {
        Bytes { bytes: Vec<u8> },
        Fill { fill: u8, length: usize },
    }

    impl CheckpointWorkState {
        fn materialize(&self) -> Vec<u8> {
            match self {
                Self::Bytes { bytes } => bytes.clone(),
                Self::Fill { fill, length } => vec![*fill; *length],
            }
        }
    }

    fn write_owned_little_endian_u64(target: &mut Vec<u8>, value: u64) {
        for shift in (0..64).step_by(8) {
            target.push((value >> shift) as u8);
        }
    }

    fn owned_checkpoint_oracle(case: &CheckpointCase, work: &[u8]) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES + work.len());
        encoded.extend_from_slice(b"ARC1");
        encoded.push(3);
        encoded.push(u8::from(case.work_phase));
        encoded.extend_from_slice(&[0, 0]);
        write_owned_little_endian_u64(&mut encoded, case.raw_page_cursor);
        write_owned_little_endian_u64(&mut encoded, case.raw_bytes);
        write_owned_little_endian_u64(&mut encoded, case.work_progress);
        write_owned_little_endian_u64(&mut encoded, case.context_digest);
        write_owned_little_endian_u64(&mut encoded, case.workspace_identity);
        encoded.extend_from_slice(work);
        encoded
    }

    #[test]
    fn checkpoint_binary_matches_schema_fixture_and_owned_oracle() {
        let fixture: CheckpointFixture = serde_json::from_str(CHECKPOINT_FIXTURE_JSON).expect("checkpoint fixture shape");
        assert_eq!(fixture.format, "ARC1");
        assert_eq!(fixture.version, 3);
        assert_eq!(fixture.maximum_bytes, ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES);
        assert_eq!(fixture.header_bytes, ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES);
        for case in fixture.cases {
            let work = case.work_state.materialize();
            let mut bytes = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES];
            let encoded = encode_artifact_command_checkpoint(
                ArtifactCommandCheckpoint {
                    work_phase: case.work_phase,
                    raw_page_cursor: case.raw_page_cursor,
                    raw_bytes: case.raw_bytes,
                    work_progress: case.work_progress,
                    context_digest: case.context_digest,
                    workspace_identity: case.workspace_identity,
                    work: &work,
                },
                &mut bytes,
            );
            if case.outcome == "capacityError" {
                assert!(encoded.is_err(), "{} must reject maximum plus one", case.name);
                continue;
            }
            assert_eq!(case.outcome, "ok", "unknown fixture outcome for {}", case.name);
            let length = encoded.unwrap_or_else(|error| panic!("{} unexpectedly rejected: {error:?}", case.name));
            let oracle = owned_checkpoint_oracle(&case, &work);
            assert_eq!(&bytes[..length], oracle, "{} differs from owned oracle", case.name);
            let decoded = decode_artifact_command_checkpoint(&bytes[..length], case.raw_bytes as usize, case.raw_page_cursor as usize, case.raw_bytes as usize, case.context_digest, case.workspace_identity)
                .unwrap_or_else(|error| panic!("{} decode rejected: {error:?}", case.name));
            assert_eq!(decoded.work_phase, case.work_phase);
            assert_eq!(decoded.raw_page_cursor, case.raw_page_cursor);
            assert_eq!(decoded.raw_bytes, case.raw_bytes);
            assert_eq!(decoded.work_progress, case.work_progress);
            assert_eq!(decoded.context_digest, case.context_digest);
            assert_eq!(decoded.workspace_identity, case.workspace_identity);
            assert_eq!(decoded.work, work);
        }
    }

    #[test]
    fn owned_little_endian_oracle_preserves_every_hostile_byte_lane() {
        let mut bytes = Vec::new();
        write_owned_little_endian_u64(&mut bytes, 0x8877_6655_4433_2211);
        write_owned_little_endian_u64(&mut bytes, u64::MAX);
        assert_eq!(bytes, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift() {
        let mut bytes = [0_u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES];
        let length = encode_artifact_command_checkpoint(ArtifactCommandCheckpoint { work_phase: true, raw_page_cursor: 2, raw_bytes: 8_192, work_progress: 7, context_digest: 9, workspace_identity: 11, work: &[1, 0xfe] }, &mut bytes)
            .expect("exact checkpoint");
        assert!(decode_artifact_command_checkpoint(&bytes[..length], 8_192, 2, 8_192, 10, 11).is_err());
        assert!(decode_artifact_command_checkpoint(&bytes[..length], 8_192, 2, 8_192, 9, 12).is_err());
        bytes[6] = 1;
        assert!(decode_artifact_command_checkpoint(&bytes[..length], 8_192, 2, 8_192, 9, 11).is_err());
    }
}
