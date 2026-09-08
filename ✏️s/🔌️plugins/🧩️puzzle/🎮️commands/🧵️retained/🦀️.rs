//! 🧵️ Fixed-capacity retained command session shared by Puzzle 2d, 3d, and 5d.

use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, Operation, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::{ArtifactToolCompletion, ArtifactToolCompletionRejection, EphemeralEmit, InteractionHoverState};
use semio_framework_plugin::{ArtifactApp, Emit, Fault, ToolJobFactoryError};
use std::sync::Arc;

//#region 🔖️Limits
pub const PUZZLE_COMMAND_RAW_BYTES: usize = 8_192;
pub const PUZZLE_COMMAND_DECODED_ITEMS: usize = 512;
pub const PUZZLE_COMMAND_WORK_ITEMS: usize = 4_096;
pub const PUZZLE_COMMAND_OUTPUT_BYTES: usize = 262_144;
pub const PUZZLE_COMMAND_STEP_MICROS: u32 = 7_500;
pub const PUZZLE_COMMAND_CHECKPOINT_BYTES: usize = 112;

pub fn puzzle_command_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(PUZZLE_COMMAND_RAW_BYTES, PUZZLE_COMMAND_DECODED_ITEMS, 1, PUZZLE_COMMAND_OUTPUT_BYTES, PUZZLE_COMMAND_STEP_MICROS, 1, 1)
}
//#endregion 🔖️Limits

//#region 🧵️Session
pub type PuzzleCommandReducer<A> = fn(
    &<A as ArtifactApp>::Command,
    &<A as ArtifactApp>::Snapshot,
    &<A as ArtifactApp>::Config,
    &protocol::InteractionState,
    &InteractionHoverState,
) -> Result<Emit<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, Fault>;

pub type PuzzleCommandExtent<A> = fn(&<A as ArtifactApp>::Command, &<A as ArtifactApp>::Snapshot, &protocol::InteractionState) -> Option<usize>;

pub enum PuzzleCommandWorkStep<A: ArtifactApp> {
    Progress { stage: &'static str, en: &'static str, de: &'static str },
    Complete(Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>),
}

pub trait PuzzleCommandWork<A: ArtifactApp>: Send {
    fn tool_id(&self) -> &'static str;
    fn bind_operation(&mut self, _operation: Operation) {}
    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState) -> Option<usize>;
    fn step(&mut self, command: &A::Command, snapshot: &A::Snapshot, config: &A::Config, interaction: &protocol::InteractionState, hover: &InteractionHoverState) -> Result<PuzzleCommandWorkStep<A>, Fault>;
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        true
    }
}

pub struct BoundedFirstStepCommandWork<A: ArtifactApp> {
    tool_id: &'static str,
    reducer: PuzzleCommandReducer<A>,
    extent: PuzzleCommandExtent<A>,
    consumed: bool,
}

impl<A: ArtifactApp> BoundedFirstStepCommandWork<A> {
    pub fn new(tool_id: &'static str, reducer: PuzzleCommandReducer<A>, extent: PuzzleCommandExtent<A>) -> Self {
        Self { tool_id, reducer, extent, consumed: false }
    }
}

impl<A: ArtifactApp> PuzzleCommandWork<A> for BoundedFirstStepCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        (self.extent)(command, snapshot, interaction)
    }

    fn step(&mut self, command: &A::Command, snapshot: &A::Snapshot, config: &A::Config, interaction: &protocol::InteractionState, hover: &InteractionHoverState) -> Result<PuzzleCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle-command-bounded-work-repeated"));
        }
        let emit = (self.reducer)(command, snapshot, config, interaction, hover)?;
        self.consumed = true;
        Ok(PuzzleCommandWorkStep::Complete(emit))
    }
}

pub struct NoopPuzzleCommandWork<A: ArtifactApp> {
    tool_id: &'static str,
    consumed: bool,
    marker: std::marker::PhantomData<fn() -> A>,
}

