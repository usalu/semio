//! 🧵️ Canonical 📡️progress implementation under its own native owner.
use semio_framework_job::*;
use semio_framework_async::ChannelPolicy;
//#region 📡️Progress
/// 🔖️ Opaque id for one addressable entity a [`ProgressEvent`] touches (a mesh, a brush placement, a
/// document node) — a bare `u64` so this crate never depends on any domain's entity-id type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(pub u64);

pub const JOB_PROGRESS_AFFECTED_ENTITIES: usize = 256;

#[derive(Debug, PartialEq, Eq)]
pub struct RetainedEntitySet {
    entries: [Option<EntityId>; JOB_PROGRESS_AFFECTED_ENTITIES],
    length: usize,
}

impl RetainedEntitySet {
    pub fn new() -> Self {
        Self { entries: [None; JOB_PROGRESS_AFFECTED_ENTITIES], length: 0 }
    }

    pub fn insert(&mut self, entity: EntityId) -> Result<(), EntityId> {
        if self.length == JOB_PROGRESS_AFFECTED_ENTITIES {
            return Err(entity);
        }
        self.entries[self.length] = Some(entity);
        self.length += 1;
        Ok(())
    }

    pub fn as_slice(&self) -> &[Option<EntityId>] {
        &self.entries[..self.length]
    }
}

impl Default for RetainedEntitySet {
    fn default() -> Self {
        Self::new()
    }
}

/// 🩺️ What kind of non-terminal report a [`ProgressEvent::Diagnostic`]/[`ProgressEvent::Failed`]
/// carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticKind {
    Info,
    Warning,
    Stalled,
    Error,
}

/// 📡️ The ten-event progress vocabulary (design ticket packet P2a item 4), proven by Puzzle 3D's
/// precompute session (design doc §6) — `Started`/`StageChanged`/`CandidateTested`/`PreviewPatch`/
/// `Diagnostic`/`Checkpoint`/`CommitCandidate`/`Completed`/`Cancelled`/`Failed`. This is a caller-side
/// UI/log projection, distinct from the trace ring [`drive_step`] writes to: a host assembles these
/// from [`StepOutcome`]s plus its own domain data (affected entities, quality/tolerance) to hand to a
/// UI over a channel governed by [`channel_policy_for`]/[`default_channel_kind_for`] — the trace ring
/// alone has no entity/quality/tolerance vocabulary, by design (it stays domain-neutral).
#[derive(Debug, PartialEq)]
#[expect(clippy::large_enum_variant, reason = "Progress events transfer retained payload and entity credits without an unbudgeted allocation per event.")]
pub enum ProgressEvent {
    Started {
        operation: OperationId,
        generation: Generation,
        base_revision: RevisionId,
        at_ms: u64,
    },
    StageChanged {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        stage: &'static str,
        at_ms: u64,
    },
    CandidateTested {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        entity: EntityId,
        accepted: bool,
        quality: f32,
        at_ms: u64,
    },
    PreviewPatch {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        base_revision: RevisionId,
        stage: &'static str,
        completed_units: u64,
        total_units: Option<u64>,
        quality: f32,
        tolerance: f32,
        affected: RetainedEntitySet,
        patch: RetainedJobPayload,
        at_ms: u64,
    },
    Diagnostic {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        kind: DiagnosticKind,
        detail: RetainedJobPayload,
        at_ms: u64,
    },
    Checkpoint {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        base_revision: RevisionId,
        applied_progress: u64,
        at_ms: u64,
    },
    CommitCandidate {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        base_revision: RevisionId,
        at_ms: u64,
    },
    Completed {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        at_ms: u64,
    },
    Cancelled {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        at_ms: u64,
    },
    Failed {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        kind: DiagnosticKind,
        detail: RetainedJobPayload,
        at_ms: u64,
    },
}

impl ProgressEvent {
    pub fn operation(&self) -> OperationId {
        match self {
            ProgressEvent::Started { operation, .. }
            | ProgressEvent::StageChanged { operation, .. }
            | ProgressEvent::CandidateTested { operation, .. }
            | ProgressEvent::PreviewPatch { operation, .. }
            | ProgressEvent::Diagnostic { operation, .. }
            | ProgressEvent::Checkpoint { operation, .. }
            | ProgressEvent::CommitCandidate { operation, .. }
            | ProgressEvent::Completed { operation, .. }
            | ProgressEvent::Cancelled { operation, .. }
            | ProgressEvent::Failed { operation, .. } => *operation,
        }
    }

