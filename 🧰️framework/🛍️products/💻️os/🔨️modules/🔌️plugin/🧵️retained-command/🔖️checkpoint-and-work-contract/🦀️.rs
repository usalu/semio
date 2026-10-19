//! 🔖️ Checkpoint and work contract of the retained command: the checkpoint size caps and codec, the reducer and extent types, the work step, inputs and retained capacity, the command work trait and its bounded implementation. See [`super`].

use super::*;

//#region 🔖️Work
pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;

pub(super) const ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES: usize = 48;

/// 🧯️ The longest reducer fault detail one job fault carries — the app's own code and message, clipped
/// to a single fault page so a runaway message narrows the report instead of losing it.
pub(super) const ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES: usize = 480;

const ARTIFACT_COMMAND_CHECKPOINT_MAGIC: [u8; 4] = *b"ARC1";

#[derive(Clone, Copy)]
pub(super) struct ArtifactCommandCheckpoint<'a> {
    pub(super) work_phase: bool,
    pub(super) raw_page_cursor: u64,
    pub raw_bytes: u64,
    pub(super) work_progress: u64,
    pub(super) context_digest: u64,
    pub(super) workspace_identity: u64,
    pub work: &'a [u8],
}

pub(super) fn encode_artifact_command_checkpoint(checkpoint: ArtifactCommandCheckpoint<'_>, target: &mut [u8]) -> Result<usize, Fault> {
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

pub(super) fn decode_artifact_command_checkpoint(bytes: &[u8], maximum_raw_bytes: usize, input_pages: usize, input_bytes: usize, current_context_digest: u64, current_workspace_identity: u64) -> Result<ArtifactCommandCheckpoint<'_>, Fault> {
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
    Option<&ArtifactOwnedToolJobContext<A>>,
    &AppOperationContext,
) -> Result<Emit<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, Fault>;

pub type ArtifactCommandExtent<A> = fn(&<A as ArtifactApp>::Command, &<A as ArtifactApp>::Snapshot, &protocol::InteractionState) -> Option<usize>;

pub enum ArtifactCommandWorkStep<A: ArtifactApp> {
    Pending { stage: &'static str, preview: &'static [u8] },
    Replay { stage: &'static str, preview: &'static [u8] },
    Progress { stage: &'static str, preview: &'static [u8] },
    Complete(Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>),
    CompleteDownload {download:ArtifactDownloadOutput,ephemeral:EphemeralEmit<A>},
    CompleteWithEphemeral { emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, ephemeral: EphemeralEmit<A> },
}

/// 🧾️ Immutable command roots and authority observed by one retained work step.
pub struct ArtifactCommandInputs<'a, A: ArtifactApp> {
    pub command: &'a A::Command,
    pub snapshot: &'a A::Snapshot,
    pub snapshot_owner: Option<&'a Arc<A::Snapshot>>,
    pub snapshot_read: ArtifactCommandSnapshotRead<'a,A::Snapshot>,
    pub config: &'a A::Config,
    pub history: &'a HistoryView,
    pub interaction: &'a protocol::InteractionState,
    pub hover: &'a InteractionHoverState,
    pub context: Option<&'a ArtifactOwnedToolJobContext<A>>,
    pub operation: &'a AppOperationContext,
}

/// 🧮️ The ONE quantity a retained route declares, in the store's OWN unit: staged edit ROWS.
///
/// A route used to spell that quantity three times and in two different units — the store footprint
/// its preflight declared (rows, via [`store::os_store::one_item::ArtifactStoreOneItemFootprint`]), the extent its work
/// answered [`ArtifactRetainedCommandPhase::Preflight`] with (items), and the `maximum_work_items`
/// its payload carried (items) — so nothing could compare them and the three drifted freely. They
/// are one declaration now: `work_items()` IS the preflight ceiling, `rows(1)` IS the footprint one
/// point-invertible durable item derives (`ArtifactStoreOneItemFootprint::for_leaf`), and `rows_for_items(n)` IS what an
/// `extent` returns. Every comparison the
/// runtime makes — preflight's `extent <= maximum_work_items`, `ArtifactStore::fold_batch_item`'s
/// `forwards.len() + inverse.len() <= footprint.work_items` — therefore measures the same thing
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactRetainedWorkCapacity {
    pub(super) invertible_items: usize,
}