impl<A: ArtifactApp> NoopPuzzleCommandWork<A> {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, consumed: false, marker: std::marker::PhantomData }
    }
}

impl<A: ArtifactApp> PuzzleCommandWork<A> for NoopPuzzleCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &A::Command, _snapshot: &A::Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(1)
    }

    fn step(&mut self, _command: &A::Command, _snapshot: &A::Snapshot, _config: &A::Config, _interaction: &protocol::InteractionState, _hover: &InteractionHoverState) -> Result<PuzzleCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle-command-noop-repeated"));
        }
        self.consumed = true;
        Ok(PuzzleCommandWorkStep::Complete(Emit::default()))
    }
}

pub struct RetainedPuzzleCommandPayload<A: ArtifactApp> {
    pub command: A::Command,
    pub snapshot: Arc<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub interaction_state: Arc<protocol::InteractionState>,
    pub interaction_hover: Arc<InteractionHoverState>,
    pub completion: ArtifactToolCompletion<A>,
    pub command_id: fn(&A::Command) -> &'static str,
    pub work: Box<dyn PuzzleCommandWork<A>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum PuzzleCommandPhase {
    WirePages,
    WireBytes,
    Decode,
    Preflight,
    Work,
    WorkProgress,
    Publish,
    Complete,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PuzzleCommandCheckpointState {
    phase: PuzzleCommandPhase,
    operation: Operation,
    tool_hash: u64,
    input_hash: u64,
    raw_len: usize,
    raw_page_cursor: usize,
    raw_scan_cursor: usize,
    work_extent: usize,
    preflight_cursor: usize,
    work_cursor: usize,
}

impl PuzzleCommandCheckpointState {
    const MAGIC: [u8; 4] = *b"PZCP";
    const VERSION: u8 = 1;

    fn encode(self) -> [u8; PUZZLE_COMMAND_CHECKPOINT_BYTES] {
        let mut bytes = [0; PUZZLE_COMMAND_CHECKPOINT_BYTES];
        bytes[..4].copy_from_slice(&Self::MAGIC);
        bytes[4] = Self::VERSION;
        bytes[5] = self.phase as u8;
        let values = [
            self.operation.operation.0,
            self.operation.base_revision.0,
            self.operation.generation.0,
            self.operation.preview_sequence,
            self.operation.seed,
            self.tool_hash,
            self.input_hash,
            self.raw_len as u64,
            self.raw_page_cursor as u64,
            self.raw_scan_cursor as u64,
            self.work_extent as u64,
            self.preflight_cursor as u64,
            self.work_cursor as u64,
        ];
        for (index, value) in values.into_iter().enumerate() {
            let start = 8 + index * 8;
            bytes[start..start + 8].copy_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != PUZZLE_COMMAND_CHECKPOINT_BYTES || bytes[..4] != Self::MAGIC || bytes[4] != Self::VERSION || bytes[6..8] != [0, 0] {
            return None;
        }
        let phase = match bytes[5] {
            0 => PuzzleCommandPhase::WirePages,
            1 => PuzzleCommandPhase::WireBytes,
            2 => PuzzleCommandPhase::Decode,
            3 => PuzzleCommandPhase::Preflight,
            4 => PuzzleCommandPhase::Work,
            5 => PuzzleCommandPhase::WorkProgress,
            6 => PuzzleCommandPhase::Publish,
            7 => PuzzleCommandPhase::Complete,
            8 => PuzzleCommandPhase::Fault,
            _ => return None,
        };
        let read = |index: usize| {
            let start = 8 + index * 8;
            Some(u64::from_le_bytes(bytes[start..start + 8].try_into().ok()?))
        };
        let usize_at = |index| usize::try_from(read(index)?).ok();
        Some(Self {
            phase,
            operation: Operation { operation: semio_framework_job::OperationId(read(0)?), base_revision: semio_framework_job::RevisionId(read(1)?), generation: semio_framework_job::Generation(read(2)?), preview_sequence: read(3)?, seed: read(4)? },
            tool_hash: read(5)?,
            input_hash: read(6)?,
            raw_len: usize_at(7)?,
            raw_page_cursor: usize_at(8)?,
            raw_scan_cursor: usize_at(9)?,
            work_extent: usize_at(10)?,
            preflight_cursor: usize_at(11)?,
            work_cursor: usize_at(12)?,
        })
    }
}

fn puzzle_checkpoint_hash<'a>(parts: impl IntoIterator<Item = &'a [u8]>) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for part in parts {
        for byte in part {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}

fn retained_input_hash(input: &RetainedToolWireInput) -> Option<u64> {
    let mut hash = 0xcbf29ce484222325_u64;
    let mut bytes = 0usize;
    for index in 0..input.page_count() {
        let page = input.page(index)?;
        bytes = bytes.checked_add(page.len())?;
        for byte in page {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    (bytes == input.declared_bytes()).then_some(hash)
}

fn retained_checkpoint_bytes(checkpoint: &RetainedToolWireInput) -> Option<[u8; PUZZLE_COMMAND_CHECKPOINT_BYTES]> {
    if checkpoint.declared_bytes() != PUZZLE_COMMAND_CHECKPOINT_BYTES {
        return None;
    }
    let mut bytes = [0; PUZZLE_COMMAND_CHECKPOINT_BYTES];
    let mut cursor = 0usize;
    for index in 0..checkpoint.page_count() {
        let page = checkpoint.page(index)?;
        let end = cursor.checked_add(page.len()).filter(|end| *end <= bytes.len())?;
        bytes[cursor..end].copy_from_slice(page);
        cursor = end;
    }
    (cursor == bytes.len()).then_some(bytes)
}

pub struct RetainedPuzzleCommandJob<A: ArtifactApp> {
    operation: Operation,
    command: Option<A::Command>,
    snapshot: Option<Arc<A::Snapshot>>,
    config: Option<Arc<A::Config>>,
    interaction_state: Option<Arc<protocol::InteractionState>>,
    interaction_hover: Option<Arc<InteractionHoverState>>,
    completion: Option<ArtifactToolCompletion<A>>,
    command_id: fn(&A::Command) -> &'static str,
    work: Option<Box<dyn PuzzleCommandWork<A>>>,
    raw_input: Option<RetainedToolWireInput>,
    checkpoint_input: Option<RetainedToolWireInput>,
    raw: [u8; PUZZLE_COMMAND_RAW_BYTES],
    raw_len: usize,
    raw_page_cursor: usize,
    raw_scan_cursor: usize,
    work_extent: usize,
    preflight_cursor: usize,
    work_cursor: usize,
    pending_progress: Option<(&'static str, &'static str, &'static str)>,
    checkpoint_pending: bool,
    restore_target: Option<PuzzleCommandCheckpointState>,
    emit: Option<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<A>>,
    phase: PuzzleCommandPhase,
    closing: bool,
}

impl<A: ArtifactApp> RetainedPuzzleCommandJob<A> {
    pub fn new(operation: Operation, payload: RetainedPuzzleCommandPayload<A>) -> Self {
        Self::from_payload(operation, payload, None)
    }

    pub fn from_wire(operation: Operation, payload: RetainedPuzzleCommandPayload<A>, input: RetainedToolWireInput) -> Self {
        Self::from_payload(operation, payload, Some(input))
    }

    pub fn validate_wire_checkpoint(operation: Operation, payload: &RetainedPuzzleCommandPayload<A>, input: &RetainedToolWireInput, checkpoint: &RetainedToolWireInput) -> Result<(), ToolJobFactoryError> {
        let bytes = retained_checkpoint_bytes(checkpoint).ok_or_else(|| ToolJobFactoryError::new("Puzzle retained checkpoint has an invalid fixed-page extent"))?;
        let state = PuzzleCommandCheckpointState::decode(&bytes).ok_or_else(|| ToolJobFactoryError::new("Puzzle retained checkpoint is corrupt"))?;
        let input_hash = retained_input_hash(input).ok_or_else(|| ToolJobFactoryError::new("Puzzle retained command input is not exactly sealed"))?;
        let tool_hash = puzzle_checkpoint_hash([payload.work.tool_id().as_bytes()]);
        let phase_is_resumable = matches!(state.phase, PuzzleCommandPhase::WireBytes | PuzzleCommandPhase::Decode | PuzzleCommandPhase::Preflight | PuzzleCommandPhase::WorkProgress);
        let cursors_are_bounded = state.raw_len == input.declared_bytes()
            && state.raw_page_cursor == input.page_count()
            && state.raw_scan_cursor <= state.raw_len
            && state.work_extent <= PUZZLE_COMMAND_WORK_ITEMS
            && state.preflight_cursor <= state.work_extent.max(1)
            && state.work_cursor <= PUZZLE_COMMAND_WORK_ITEMS
            && match state.phase {
                PuzzleCommandPhase::WireBytes => state.work_extent == 0 && state.preflight_cursor == 0 && state.work_cursor == 0,
                PuzzleCommandPhase::Decode => state.raw_scan_cursor == state.raw_len && state.work_extent == 0 && state.preflight_cursor == 0 && state.work_cursor == 0,
                PuzzleCommandPhase::Preflight => state.raw_scan_cursor == state.raw_len && state.work_extent != 0 && state.preflight_cursor != 0 && state.work_cursor == 0,
                PuzzleCommandPhase::WorkProgress => state.raw_scan_cursor == state.raw_len && state.work_extent != 0 && state.preflight_cursor == state.work_extent && state.work_cursor != 0,
                _ => false,
            };
        let operation_matches = state.operation.operation == operation.operation && state.operation.base_revision == operation.base_revision && state.operation.generation == operation.generation && state.operation.seed == operation.seed;
        if !operation_matches || state.tool_hash != tool_hash || state.input_hash != input_hash || !phase_is_resumable || !cursors_are_bounded {
            return Err(ToolJobFactoryError::new("Puzzle retained checkpoint authority or cursor state is stale"));
        }
        Ok(())
    }

    pub fn from_validated_wire_checkpoint(operation: Operation, payload: RetainedPuzzleCommandPayload<A>, input: RetainedToolWireInput, checkpoint: RetainedToolWireInput) -> Self {
        let bytes = retained_checkpoint_bytes(&checkpoint).expect("validated Puzzle checkpoint bytes");
        let state = PuzzleCommandCheckpointState::decode(&bytes).expect("validated Puzzle checkpoint state");
        let mut job = Self::from_payload(operation, payload, Some(input));
        job.checkpoint_input = Some(checkpoint);
        job.restore_target = Some(state);
        job
    }

    fn from_payload(operation: Operation, mut payload: RetainedPuzzleCommandPayload<A>, raw_input: Option<RetainedToolWireInput>) -> Self {
        let phase = if raw_input.is_some() { PuzzleCommandPhase::WirePages } else { PuzzleCommandPhase::Preflight };
        payload.work.bind_operation(operation);
        Self {
            operation,
            command: Some(payload.command),
            snapshot: Some(payload.snapshot),
            config: Some(payload.config),
            interaction_state: Some(payload.interaction_state),
            interaction_hover: Some(payload.interaction_hover),
            completion: Some(payload.completion),
            command_id: payload.command_id,
            work: Some(payload.work),
            raw_input,
            checkpoint_input: None,
            raw: [0; PUZZLE_COMMAND_RAW_BYTES],
            raw_len: 0,
            raw_page_cursor: 0,
            raw_scan_cursor: 0,
            work_extent: 0,
            preflight_cursor: 0,
            work_cursor: 0,
            pending_progress: None,
            checkpoint_pending: false,
            restore_target: None,
            emit: None,
            pending_completion_rejection: None,
            phase,
            closing: false,
        }
    }

    fn retained_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
        cx.payload_from_bytes(stream, bytes).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        })
    }

    fn checkpoint_state(&self) -> [u8; PUZZLE_COMMAND_CHECKPOINT_BYTES] {
        let tool_hash = self.work.as_ref().map_or(0, |work| puzzle_checkpoint_hash([work.tool_id().as_bytes()]));
        let input_hash = puzzle_checkpoint_hash([&self.raw[..self.raw_len]]);
        PuzzleCommandCheckpointState {
            phase: self.phase,
            operation: self.operation,
            tool_hash,
            input_hash,
            raw_len: self.raw_len,
            raw_page_cursor: self.raw_page_cursor,
            raw_scan_cursor: self.raw_scan_cursor,
            work_extent: self.work_extent,
            preflight_cursor: self.preflight_cursor,
            work_cursor: self.work_cursor,
        }
        .encode()
    }

    fn publish_checkpoint(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let state = self.checkpoint_state();
        let payload = match cx.payload_from_bytes(JobPayloadStream::CheckpointState, &state) {
            Ok(payload) => payload,
            Err(rejected) => {
                drop(rejected.into_source());
                return StepOutcome::Yield;
            }
        };
        self.checkpoint_pending = false;
        StepOutcome::CheckpointReady(Checkpoint { state: payload, applied_progress: self.raw_page_cursor.saturating_add(self.raw_scan_cursor).saturating_add(self.preflight_cursor).saturating_add(self.work_cursor) as u64 })
    }

    fn checkpoint(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if self.restore_target.is_some() {
            return StepOutcome::Yield;
        }
        self.checkpoint_pending = true;
        self.publish_checkpoint(cx)
    }

    fn preview(&self, cx: &mut StepContext<'_>, en: &str, de: &str) -> StepOutcome {
        if self.restore_target.is_some() {
            return StepOutcome::Yield;
        }
        let bytes = serde_json::to_vec(&serde_json::json!({ "en": en, "de": de })).unwrap_or_default();
        StepOutcome::PreviewReady(Self::retained_payload(cx, JobPayloadStream::Preview, &bytes))
    }

    fn fault(&mut self, cx: &mut StepContext<'_>, message: &'static [u8]) -> StepOutcome {
        self.phase = PuzzleCommandPhase::Fault;
        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, message) })
    }