    pub fn generation(&self) -> Generation {
        match self {
            ProgressEvent::Started { generation, .. }
            | ProgressEvent::StageChanged { generation, .. }
            | ProgressEvent::CandidateTested { generation, .. }
            | ProgressEvent::PreviewPatch { generation, .. }
            | ProgressEvent::Diagnostic { generation, .. }
            | ProgressEvent::Checkpoint { generation, .. }
            | ProgressEvent::CommitCandidate { generation, .. }
            | ProgressEvent::Completed { generation, .. }
            | ProgressEvent::Cancelled { generation, .. }
            | ProgressEvent::Failed { generation, .. } => *generation,
        }
    }
}

/// 🚰️ The six channel-policy categories the design ticket's progress-stream vocabulary names —
/// distinct from [`ProgressEvent`]'s ten variants because two categories (`PointerHover`/`Telemetry`)
/// are UI/sampling channels outside the job progress vocabulary itself, and one vocabulary variant
/// ([`ProgressEvent::PreviewPatch`]) splits across two categories by payload size (see
/// [`default_channel_kind_for`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressChannelKind {
    /// 🖱️ Pointer/hover UI events — latest-wins, one slot.
    PointerHover,
    /// 🎨️ Preview geometry — coalesced by `(operation, entity, stage)`.
    PreviewGeometry,
    /// 🔒️ Commits and checkpoints — lossless, bounded (never dropped, backpressure instead).
    CommitAndCheckpoint,
    /// ⚠️ Diagnostics — a bounded overwrite-oldest ring.
    DiagnosticRing,
    /// 📉️ Telemetry — lossy, latest sample only.
    Telemetry,
    /// 🪨️ Large preview geometry — byte-credit controlled.
    LargeGeometry,
}

/// 🔏️ The recommended [`ChannelPolicy`] for one [`ProgressChannelKind`] — design ticket packet P2a
/// item 4's channel-policy matrix, made concrete. A host wiring an actual channel may widen these
/// bounds for its own deployment; these are the floor every implementation should start from.
pub fn channel_policy_for(kind: ProgressChannelKind) -> ChannelPolicy {
    match kind {
        ProgressChannelKind::PointerHover => ChannelPolicy::LatestWins { max_bytes: 4 * 1024 },
        ProgressChannelKind::PreviewGeometry => ChannelPolicy::Coalesced { key: "operation:entity:stage".to_string(), max_items: 64, max_bytes: 4 * 1024 * 1024 },
        ProgressChannelKind::CommitAndCheckpoint => ChannelPolicy::LosslessBounded { max_items: 256, max_bytes: 16 * 1024 * 1024 },
        ProgressChannelKind::DiagnosticRing => ChannelPolicy::Ring { max_items: 128, max_bytes: 512 * 1024 },
        ProgressChannelKind::Telemetry => ChannelPolicy::LatestWins { max_bytes: 1024 },
        ProgressChannelKind::LargeGeometry => ChannelPolicy::ByteCredit { max_items: 32, max_bytes: 32 * 1024 * 1024 },
    }
}

/// 📏️ A [`ProgressEvent::PreviewPatch`] at or above this many patch bytes routes to
/// [`ProgressChannelKind::LargeGeometry`] instead of [`ProgressChannelKind::PreviewGeometry`].
pub const LARGE_PREVIEW_PATCH_BYTES: usize = 256 * 1024;

/// 🗺️ The recommended [`ProgressChannelKind`] for one [`ProgressEvent`] — the default routing a host
/// applies before [`channel_policy_for`].
pub fn default_channel_kind_for(event: &ProgressEvent) -> ProgressChannelKind {
    match event {
        ProgressEvent::Started { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::StageChanged { .. } => ProgressChannelKind::DiagnosticRing,
        ProgressEvent::CandidateTested { .. } => ProgressChannelKind::DiagnosticRing,
        ProgressEvent::PreviewPatch { patch, .. } if patch.len() >= LARGE_PREVIEW_PATCH_BYTES => ProgressChannelKind::LargeGeometry,
        ProgressEvent::PreviewPatch { .. } => ProgressChannelKind::PreviewGeometry,
        ProgressEvent::Diagnostic { .. } => ProgressChannelKind::DiagnosticRing,
        ProgressEvent::Checkpoint { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::CommitCandidate { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::Completed { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::Cancelled { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::Failed { .. } => ProgressChannelKind::CommitAndCheckpoint,
    }
}
//#endregion 📡️Progress
