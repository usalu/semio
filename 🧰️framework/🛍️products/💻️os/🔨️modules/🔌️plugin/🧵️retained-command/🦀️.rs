//! 🧵️ Shared retained shell for app-owned typed command reducers.

use crate::app::{AppOperationContext, ArtifactApp, ArtifactOwnedToolJobContext, ArtifactToolCompletion, ArtifactDownloadOutput, Emit, EphemeralEmit, HistoryView, InteractionHoverState};
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework::Fault;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, JobPublicationKind, RetainedFaultPublication, RetainedJobPayload, RetainedJobPublication, StepContext};
use std::sync::Arc;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retirement::controlled::ControlledRetirement};
use std::mem::MaybeUninit;

#[path="♻️metadata/🦀️.rs"]
mod metadata_retirement;
#[path="🎟️admission/🦀️.rs"]
mod admission;
pub use admission::ArtifactRetainedAdmissionRefusal;
#[path="📸️checkpoint/👣️cursor/🦀️.rs"]
mod checkpoint_cursor;
pub use checkpoint_cursor::ArtifactCommandCheckpointCursor;
#[path="🧬️context/🦀️.rs"]
mod context;
pub use context::ArtifactOwnedContextHandle;
pub(crate) use context::{ContextOriginalResidual,ContextOwnedFields,ContextRetirementParts,ContextToolRunMetadata,ContextToolRunSources};

//#region 🔖️Work
pub const ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES: usize = 512;
const ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES: usize = 48;
/// 🧯️ The longest reducer fault detail one job fault carries — the app's own code and message, clipped
/// to a single fault page so a runaway message narrows the report instead of losing it.
const ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES: usize = 480;
const ARTIFACT_COMMAND_CHECKPOINT_MAGIC: [u8; 4] = *b"ARC1";

#[derive(Clone, Copy)]
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
    Option<&ArtifactOwnedToolJobContext<A>>,
    &AppOperationContext,
) -> Result<Emit<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, Fault>;

pub type ArtifactCommandExtent<A> = fn(&<A as ArtifactApp>::Command, &<A as ArtifactApp>::Snapshot, &protocol::InteractionState) -> Option<usize>;

pub enum ArtifactCommandWorkStep<A: ArtifactApp> {
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
/// its preflight declared (rows, via [`store::ArtifactStoreOneItemFootprint`]), the extent its work
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
    invertible_items: usize,
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
        items.saturating_mul(store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS)
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
    /// 🧰 Stable identity of the factory-provided mutable workspace retained by this job.
    fn workspace_identity(&self) -> u64 {
        0
    }
    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState, context: Option<&ArtifactOwnedToolJobContext<A>>) -> Option<usize>;
    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, A>, _maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "artifact command normal-work demand is undeclared"))
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, Fault>;
    fn checkpoint_byte(&self, _index: usize) -> Option<u8> { None }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.is_empty() {
            Ok(())
        } else {
            Err(Fault::from("retained-command-work-checkpoint-unsupported"))
        }
    }
    fn terminal_frame_release_bytes(&self)->Option<usize>{None}
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _grant:RetainedCloneGrant) -> InteractiveJobCloseStep {
        InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()}
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

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, A>, _maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, Fault> {
        let ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config, history, interaction, hover, context, operation } = *input;
        if self.consumed {
            return Err(Fault::from("retained-command-bounded-work-repeated"));
        }
        self.consumed = true;
        (self.reducer)(command, snapshot, config, history, interaction, hover, context, operation).map(ArtifactCommandWorkStep::Complete)
    }
}
//#endregion 🔖️Work

//#region 🧳️Payload
/// 🧳️ Owned command roots and completion authority admitted into a retained payload.
pub struct ArtifactRetainedCommandInputs<A: ArtifactApp> {
    pub command: A::Command,
    pub snapshot: Arc<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub history: Arc<HistoryView>,
    pub interaction_state: Arc<protocol::InteractionState>,
    pub interaction_hover: Arc<InteractionHoverState>,
    pub context: Option<ArtifactOwnedContextHandle<A>>,
    pub operation: AppOperationContext,
    pub completion: ArtifactToolCompletion<A>,
}

pub struct ArtifactRetainedCommandPayload<A: ArtifactApp> {
    pub command: A::Command,
    pub snapshot: Arc<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub history: Arc<HistoryView>,
    pub interaction_state: Arc<protocol::InteractionState>,
    pub interaction_hover: Arc<InteractionHoverState>,
    pub context: Option<ArtifactOwnedContextHandle<A>>,
    pub operation: AppOperationContext,
    pub completion: ArtifactToolCompletion<A>,
    pub command_id: fn(&A::Command) -> &'static str,
    pub maximum_raw_bytes: usize,
    pub maximum_work_items: usize,
    pub raw: Vec<u8>,
    pub admission_refusal:Option<ArtifactRetainedAdmissionRefusal>,
    pub work: Box<dyn ArtifactCommandWork<A>>,
}