    fn matches_restore_target(&self, target: PuzzleCommandCheckpointState) -> bool {
        self.phase == target.phase
            && self.raw_len == target.raw_len
            && self.raw_page_cursor == target.raw_page_cursor
            && self.raw_scan_cursor == target.raw_scan_cursor
            && self.work_extent == target.work_extent
            && self.preflight_cursor == target.preflight_cursor
            && self.work_cursor == target.work_cursor
    }

    fn restore_has_passed_target(&self, target: PuzzleCommandCheckpointState) -> bool {
        match target.phase {
            PuzzleCommandPhase::WireBytes => self.phase != PuzzleCommandPhase::WirePages && (self.phase != PuzzleCommandPhase::WireBytes || self.raw_scan_cursor > target.raw_scan_cursor),
            PuzzleCommandPhase::Decode => !matches!(self.phase, PuzzleCommandPhase::WirePages | PuzzleCommandPhase::WireBytes | PuzzleCommandPhase::Decode),
            PuzzleCommandPhase::Preflight => {
                !matches!(self.phase, PuzzleCommandPhase::WirePages | PuzzleCommandPhase::WireBytes | PuzzleCommandPhase::Decode | PuzzleCommandPhase::Preflight)
                    || (self.phase == PuzzleCommandPhase::Preflight && self.preflight_cursor > target.preflight_cursor)
            }
            PuzzleCommandPhase::WorkProgress => matches!(self.phase, PuzzleCommandPhase::Publish | PuzzleCommandPhase::Complete | PuzzleCommandPhase::Fault) || self.work_cursor > target.work_cursor,
            _ => true,
        }
    }

