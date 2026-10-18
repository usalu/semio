//! 🛰️ Owned operation and guest instance: the owned operation and stage, the pending record, the checkpoint metadata and view and the owned guest instance. See [`super`].

use super::*;

//#region 🧠️OwnedRuntime
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
pub(super) enum OwnedOperation {
    Describe,
    ChannelVersion,
    Poll,
    AdmitActorContext,
    CloseActorContext,
    StartJob,
    StepJob,
    ReadJobOutput,
    AcknowledgeJobOutput,
    CancelJob,
    CloseJobs,
    Checkpoint,
    Restore,
    PackSchemaHash,
    Genesis,
    PrintMirror,
    ApplyOps,
    ReplayEnvelopes,
    SqliteSchema,
    SqliteExport,
    SqliteImport,
    SqliteRetirement,
    SqliteClose,
    SqliteTakeFile,
    SqliteTakePayload,
}

impl OwnedOperation {
    pub(super) fn export(self) -> OwnedSemioExport {
        match self {
            Self::Describe => OwnedSemioExport::Describe,
            Self::ChannelVersion => OwnedSemioExport::ChannelVersion,
            Self::Poll => OwnedSemioExport::Poll,
            Self::AdmitActorContext=>OwnedSemioExport::AdmitActorContext,
            Self::CloseActorContext=>OwnedSemioExport::CloseActorContext,
            Self::StartJob => OwnedSemioExport::StartJob,
            Self::StepJob => OwnedSemioExport::StepJob,
            Self::ReadJobOutput => OwnedSemioExport::ReadJobOutput,
            Self::AcknowledgeJobOutput => OwnedSemioExport::AcknowledgeJobOutput,
            Self::CancelJob => OwnedSemioExport::CancelJob,
            Self::CloseJobs => OwnedSemioExport::CloseJobs,
            Self::Checkpoint => OwnedSemioExport::Checkpoint,
            Self::Restore => OwnedSemioExport::Restore,
            Self::PackSchemaHash => OwnedSemioExport::PackSchemaHash,
            Self::Genesis => OwnedSemioExport::Genesis,
            Self::PrintMirror => OwnedSemioExport::PrintMirror,
            Self::ApplyOps => OwnedSemioExport::ApplyOps,
            Self::ReplayEnvelopes => OwnedSemioExport::ReplayEnvelopes,
            Self::SqliteSchema => OwnedSemioExport::SqliteSchema,
            Self::SqliteExport => OwnedSemioExport::SqliteExport,
            Self::SqliteImport => OwnedSemioExport::SqliteImport,
            Self::SqliteRetirement => OwnedSemioExport::SqliteRetirement,
            Self::SqliteClose => OwnedSemioExport::SqliteClose,
            Self::SqliteTakeFile => OwnedSemioExport::SqliteTakeFile,
            Self::SqliteTakePayload => OwnedSemioExport::SqliteTakePayload,

        }
    }
}

#[derive(Clone, Debug, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
pub(super) enum OwnedStage {
    Allocate { input: Vec<u8> },
    Call,
    Deallocate { output: Vec<u8> },
}

#[derive(Clone, Debug, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
pub(super) struct OwnedPending {
    pub(super) retained_input: Option<Vec<u8>>,
    pub(super) operation: OwnedOperation,
    pub(super) stage: OwnedStage,
    pub(super) fuel_used: u64,
}

#[derive(Debug, serde::Deserialize, FromValue)]
pub(super) struct OwnedCheckpointMetadata {
    pub(super) pending: Option<OwnedPending>,
    pub(super) original_receipt: Option<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceipt>,
    pub(super) guest_checkpoint: Option<Vec<u8>>,
    pub(super) context: i32,
    pub(super) next_resource: i32,
    pub(super) instance_id: u32,
}

#[derive(serde::Serialize)]
pub(super) struct OwnedCheckpointView<'a>{pub(super) pending:&'a Option<OwnedPending>,pub(super) original_receipt:&'a Option<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceipt>,pub(super) guest_checkpoint:&'a Option<Vec<u8>>,pub(super) context:i32,pub(super) next_resource:i32,pub(super) instance_id:u32}

/// 🏃️ One instance of an owned compiled guest: its interpreter state, the operation it has in flight, the original receipt
/// that operation keeps, and what the guest printed.
pub struct OwnedGuestInstance {
    pub(super) original_job_clock:Option<(u64,fn()->Option<u64>)>,
    pub(super) artifact: Arc<OwnedSemioArtifact>,
    pub(super) actor: Option<OwnedSemioInstance>,
    pub(super) original_core:std::mem::ManuallyDrop<Option<crate::interpreter::CoreInstanceOpening>>,
    pub(super) original_source:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub(super) original_startup_input:Option<semio_framework_actor::RetainedTurnInput>,
    pub(super) original_call:Option<original_call::OwnedOriginalCall>,
    pub(super) pending: Option<OwnedPending>,
    pub(super) original_receipt: Option<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceipt>,
    /// ☠️ Set by the first guest trap. A trapped guest keeps whatever linear memory, shadow-stack
    /// pointer and allocator state the trap left behind, so every later call starts from a lower
    /// stack pointer and traps again 112 bytes further down — ticket 26/09/18 slice A1 read that
    /// drift as the defect itself. There is no recovery short of re-instantiation, so this refuses.
    pub(super) poisoned: bool,
    /// 🩺️ Everything this guest has written to `wasi:cli/stdout`/`stderr`, bounded by
    /// [`GUEST_DIAGNOSTICS_CAPACITY_BYTES`] — see [`retain_owned_guest_diagnostics`]. Unlike the
    /// wasmtime path this is ALWAYS armed: the owned interpreter serves the stream shim itself at
    /// no cost, and a guest panic message that is thrown away is a whole investigation.
    pub(super) diagnostics: Vec<u8>,
    pub(super) context: i32,
    pub(super) next_resource: i32,
    pub(super) instance_id: u32,
}