impl<A: ArtifactApp> ArtifactRetainedCommandPayload<A> {
    pub fn new(inputs: ArtifactRetainedCommandInputs<A>, command_id: fn(&A::Command) -> &'static str, maximum_raw_bytes: usize, maximum_work_items: usize, work: Box<dyn ArtifactCommandWork<A>>) -> Self {
        let (raw,admission_refusal)=admission::admit_raw(maximum_raw_bytes,maximum_work_items);
        let ArtifactRetainedCommandInputs { command, snapshot, config, history, interaction_state, interaction_hover, context, operation, completion } = inputs;
        Self { command, snapshot, config, history, interaction_state, interaction_hover, context, operation, completion, command_id, maximum_raw_bytes, maximum_work_items, raw, admission_refusal,work }

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

#[derive(Clone,Copy)]
enum ArtifactCommandPublication {
    Static { kind: JobPublicationKind, bytes: &'static [u8] },
    Checkpoint,
}

pub struct ArtifactRetainedCommandJob<A: ArtifactApp> {
    command: Option<A::Command>,
    snapshot: Option<Arc<A::Snapshot>>,
    config: Option<Arc<A::Config>>,
    history: Option<Arc<HistoryView>>,
    interaction_state: Option<Arc<protocol::InteractionState>>,
    interaction_hover: Option<Arc<InteractionHoverState>>,
    context: Option<ArtifactOwnedContextHandle<A>>,
    operation: Option<AppOperationContext>,
    completion: Option<ArtifactToolCompletion<A>>,
    command_id: fn(&A::Command) -> &'static str,
    maximum_raw_bytes: usize,
    maximum_work_items: usize,
    work: Option<Box<dyn ArtifactCommandWork<A>>>,
    checkpoint_input: Option<RetainedToolWireInput>,
    checkpoint_bytes: [u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
    checkpoint_target: [MaybeUninit<u8>; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
    checkpoint_capture: Option<ArtifactCommandCheckpointCursor>,
    publication: RetainedJobPublication,
    fault_publication: RetainedFaultPublication,
    pending_publication: Option<ArtifactCommandPublication>,
    publication_delivered: bool,
    fault: Option<Fault>,
    fault_retirement: Option<ControlledRetirement<Fault>>,
    decode_error: Option<protocol::ProtocolError>,
    checkpoint_byte_len: usize,
    checkpoint_page_cursor: usize,
    checkpoint_page_offset: usize,
    raw_input: Option<RetainedToolWireInput>,
    raw: Vec<u8>,
    admission_refusal:Option<ArtifactRetainedAdmissionRefusal>,
    raw_page_cursor: usize,
    raw_page_offset: usize,
    emit: Option<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>,
    download:Option<ArtifactDownloadOutput>,
    download_retirement:Option<ControlledRetirement<ArtifactDownloadOutput>>,
    completion_retirement:Option<ControlledRetirement<ArtifactToolCompletion<A>>>,
    context_retirement:Option<ControlledRetirement<ArtifactOwnedContextHandle<A>>>,
    ephemeral: Option<EphemeralEmit<A>>,
    phase: ArtifactRetainedCommandPhase,
    checkpoint_pending: bool,
    work_progress: u64,
    closing: bool,
}

impl<A: ArtifactApp> ArtifactRetainedCommandJob<A> {
    fn controlled_close_step(step:Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>)->InteractiveJobCloseStep{match step{Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress))=>InteractiveJobCloseStep::Pending{progress},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}}
    fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.publication.terminal_is_empty(){return self.publication.retirement_demands();}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands();}
        if let Some(cursor)=self.checkpoint_capture.as_ref().filter(|cursor|!cursor.terminal_is_empty()){return Ok(cursor.retirement_demands());}
        if self.checkpoint_capture.is_some()||self.pending_publication.is_some()||self.publication_delivered{return Ok(RetirementDemand{depth:1,..Default::default()});}
        if self.fault.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.fault_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
        if self.decode_error.is_some(){return protocol::protocol_error_retirement_demand(&self.decode_error);}
        if self.admission_refusal.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if !self.raw.is_empty(){return Ok(RetirementDemand{copy_bytes:1,depth:1,..Default::default()});}
        if self.raw.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.raw.capacity(),depth:1,..Default::default()});}
        for input in [&self.checkpoint_input,&self.raw_input]{if let Some(owner)=input.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_close_copy_byte_demand()?,capacity_bytes:owner.next_close_capacity_byte_demand(copy)?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?});}}
        if self.download.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.download_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
        if let Some(emit)=self.emit.as_ref(){
            if let Some(mut demand)=emit.child_close_demands(copy)?{
                demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained emitted child depth overflow"))?;
                return Ok(demand);
            }
            return Err(Self::unadmitted_input());
        }
        if self.ephemeral.is_some(){return Err(Self::unadmitted_input());}
        if let Some(work)=self.work.as_ref(){return if work.terminal_is_empty(){Ok(RetirementDemand{release_bytes:work.terminal_frame_release_bytes().ok_or_else(Self::unadmitted_input)?,depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:work.next_close_copy_byte_demand()?,capacity_bytes:work.next_close_capacity_byte_demand(copy)?,release_bytes:work.next_close_release_byte_demand()?,depth:work.next_close_depth_demand()?})};}
        if self.context.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.context_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
        if self.command.is_some()||self.snapshot.is_some()||self.config.is_some()||self.history.is_some()||self.interaction_state.is_some()||self.interaction_hover.is_some()||self.operation.is_some(){return Err(Self::unadmitted_input());}
        if self.completion.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.completion_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
        Ok(Default::default())
    }
    fn unadmitted_input()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained command input requires its original app-owned retirement authority")}

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
        let phase = if payload.admission_refusal.is_some(){ArtifactRetainedCommandPhase::Fault}else if checkpoint_input.is_some() {
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
            checkpoint_target: [MaybeUninit::uninit(); ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_capture: None,
            publication: RetainedJobPublication::new(),
            fault_publication: RetainedFaultPublication::new(),
            pending_publication: None,
            publication_delivered: false,
            fault: None,
            fault_retirement: None,
            decode_error: None,
            checkpoint_byte_len: 0,
            checkpoint_page_cursor: 0,
            checkpoint_page_offset: 0,
            raw_input,
            raw: payload.raw,
            admission_refusal:payload.admission_refusal,
            raw_page_cursor: 0,
            raw_page_offset: 0,
            emit: None,
            download:None,
            download_retirement:None,
            completion_retirement:None,
            context_retirement:None,
            ephemeral: None,
            phase,
            checkpoint_pending: false,
            work_progress: 0,
            closing: false,
        }
    }

    fn metadata(cx: &mut StepContext<'_>) -> Result<bool, ValueError> {
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(false); }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(true)
    }

    fn receive_step(cx: &mut StepContext<'_>, step: Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
        match step {
            Ok(step) => { cx.consume_retained(step.progress())?; Ok(step) }
            Err(error) => { cx.consume_retained(error.retained_progress())?; Err(error) }
        }
    }

    fn retire_publication(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        if !self.publication.terminal_is_empty() {
            let grant=cx.retained_grant();
            let step=self.publication.close_step(grant);
            Self::receive_step(cx,step)?;
            return Ok(());
        }
        if !self.fault_publication.terminal_is_empty() {
            let grant=cx.retained_grant();
            let step=self.fault_publication.close_step(grant);
            Self::receive_step(cx,step)?;
            return Ok(());
        }
        if let Some(cursor) = self.checkpoint_capture.as_mut().filter(|cursor| !cursor.terminal_is_empty()) {
            let grant=cx.retained_grant();
            let step=cursor.close_step(grant);
            Self::receive_step(cx,step)?;
            return Ok(());
        }
        if Self::metadata(cx)? {
            if matches!(self.pending_publication, Some(ArtifactCommandPublication::Checkpoint)) { self.checkpoint_pending = false; }
            self.checkpoint_capture.take();
            self.pending_publication = None;
            self.publication_delivered = false;
        }
        Ok(())
    }

    fn checkpoint<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.checkpoint_capture.is_none() {
            if !Self::metadata(cx)? { return Ok(None); }
            let work = self.work.as_ref().ok_or_else(Self::unadmitted_input)?;
            self.checkpoint_capture = Some(ArtifactCommandCheckpointCursor::new(self.phase == ArtifactRetainedCommandPhase::Work, [
                self.raw_page_cursor as u64, self.raw.len() as u64, self.work_progress,
                self.context.as_ref().map_or(0, |context| context.identity_digest()), work.workspace_identity(),
            ]));
            self.pending_publication = Some(ArtifactCommandPublication::Checkpoint);
            return Ok(None);
        }
        let work = self.work.as_deref().ok_or_else(Self::unadmitted_input)?;
        let cursor = self.checkpoint_capture.as_mut().ok_or_else(Self::unadmitted_input)?;
        if !cursor.is_complete() {
            let step = cursor.advance_one(work, &mut self.checkpoint_target, |work, index| work.checkpoint_byte(index), cx.retained_grant());
            Self::receive_step(cx, step)?;
            return Ok(None);
        }
        let source = cursor.bytes(&self.checkpoint_target)?;
        let result = self.publication.advance_from_source(JobPublicationKind::Checkpoint { applied_progress: self.work_progress.max(self.raw_page_cursor as u64) }, source, cx)?;
        if result.is_some() { self.publication_delivered = true; }
        Ok(result)
    }

    fn advance_publication<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if let Some(fault) = self.fault.as_ref() {
            let result = self.fault_publication.advance_from_fault(fault, cx)?;
            if result.is_some() { self.publication_delivered = true; }
            return Ok(result);
        }
        match self.pending_publication {
            Some(ArtifactCommandPublication::Static { kind, bytes }) => {
                let result = self.publication.advance_from_source(kind, bytes, cx)?;
                if result.is_some() { self.publication_delivered = true; }
                Ok(result)
            }
            Some(ArtifactCommandPublication::Checkpoint) => self.checkpoint(cx),
            None => Ok(None),
        }
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
        self.raw_page_offset = 0;
        Ok(())
    }

    fn preview(&mut self, bytes: &'static [u8]) {
        self.pending_publication = Some(ArtifactCommandPublication::Static { kind: JobPublicationKind::Preview, bytes });
    }

    fn fault(&mut self, bytes: &'static [u8]) {
        self.phase = ArtifactRetainedCommandPhase::Fault;
        self.pending_publication = Some(ArtifactCommandPublication::Static { kind: JobPublicationKind::Fault, bytes });
    }

    fn reducer_fault(&mut self, fault: Fault) {
        self.phase = ArtifactRetainedCommandPhase::Fault;
        self.fault = Some(fault);
    }

    #[cfg(test)]
    pub(crate) fn test_pending_emit_shape(&self) -> Option<(usize, usize, Vec<String>)> {
        self.emit.as_ref().map(|emit| (emit.child_emits.len(), emit.artifact_mutations.len(), emit.child_emits.iter().map(|child| child.child_id.clone()).collect()))
    }
}