    fn step_inner(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return StepOutcome::Yield;
        }
        if self.checkpoint_pending {
            return self.publish_checkpoint(cx);
        }
        cx.consume_fuel(1);
        match self.phase {
            PuzzleCommandPhase::WirePages => {
                cx.set_stage("puzzle-command-wire-page");
                let Some(input) = self.raw_input.as_ref() else { return self.fault(cx, b"puzzle command lost retained wire input") };
                if let Some(page) = input.page(self.raw_page_cursor) {
                    let Some(end) = self.raw_len.checked_add(page.len()).filter(|end| *end <= self.raw.len()) else { return self.fault(cx, b"puzzle command wire input exceeds fixed byte capacity") };
                    self.raw[self.raw_len..end].copy_from_slice(page);
                    self.raw_len = end;
                    self.raw_page_cursor = self.raw_page_cursor.saturating_add(1);
                    return self.preview(cx, "Reading command page", "Befehlsseite wird gelesen");
                }
                self.phase = PuzzleCommandPhase::WireBytes;
                self.checkpoint(cx)
            }
            PuzzleCommandPhase::WireBytes => {
                cx.set_stage("puzzle-command-wire-byte");
                if self.raw_scan_cursor < self.raw_len {
                    self.raw_scan_cursor = self.raw_scan_cursor.saturating_add(1);
                    return self.checkpoint(cx);
                }
                self.phase = PuzzleCommandPhase::Decode;
                self.checkpoint(cx)
            }
            PuzzleCommandPhase::Decode => {
                cx.set_stage("puzzle-command-decode");
                let decoded = match <A::Command as protocol::OpBinary>::decode_op(&self.raw[..self.raw_len]) {
                    Ok(command) => command,
                    Err(_) => return self.fault(cx, b"puzzle command wire payload is malformed"),
                };
                let Some(work) = self.work.as_ref() else { return self.fault(cx, b"puzzle command work owner is absent") };
                if (self.command_id)(&decoded) != work.tool_id() {
                    return self.fault(cx, b"puzzle command wire authority does not match the admitted tool");
                }
                self.command = Some(decoded);
                self.phase = PuzzleCommandPhase::Preflight;
                self.preview(cx, "Validating command capacity", "Befehlskapazität wird geprüft")
            }
            PuzzleCommandPhase::Preflight => {
                cx.set_stage("puzzle-command-preflight");
                if self.work_extent == 0 {
                    let (Some(command), Some(snapshot), Some(interaction), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.interaction_state.as_ref(), self.work.as_ref()) else {
                        return self.fault(cx, b"puzzle command preflight owner is absent");
                    };
                    let Some(extent) = work.extent(command, snapshot, interaction).filter(|extent| *extent <= PUZZLE_COMMAND_WORK_ITEMS) else { return self.fault(cx, b"puzzle command exceeds fixed semantic work capacity") };
                    self.work_extent = extent.max(1);
                }
                if self.preflight_cursor < self.work_extent {
                    self.preflight_cursor = self.preflight_cursor.saturating_add(1);
                    return self.checkpoint(cx);
                }
                self.phase = PuzzleCommandPhase::Work;
                self.preview(cx, "Applying command", "Befehl wird angewendet")
            }
            PuzzleCommandPhase::Work => {
                cx.set_stage("puzzle-command-work");
                let (Some(command), Some(snapshot), Some(config), Some(interaction), Some(hover), Some(work)) =
                    (self.command.as_ref(), self.snapshot.as_ref(), self.config.as_ref(), self.interaction_state.as_ref(), self.interaction_hover.as_ref(), self.work.as_mut())
                else {
                    return self.fault(cx, b"puzzle command work owner is absent");
                };
                match work.step(command, snapshot, config, interaction, hover) {
                    Ok(PuzzleCommandWorkStep::Progress { stage, en, de }) => {
                        self.work_cursor = self.work_cursor.saturating_add(1);
                        self.pending_progress = Some((stage, en, de));
                        self.phase = PuzzleCommandPhase::WorkProgress;
                        self.checkpoint(cx)
                    }
                    Ok(PuzzleCommandWorkStep::Complete(emit)) => {
                        self.emit = Some(emit);
                        self.phase = PuzzleCommandPhase::Publish;
                        self.preview(cx, "Publishing result", "Ergebnis wird veröffentlicht")
                    }
                    Err(_) => self.fault(cx, b"puzzle command reducer rejected the admitted operation"),
                }
            }
            PuzzleCommandPhase::WorkProgress => {
                let Some((stage, en, de)) = self.pending_progress.take() else { return self.fault(cx, b"puzzle command progress owner is absent") };
                cx.set_stage(stage);
                self.phase = PuzzleCommandPhase::Work;
                self.preview(cx, en, de)
            }
            PuzzleCommandPhase::Publish => {
                cx.set_stage("puzzle-command-publish");
                let Some(completion) = self.completion.as_ref() else { return self.fault(cx, b"puzzle command completion authority is absent") };
                if !completion.has_mounted_consumer() {
                    return self.fault(cx, b"puzzle command completion consumer is absent");
                }
                let Some(emit) = self.emit.take() else { return self.fault(cx, b"puzzle command result owner is absent") };
                if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                    self.pending_completion_rejection = Some(rejected);
                    return self.fault(cx, b"puzzle command result publication was rejected");
                }
                self.phase = PuzzleCommandPhase::Complete;
                StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
            }
            PuzzleCommandPhase::Complete => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
            PuzzleCommandPhase::Fault => self.fault(cx, b"puzzle command remains faulted"),
        }
    }
}