impl ArtifactRetainedWorkCapacity {
    /// 🧾️ Declares a route that folds at most `invertible_items` point-invertible durable items.
    pub const fn for_invertible_items(invertible_items: usize) -> Self {
        Self { invertible_items }
    }

    pub const fn invertible_items(self) -> usize {
        self.invertible_items
    }

    /// 🧺️ Staged edit rows `items` point-invertible durable items fold — one forward row each plus
    /// the one row each `Mutation::inverse` yields.
    pub const fn rows(self, items: usize) -> usize {
        items.saturating_mul(store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS)
    }

    /// 🧾️ The preflight ceiling this route's payload carries as `maximum_work_items`.
    pub const fn work_items(self) -> usize {
        self.rows(self.invertible_items)
    }

    /// 🧮️ The extent an `ArtifactCommandWork` answers for a gesture of `items` durable items, or
    /// `None` once the route's own declared capacity is exceeded.
    pub const fn rows_for_items(self, items: usize) -> Option<usize> {
        let rows = self.rows(items);
        if self.admits(rows) {
            Some(rows)
        } else {
            None
        }
    }

    pub const fn admits(self, extent: usize) -> bool {
        extent != 0 && extent <= self.work_items()
    }
}

pub trait ArtifactCommandWork<A: ArtifactApp>: Send {
    fn tool_id(&self) -> &'static str;
    /// 📨️ Receives original owned command fields under the initiating caller before immutable work views begin.
    fn receive_original_command(&mut self,_command:&mut A::Command,_cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{Ok(true)}
    /// 🧰 Stable identity of the factory-provided mutable workspace retained by this job.
    fn workspace_identity(&self) -> u64 {
        0
    }
    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState, context: Option<&ArtifactOwnedToolJobContext<A>>) -> Option<usize>;
    /// 🚪️ Quotes the normal-work entry demand the host checks against the caller's remaining retained grant before calling [`Self::step`]; a work that declares nothing needs its entry item and one nesting level, and its actual effects are held to the grant by their receipts.
    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, A>, _maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, _cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, semio_framework_job::OriginalTurnFailure>;
    fn checkpoint_byte(&self, _index: usize) -> Option<u8> { None }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.is_empty() {
            Ok(())
        } else {
            Err(Fault::from("retained-command-work-checkpoint-unsupported"))
        }
    }
    fn terminal_frame_release_bytes(&self)->Option<usize>{None}
    /// 📣️ Whether this work reports phase previews and resumable checkpoints; a one-shot reducer finishes inside its first step, has no mid-work state to resume and owns no progress worth a host round trip.
    fn reports_progress(&self) -> bool {
        true
    }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _cx:&mut StepContext<'_>) -> Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        Ok(InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand_without_owner()}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{self.close_demand_without_owner()}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand_without_owner()}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand_without_owner()}
    fn close_demand_without_owner(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){Ok(0)}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"artifact command retained close demand is undeclared"))}}
    fn terminal_is_empty(&self) -> bool {
        true
    }
}

pub struct BoundedArtifactCommandWork<A: ArtifactApp> {
    pub tool_id: &'static str,
    pub reducer: ArtifactCommandReducer<A>,
    pub extent: ArtifactCommandExtent<A>,
    pub consumed: bool,
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

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, A>, _maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn reports_progress(&self) -> bool {
        false
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, _cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let ArtifactCommandInputs { snapshot_owner: _, snapshot_read: _, command, snapshot, config, history, interaction, hover, context, operation } = *input;
        if self.consumed {
            return Err(Fault::from("retained-command-bounded-work-repeated"));
        }
        self.consumed = true;
        (self.reducer)(command, snapshot, config, history, interaction, hover, context, operation).map(ArtifactCommandWorkStep::Complete)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}
//#endregion 🔖️Work