/// 🧯️ The bounded canonical Fault wire one refused reducer step reports. A normal fault crosses exactly;
/// an oversized one keeps its typed code, source span, and named params while prose is narrowed.
pub fn reducer_fault_detail(fault: &Fault) -> Vec<u8> {
    semio_framework_diagnostic::encode_fault_bytes_bounded(fault, ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES).unwrap_or_else(|| {
        semio_framework_diagnostic::encode_fault_bytes(&Fault::new(semio_framework::FaultOrigin::Framework, "retained-command.fault-capacity", "the reducer fault exceeded its bounded diagnostic carrier"))
    })
}

/// 🔎️ Recovers the exact typed reducer fault while leaving framework-authored plain fault pages distinct.
pub fn reducer_fault_of_detail(detail: &[u8]) -> Option<Fault> {
    semio_framework_diagnostic::try_decode_fault_bytes(detail)
}

impl<A: ArtifactApp> InteractiveJob for ArtifactRetainedCommandJob<A> {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if cx.is_cancelled() { return JobOutcomeBorrow::admit_cancelled(cx); }
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(None); }
        if self.publication_delivered { self.retire_publication(cx)?; return Ok(None); }
        if self.pending_publication.is_some() || self.fault.is_some() { return self.advance_publication(cx); }
        if self.checkpoint_pending { return self.checkpoint(cx); }
        match self.phase {
            ArtifactRetainedCommandPhase::CheckpointPages => {
                cx.set_stage("retained-command-checkpoint-page");
                let input = self.checkpoint_input.as_ref().ok_or_else(Self::unadmitted_input)?;
                if let Some(page) = input.page(self.checkpoint_page_cursor) {
                    let source = page.get(self.checkpoint_page_offset..).ok_or_else(Self::unadmitted_input)?;
                    if self.checkpoint_byte_len.checked_add(source.len()).is_none_or(|length| length > self.checkpoint_bytes.len()) {
                        if Self::metadata(cx)? { self.fault(b"retained command checkpoint exceeds capacity"); }
                        return Ok(None);
                    }
                    let bytes = source.len().min(cx.retained_grant().maximum_copy_bytes);
                    if bytes == 0 && !source.is_empty() { return Ok(None); }
                    let end = self.checkpoint_byte_len + bytes;
                    self.checkpoint_bytes[self.checkpoint_byte_len..end].copy_from_slice(&source[..bytes]);
                    self.checkpoint_byte_len = end;
                    self.checkpoint_page_offset += bytes;
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                    cx.consume_fuel(1);
                    if self.checkpoint_page_offset == page.len() {
                        self.checkpoint_page_cursor += 1;
                        self.checkpoint_page_offset = 0;
                        self.preview(b"{\"en\":\"Reading checkpoint\",\"de\":\"Pr\xC3\xBCfpunkt wird gelesen\"}");
                    }
                    return Ok(None);
                }
                if !Self::metadata(cx)? { return Ok(None); }
                match self.restore_checkpoint() {
                    Ok(()) => {
                        self.checkpoint_input.as_mut().ok_or_else(Self::unadmitted_input)?.begin_close();
                        self.phase = ArtifactRetainedCommandPhase::CheckpointRetire;
                        self.preview(br#"{"en":"Restoring command","de":"Befehl wird wiederhergestellt"}"#);
                    }
                    Err(fault) => self.reducer_fault(fault),
                }
            }
            ArtifactRetainedCommandPhase::CheckpointRetire => {
                let owner = self.checkpoint_input.as_mut().ok_or_else(Self::unadmitted_input)?;
                if !owner.terminal_is_empty() {
                    let grant = cx.retained_grant();
                    let step = owner.close_step(grant).admit(grant, owner.terminal_is_empty());
                    cx.consume_retained(step.progress())?;
                    if let InteractiveJobCloseStep::Refused { kind, progress } = step {
                        return Err(ValueError::literal(kind, "original checkpoint receiver retirement refused").with_retained_progress(progress));
                    }
                    return Ok(None);
                }
                if Self::metadata(cx)? {
                    self.checkpoint_input.take();
                    self.phase = ArtifactRetainedCommandPhase::WirePages;
                    self.checkpoint_pending = true;
                }
            }
            ArtifactRetainedCommandPhase::WirePages => {
                cx.set_stage("retained-command-wire-page");
                let input = self.raw_input.as_ref().ok_or_else(Self::unadmitted_input)?;
                if let Some(page) = input.page(self.raw_page_cursor) {
                    let source = page.get(self.raw_page_offset..).ok_or_else(Self::unadmitted_input)?;
                    if self.raw.len().checked_add(source.len()).is_none_or(|length| length > self.maximum_raw_bytes || length > self.raw.capacity()) {
                        if Self::metadata(cx)? { self.fault(b"retained command exceeds original raw capacity"); }
                        return Ok(None);
                    }
                    let bytes = source.len().min(cx.retained_grant().maximum_copy_bytes);
                    if bytes == 0 && !source.is_empty() { return Ok(None); }
                    self.raw.extend_from_slice(&source[..bytes]);
                    self.raw_page_offset += bytes;
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                    cx.consume_fuel(1);
                    if self.raw_page_offset == page.len() {
                        self.raw_page_cursor += 1;
                        self.raw_page_offset = 0;
                        self.checkpoint_pending = true;
                        self.preview(br#"{"en":"Reading command page","de":"Befehlsseite wird gelesen"}"#);
                    }
                    return Ok(None);
                }
                if Self::metadata(cx)? {
                    self.phase = ArtifactRetainedCommandPhase::Decode;
                    self.checkpoint_pending = true;
                }
            }
            ArtifactRetainedCommandPhase::Decode => {
                cx.set_stage("retained-command-decode");
                if !Self::metadata(cx)? { return Ok(None); }
                let decoded = match <A::Command as protocol::OpBinary>::decode_op(&self.raw) {
                    Ok(command) => command,
                    Err(error) => { self.decode_error=Some(error); self.fault(b"retained command wire payload is malformed"); return Ok(None); }
                };
                let work = self.work.as_ref().ok_or_else(Self::unadmitted_input)?;
                if (self.command_id)(&decoded) != work.tool_id() {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "received command does not match original app work authority"));
                }
                self.command = Some(decoded);
                self.phase = ArtifactRetainedCommandPhase::Preflight;
                self.preview(b"{\"en\":\"Validating command\",\"de\":\"Befehl wird gepr\xC3\xBCft\"}");
            }
            ArtifactRetainedCommandPhase::Preflight => {
                cx.set_stage("retained-command-preflight");
                if !Self::metadata(cx)? { return Ok(None); }
                let (Some(command), Some(snapshot), Some(interaction), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.interaction_state.as_ref(), self.work.as_ref()) else { return Err(Self::unadmitted_input()); };
                let Some(extent) = work.extent(command, snapshot, interaction, self.context.as_deref()) else {
                    self.fault(b"retained command work refused the command before any capacity was measured");
                    return Ok(None);
                };
                if extent == 0 || extent > self.maximum_work_items {
                    self.fault(b"retained command exceeds semantic work capacity");
                    return Ok(None);
                }
                self.phase = ArtifactRetainedCommandPhase::Work;
                self.preview(br#"{"en":"Applying command","de":"Befehl wird angewendet"}"#);
            }
            ArtifactRetainedCommandPhase::Work => {
                cx.set_stage("retained-command-work");
                let (Some(command), Some(snapshot), Some(config), Some(history), Some(interaction), Some(hover), Some(operation), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.config.as_ref(), self.history.as_ref(), self.interaction_state.as_ref(), self.interaction_hover.as_ref(), self.operation.as_ref(), self.work.as_mut()) else { return Err(Self::unadmitted_input()); };
                let before = cx.retained_progress();
                let fuel = cx.fuel_remaining();
                let step = work.step(&ArtifactCommandInputs { command, snapshot, snapshot_owner: Some(snapshot), config, history, interaction, hover, context: self.context.as_deref(), operation }, cx);
                if cx.retained_progress() == before { Self::metadata(cx)?; }
                if cx.fuel_remaining() == fuel { cx.consume_fuel(1); }
                match step {
                    Ok(ArtifactCommandWorkStep::Replay { stage, preview }) => { cx.set_stage(stage); self.checkpoint_pending = true; self.preview(preview); }
                    Ok(ArtifactCommandWorkStep::Progress { stage, preview }) => { cx.set_stage(stage); self.work_progress = self.work_progress.saturating_add(1); self.checkpoint_pending = true; self.preview(preview); }
                    Ok(ArtifactCommandWorkStep::CompleteDownload { download, ephemeral }) => {
                        self.download = Some(download); self.ephemeral = Some(ephemeral); self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(r#"{"en":"Publishing download","de":"Download wird veröffentlicht"}"#.as_bytes());
                    }
                    Ok(ArtifactCommandWorkStep::Complete(emit)) => {
                        self.emit = Some(emit); self.ephemeral = Some(EphemeralEmit::default()); self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(b"{\"en\":\"Publishing result\",\"de\":\"Ergebnis wird ver\xC3\xB6ffentlicht\"}");
                    }
                    Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral }) => {
                        self.emit = Some(emit); self.ephemeral = Some(ephemeral); self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(b"{\"en\":\"Publishing result\",\"de\":\"Ergebnis wird ver\xC3\xB6ffentlicht\"}");
                    }
                    Err(fault) => self.reducer_fault(fault),
                }
            }
            ArtifactRetainedCommandPhase::Publish => {
                cx.set_stage("retained-command-publish");
                if let Some(emit) = self.emit.as_mut() {
                    let step = emit.prepare_child_one(cx.retained_grant());
                    let progress = match &step { Ok(step) => step.progress(), Err(fault) => fault.retained_progress() };
                    cx.consume_retained(progress)?;
                    match step {
                        Ok(crate::app::ChildEmitPreparationStep::Ready(progress)) if progress == RetainedCloneProgress::default() => {}
                        Ok(crate::app::ChildEmitPreparationStep::Ready(_) | crate::app::ChildEmitPreparationStep::Pending(_)) => { cx.consume_fuel(1); return Ok(None); }
                        Ok(crate::app::ChildEmitPreparationStep::Refused(fault, _)) | Err(fault) => { self.reducer_fault(fault); return Ok(None); }
                    }
                }
                if !Self::metadata(cx)? { return Ok(None); }
                let completion = self.completion.as_ref().ok_or_else(Self::unadmitted_input)?;
                if !completion.has_mounted_consumer() { self.fault(b"retained command completion consumer is absent"); return Ok(None); }
                if let Some(download) = self.download.take() {
                    let Some(ephemeral) = self.ephemeral.take() else { self.download = Some(download); return Err(Self::unadmitted_input()); };
                    if let Err(rejected) = completion.complete_download(Ok(download), ephemeral) {
                        self.download = rejected.download.ok(); self.ephemeral = Some(rejected.ephemeral); self.reducer_fault(rejected.fault); return Ok(None);
                    }
                } else {
                    let emit = self.emit.take().ok_or_else(Self::unadmitted_input)?;
                    let Some(ephemeral) = self.ephemeral.take() else { self.emit = Some(emit); return Err(Self::unadmitted_input()); };
                    if let Err(rejected) = completion.complete(Ok(emit), ephemeral) {
                        self.emit = rejected.emit.ok(); self.ephemeral = Some(rejected.ephemeral); self.reducer_fault(rejected.fault); return Ok(None);
                    }
                }
                self.phase = ArtifactRetainedCommandPhase::Complete;
            }
            ArtifactRetainedCommandPhase::Complete => return JobOutcomeBorrow::admit_complete(cx, None, None),
            ArtifactRetainedCommandPhase::Fault => {
                if Self::metadata(cx)? { self.fault(self.admission_refusal.map(ArtifactRetainedAdmissionRefusal::detail).unwrap_or(b"retained command remains faulted")); }
            }
        }
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.phase == ArtifactRetainedCommandPhase::Complete => descriptor.complete(None, None),
            JobOutcomeKind::Fault if self.fault.is_some() => self.fault_publication.borrow_outcome(descriptor),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(Self::unadmitted_input()),
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

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep {
        if !self.closing{return InteractiveJobCloseStep::Blocked;}
        if grant.maximum_items==0{return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()};}
        let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};
        if grant.maximum_depth<demand.depth{return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::DepthLimit,progress:Default::default()};}
        if !self.publication.terminal_is_empty(){return Self::controlled_close_step(self.publication.close_step(grant));}
        if !self.fault_publication.terminal_is_empty(){return Self::controlled_close_step(self.fault_publication.close_step(grant));}
        if let Some(cursor)=self.checkpoint_capture.as_mut().filter(|cursor|!cursor.terminal_is_empty()){return Self::controlled_close_step(cursor.close_step(grant));}
        if self.checkpoint_capture.is_some()||self.pending_publication.is_some()||self.publication_delivered{self.checkpoint_capture.take();self.pending_publication=None;self.publication_delivered=false;self.checkpoint_pending=false;return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
        if let Some(fault)=self.fault.take(){match ControlledRetirement::new(fault){Ok(owner)=>self.fault_retirement=Some(owner),Err((error,original))=>{self.fault=Some(original);return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()};}}return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
        if let Some(owner)=self.fault_retirement.as_mut(){let step=owner.step(grant);if owner.terminal_is_empty(){self.fault_retirement.take();}return Self::controlled_close_step(step);}
        if self.decode_error.is_some(){return Self::controlled_close_step(protocol::close_protocol_error_one(&mut self.decode_error,grant));}
        if self.admission_refusal.take().is_some(){return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
        if !self.raw.is_empty(){let bytes=self.raw.len().min(grant.maximum_copy_bytes);if bytes==0{return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()};}self.raw.truncate(self.raw.len()-bytes);return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}};}
        if self.raw.capacity()!=0{let bytes=self.raw.capacity();if grant.maximum_release_bytes<bytes{return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()};}drop(std::mem::take(&mut self.raw));return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}};}
        for input in [&mut self.checkpoint_input,&mut self.raw_input]{if let Some(owner)=input.as_mut(){let step=owner.close_step(grant);if owner.terminal_is_empty(){input.take();}return match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step};}}
        if let Some(download)=self.download.take(){match ControlledRetirement::new(download){Ok(owner)=>self.download_retirement=Some(owner),Err((error,original))=>{self.download=Some(original);return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()};}}return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
        if let Some(owner)=self.download_retirement.as_mut(){let step=owner.step(grant);if owner.terminal_is_empty(){self.download_retirement.take();}return Self::controlled_close_step(step);}
        if let Some(emit)=self.emit.as_mut(){
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            return match emit.close_child_one(child){
                Ok(Some(crate::app::PluginLifecycleStep::Progress(progress)|crate::app::PluginLifecycleStep::Complete(progress))) if progress.fits(child)=>InteractiveJobCloseStep::Pending{progress},
                Ok(Some(crate::app::PluginLifecycleStep::Blocked{..}|crate::app::PluginLifecycleStep::AwaitingInput{..}))=>InteractiveJobCloseStep::Blocked,
                Ok(Some(_))=>InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()},
                Ok(None)|Err(_)=>InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::UnsupportedOwner,progress:Default::default()},
            };
        }
        if self.ephemeral.is_some(){return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::UnsupportedOwner,progress:Default::default()};}
        if let Some(work)=self.work.as_mut(){if !work.terminal_is_empty(){return work.close_step(grant);}let Some(bytes)=work.terminal_frame_release_bytes()else{return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::UnsupportedOwner,progress:Default::default()};};if grant.maximum_release_bytes<bytes{return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()};}self.work.take();return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}};}
        if self.command.is_some()||self.snapshot.is_some()||self.config.is_some()||self.history.is_some()||self.interaction_state.is_some()||self.interaction_hover.is_some()||self.context.is_some()||self.operation.is_some(){return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::UnsupportedOwner,progress:Default::default()};}
        if let Some(completion)=self.completion.take(){match ControlledRetirement::new(completion){Ok(owner)=>self.completion_retirement=Some(owner),Err((error,original))=>{self.completion=Some(original);return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()};}}return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
        if let Some(owner)=self.completion_retirement.as_mut(){let step=owner.step(grant);if owner.terminal_is_empty(){self.completion_retirement.take();}return Self::controlled_close_step(step);}
        InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()}
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.publication.terminal_is_empty()
            && self.fault_publication.terminal_is_empty()
            && self.checkpoint_capture.is_none()
            && self.pending_publication.is_none()
            && !self.publication_delivered
            && self.fault.is_none()
            && self.fault_retirement.is_none()
            && self.decode_error.is_none()
            && self.admission_refusal.is_none()
            && self.raw.is_empty()
            && self.raw.capacity() == 0
            && self.checkpoint_input.is_none()
            && self.raw_input.is_none()
            && self.emit.is_none()
            && self.download.is_none()
            && self.download_retirement.is_none()
            && self.ephemeral.is_none()
            && self.work.is_none()
            && self.command.is_none()
            && self.snapshot.is_none()
            && self.config.is_none()
            && self.history.is_none()
            && self.interaction_state.is_none()
            && self.interaction_hover.is_none()
            && self.context.is_none()
            && self.context_retirement.is_none()
            && self.operation.is_none()
            && self.completion.is_none()
            && self.completion_retirement.is_none()
    }
}
//#endregion 🧵️Job