impl Drop for OwnedGuestInstance{fn drop(&mut self){assert!(std::thread::panicking()||(self.original_core.is_none()&&self.original_source.is_none()),"Owned original instance reached Drop before its paid Core and source close")}}

impl OwnedGuestInstance {
    pub fn original_core_mut(&mut self)->Result<(&Arc<crate::interpreter::CoreModule>,&mut crate::interpreter::CoreInstanceOpening),semio_framework_value::ValueError>{let core=self.original_core.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"Owned call requires its genuine paid original Core holder"))?;Ok((self.artifact.original_actor_module(),core))}
    /// 👁️ Borrows already funded original Core memory without copying or reopening its owner.
    pub fn original_memory(&self,index:u32)->Option<&[u8]>{self.original_core.as_ref()?.resources()?.memory(index)}
    pub fn original_startup_input(&self)->Option<semio_framework_actor::RetainedTurnInput>{self.original_startup_input}

    /// 🕰️ Forwards the original caller clock without inventing an interpreter-local epoch.
    pub fn bind_original_clock(&mut self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_value::ValueError>{cx.original_source_parent()?;let binding=cx.original_clock_binding().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original owned clock requires its issuer binding"))?;self.original_job_clock=Some((binding,cx.clock_source()));Ok(())}
    pub fn bind_original_clock_from_source(&mut self,source:&semio_framework_job::OriginalCallerSourceLoan)->Result<(),semio_framework_value::ValueError>{let(binding,clock,_)=source.original_clock_source()?;self.original_job_clock=Some((binding,clock));Ok(())}
    pub fn original_clock_observation(&self,binding:u64)->Option<u64>{self.original_job_clock.filter(|(original,_)|*original==binding).and_then(|(_,clock)|clock())}
    pub fn release_original_clock(&mut self,binding:u64)->bool{if self.original_job_clock.is_some_and(|original|original.0==binding){self.original_job_clock=None;true}else{false}}

    /// 🔁️ True while an owned operation has not completed — the guest yielded on its fuel or deadline and owns the host's next
    /// call.
    pub fn turn_in_flight(&self) -> bool {
        self.pending.is_some()
    }

    /// ☠️ True once a guest trap left this instance unusable; every later call refuses.
    pub fn poisoned(&self) -> bool {
        self.poisoned
    }

    /// 🩺️ Everything this guest has written to `wasi:cli/stdout`/`stderr`, bounded by [`GUEST_DIAGNOSTICS_CAPACITY_BYTES`].
    pub fn diagnostics(&self) -> &[u8] {
        &self.diagnostics
    }

    /// ⏸️ The snapshot operation this instance has in flight and the fuel it ran so far; refused when another owned operation
    /// is in flight.
    pub fn pending_snapshot(&self) -> Result<Option<(OwnedSnapshotOperation, u64)>, TurnFault> {
        self.pending.as_ref().map(|pending| OwnedSnapshotOperation::from_operation(pending.operation).map(|operation| (operation, pending.fuel_used)).ok_or_else(|| TurnFault::Trapped(format!("owned operation {:?} is not a snapshot operation", pending.operation)))).transpose()
    }

    /// 🌱️ Begins `operation` on the input bytes `input` holds, taking them only once the begin is admitted.
    pub fn begin_snapshot(&mut self, operation: OwnedSnapshotOperation, input: &mut Option<Vec<u8>>) -> Result<(), TurnFault> {
        if self.original_core.is_some(){return Err(identity_turn_fault(operation_authority::refusal(4)))}
        if self.poisoned {
            return Err(TurnFault::Trapped("owned snapshot instance is poisoned".into()));
        }
        if self.pending.is_some() {
            return Err(TurnFault::Trapped("owned snapshot operation remains pending".into()));
        }
        let bytes = input.as_ref().ok_or_else(|| identity_turn_fault(operation_authority::refusal(4)))?;
        let length = i32::try_from(bytes.len()).map_err(|_| identity_turn_fault(operation_authority::refusal(2)))?;
        self.pending = Some(OwnedPending { retained_input: None, operation: operation.operation(), stage: OwnedStage::Allocate { input: input.take().unwrap_or_default() }, fuel_used: 0 });
        self.actor.as_mut().ok_or_else(||identity_turn_fault(operation_authority::refusal(4)))?.begin(OwnedSemioExport::Allocate, vec![Value::I32(length)])?;
        Ok(())
    }

    /// ▶️ Resumes the same operation within its supplied absolute original deadline and receiving authority.
    pub fn resume_snapshot(&mut self, operation: OwnedSnapshotOperation, fuel: u64, deadline_us: u64, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>, receiver: &mut dyn OriginalOperationReceiving) -> Result<OwnedInvocation, TurnFault> {
        resume_owned_operation_bound(self, operation.operation(), fuel, deadline_us, OwnedDeadline::Original, progress, cancellation, Some(receiver))
    }
}
//#endregion 🧠️OwnedRuntime