impl<A: ArtifactApp> InteractiveJob for RetainedPuzzleCommandJob<A> {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let outcome = self.step_inner(cx);
        let Some(target) = self.restore_target else { return outcome };
        if matches!(outcome, StepOutcome::Fault(_) | StepOutcome::Cancelled) {
            return outcome;
        }
        if self.matches_restore_target(target) {
            self.restore_target = None;
        } else if self.restore_has_passed_target(target) {
            return self.fault(cx, b"puzzle command checkpoint replay diverged");
        }
        StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.pending_progress = None;
        self.checkpoint_pending = false;
        self.restore_target = None;
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
        if let Some(checkpoint) = self.checkpoint_input.as_mut() {
            checkpoint.begin_close();
        }
        if let Some(work) = self.work.as_mut() {
            work.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if self.raw_len != 0 {
            if maximum_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released_bytes = self.raw_len.min(maximum_bytes);
            let retained = self.raw_len - released_bytes;
            self.raw[retained..self.raw_len].fill(0);
            self.raw_len = retained;
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes };
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
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return match step {
                        semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items, released_bytes },
                        semio_framework_plugin::PluginCloseStep::Blocked { .. } | semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } => InteractiveJobCloseStep::Blocked,
                        semio_framework_plugin::PluginCloseStep::Complete => unreachable!("child close helper consumes completed children"),
                    };
                }
            }
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.pending_completion_rejection = None;
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
        retire_one!(emit);
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
        retire_one!(interaction_state);
        retire_one!(interaction_hover);
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
            && self.raw_len == 0
            && self.raw_input.is_none()
            && self.checkpoint_input.is_none()
            && self.pending_completion_rejection.is_none()
            && self.emit.is_none()
            && self.work.is_none()
            && self.command.is_none()
            && self.snapshot.is_none()
            && self.config.is_none()
            && self.interaction_state.is_none()
            && self.interaction_hover.is_none()
            && self.completion.is_none()
    }
}
//#endregion 🧵️Session

//#region 🧪️FixtureOracle
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️FixtureOracle