#[cfg(test)]
pub(crate) fn test_raw_allocation_close<A: ArtifactApp>() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🚪️raw-allocation-close.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut raw = Vec::with_capacity(case["capacity"].as_u64().unwrap() as usize);
        raw.resize(case["initializedBytes"].as_u64().unwrap() as usize, 42);
        let mut job = ArtifactRetainedCommandJob::<A> {
            command: None,
            snapshot: None,
            config: None,
            history: None,
            interaction_state: None,
            interaction_hover: None,
            context: None,
            operation: None,
            completion: None,
            command_id: |_| "fixture",
            maximum_raw_bytes: raw.capacity(),
            maximum_work_items: 1,
            work: None,
            checkpoint_input: None,
            checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_target: [MaybeUninit::uninit(); ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_capture: None,
            publication: RetainedJobPublication::new(),
            fault_publication: RetainedFaultPublication::new(),
            pending_publication: None,
            publication_delivered: false,
            fault: None,
            fault_retirement: None,
            decode_error: None,
            checkpoint_byte_len: 0,
            checkpoint_page_cursor: 0,
            checkpoint_page_offset: 0,
            raw_input: None,
            raw,
            admission_refusal:None,
            raw_page_cursor: 0,
            raw_page_offset: 0,
            emit: None,
            download:None,
            download_retirement:None,
            completion_retirement:None,
            context_retirement:None,
            ephemeral: None,
            phase: ArtifactRetainedCommandPhase::Complete,
            checkpoint_pending: false,
            work_progress: 0,
            closing: false,
        };
        job.begin_close();
        let capacity=job.raw.capacity();
        let pointer=job.raw.as_ptr();
        let expected:Vec<u8>=serde_json::from_str(&serde_json::to_string(&vec![42u8;case["initializedBytes"].as_u64().unwrap() as usize]).unwrap()).unwrap();
        assert_eq!(job.raw,expected);
        assert_eq!(job.next_close_copy_byte_demand().unwrap(),0);
        let unfunded=RetainedCloneGrant{maximum_items:0,maximum_depth:1,..Default::default()};
        assert!(matches!(job.close_step(unfunded),InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress} if progress==RetainedCloneProgress::default()));
        assert_eq!(job.raw.as_ptr(),pointer);assert_eq!(job.raw.capacity(),capacity);
        let mut copied=0;let mut released=0;
        for _ in 0..16 {
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixture["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:0,maximum_release_bytes:job.next_close_release_byte_demand().unwrap(),maximum_depth:job.next_close_depth_demand().unwrap()};
            let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant));
            let progress=match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>progress,_=>panic!("funded original raw allocation must complete")};
            assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,progress.released_bytes);
            assert!(progress.copied_items<=grant.maximum_items&&progress.copied_bytes<=grant.maximum_copy_bytes&&progress.released_bytes<=grant.maximum_release_bytes);
            copied+=progress.copied_bytes;released+=progress.released_bytes;
            if job.terminal_is_empty(){break;}
        }
        assert!(job.terminal_is_empty());
        assert_eq!(copied,case["expectedCopiedBytes"].as_u64().unwrap() as usize);
        assert_eq!(released,case["expectedReleasedBytes"].as_u64().unwrap() as usize);
        assert_eq!(released,capacity);
        eprintln!("[DEBUG] retained command raw {} copied={copied} physically-released={released}",case["id"]);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
                                     
