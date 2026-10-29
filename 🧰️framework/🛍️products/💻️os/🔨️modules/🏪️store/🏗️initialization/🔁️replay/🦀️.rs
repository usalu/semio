//! 🔁️ The framework-default retained store initializer: a ledger replay under the grant-based retained-clone contract.

#[path = "../🌱️genesis/🏭️factory/🧬️schema/🦀️.rs"]
pub mod original_store_genesis_factory;
#[cfg(test)]
#[path = "../📚️catalog/🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
#[path = "../🤝️authority/🦀️.rs"]
pub mod initialization_authority;
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_framework_os_spr::{ActorId, Edit, Mutation, MutationId, OpBinary, OpText};
use semio_framework_os_store_space_schema::ArtifactEnvelope;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, FromValue, ToValue};
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use crate::initialization_authority::ArtifactStoreInitializationAuthority;
use semio_framework_os_store_initialization_runtime::lifted_store::{ArtifactStoreInitializationEditAdmission, ArtifactStoreInitializationEditIndex, ArtifactStoreInitializationOwnerCatalog, ArtifactStoreInitializationRuntime};
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_store_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands};
use semio_framework_os_store_owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
use semio_framework_job::{Generation, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, OperationId, StepContext};
use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{controlled::ControlledRetirement, RetireOwned};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::{size_of, size_of_val, ManuallyDrop};
use std::sync::Arc;
use semio_framework_os_store_engine_owners::producer::DocumentStoreOwnerProducer;
use std::marker::PhantomData;
pub use semio_framework_os_store_initialization_runtime::lifted_initialization_replay::{ArtifactStoreInitializationNext};
#[path="🧬️native/🦀️.rs"]
mod native;
use semio_framework_os_store_ledger::native_digest;

/// 📏️ The widest id or encoded operation one replay turn admits, the page every retained store initializer shares.
pub const ARTIFACT_STORE_REPLAY_FIELD_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

const ITEM: RetainedCloneProgress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: 0 };





#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    AwaitEnvelope,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    AdmitOwners,
    AdmitCatalog,
    BindGenesis,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    CountApplied { position: usize },
    FindApplied { position: usize },
    Fold { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    RetireIndex,
    BuildCandidate,
    Complete,
    RetireCancelled,
    RetireFault,
    Cancelled,
    Fault,
}

enum Turn {
    Yield,
    Complete,
    Cancelled,
    Fault,
}

pub fn permits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items > 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "replay initializer retained child depth overflow"))?;
    Ok(demand)
}

/// 🔁️ Validates a completed envelope, folds its original ledger inputs through the installed native semantic producer,
/// and adopts the resulting Store under the initiating caller's exact construction and retirement grants.
/// Every partial snapshot read, operation source, replacement receiver and producer remains owned until funded closure.
/// 🔗️ [`ArtifactStoreInitializationRuntime`] · [`ArtifactStore::from_initialized_runtime_with_owners`]
pub struct ArtifactStoreReplayInitializer<P, M, C>
where
    P: RetireOwned + semio_framework_os_spr::StoreState,
    M: RetireOwned + semio_framework_pack_json::ArtifactCanonicalJsonTree + semio_framework_os_spr::StoreMutationFor<P>,
    C:DocumentStoreOwnerProducer<P,M>,
{
    pub producer:PhantomData<fn()->C>,
    native:native::NativeInitializationFold<P,M>,
    supersession_native:native_supersession::NativeSupersessionFold,
    digest:native_digest::NativeEditDigest,
    effective_digest:native_digest::NativeEffectiveDigest,
    commit_ready:bool,
    pub operation: OperationId,
    pub generation: Generation,
    pub schema: &'static str,
    pub envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    pub owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    pub catalog: ManuallyDrop<Option<ArtifactStoreInitializationOwnerCatalog>>,
    pub registry_birth:ManuallyDrop<semio_framework_value_snapshot_read::snapshot_read_registry_handle::birth::SnapshotReadRegistryBirth>,
    pub runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    pub factory: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>,
    pub candidate: ManuallyDrop<Option<Box<ArtifactStore<P, M>>>>,
    pub active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub envelope_retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub actor: ManuallyDrop<Option<ActorId>>,
    pub actor_close: ManuallyDrop<Option<ControlledRetirement<ActorId>>>,
    pub edit_index: ArtifactStoreInitializationEditIndex,
    pub index_open: bool,
    pub publication: RetainedJobPublication,
    pub phase: Phase,
    pub cancel_requested: bool,
    pub fault: Option<String>,
    pub original_failure:Option<semio_framework_job::OriginalTurnFailure>,
    fault_detail: Option<String>,
    pub folded: u64,
    pub total: u64,
    terminal_handoff: bool,
}

impl<P, M, C> ArtifactStoreReplayInitializer<P, M, C>
where
    P: RetireOwned + semio_framework_os_spr::StoreState,
    M: RetireOwned + semio_framework_pack_json::ArtifactCanonicalJsonTree + semio_framework_os_spr::StoreMutationFor<P>,
    C:DocumentStoreOwnerProducer<P,M>,
{
    /// 🌱️ Captures the completed `envelope` of a document of `schema` for the replay `operation` of `generation`, owned by `actor`.
    pub fn new(envelope: ArtifactEnvelope<P, M>, schema: &'static str, operation: OperationId, generation: Generation, actor: ActorId) -> Self {
        let mut owner=Self::awaiting_envelope(schema,operation,generation,actor);
        owner.envelope=ManuallyDrop::new(Some(envelope));
        owner.phase=Phase::ValidateEnvelope;
        owner
    }

    /// 🧳️ Retains the real actor and initializer authority before its original envelope is constructed.
    fn awaiting_envelope(schema:&'static str,operation:OperationId,generation:Generation,actor:ActorId)->Self{
        Self {
            producer:PhantomData,
            native:Default::default(),
            supersession_native:Default::default(),digest:Default::default(),effective_digest:Default::default(),commit_ready:false,
            operation,
            generation,
            schema,
            envelope: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(None),
            catalog: ManuallyDrop::new(None),
            registry_birth:ManuallyDrop::new(semio_framework_value_snapshot_read::snapshot_read_registry_handle::birth::SnapshotReadRegistryBirth::new()),
            runtime: ManuallyDrop::new(None),
            factory: None,
            candidate: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            envelope_retirement: ManuallyDrop::new(None),
            actor: ManuallyDrop::new(Some(actor)),
            actor_close: ManuallyDrop::new(None),
            edit_index: ArtifactStoreInitializationEditIndex::default(),
            index_open: false,
            publication: RetainedJobPublication::new(),
            phase: Phase::AwaitEnvelope,
            cancel_requested: false,
            fault: None,
            original_failure:None,
            fault_detail: None,
            folded: 0,
            total: 0,
            terminal_handoff: false,
        }
    }

    /// 📏️ Prices writing one fresh fixed initializer header directly into its actual allocation.
    pub fn awaiting_envelope_birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<Option<ActorId>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}

    /// 🌱️ Admits the original empty envelope recipient before payload construction under the same caller.
    pub fn admit_awaiting_envelope(actor:&mut Option<ActorId>,schema:&'static str,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if actor.is_none()||output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original initializer birth requires its actor and empty output").into())}
        let d=Self::awaiting_envelope_birth_demands();let operation=cx.operation();let generation=cx.generation();
        semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:d.depth},|_|{
            let storage=Box::<Self>::new_uninit();
            let owner=Box::write(storage,Self::awaiting_envelope(schema,operation,generation,actor.take().unwrap()));
            Ok((owner,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
        })?;Ok(output.is_some())
    }

    /// 🧷️ Borrows the explicit original envelope admission frontier.
    pub fn is_awaiting_envelope(&self)->bool{self.phase==Phase::AwaitEnvelope}

    /// 📥️ Transfers the already admitted original envelope without allocating or reconstructing any owner.
    pub fn receive_original_envelope(&mut self,envelope:ArtifactEnvelope<P,M>)->Result<(),ArtifactEnvelope<P,M>>{
        if self.phase!=Phase::AwaitEnvelope||self.cancel_requested||self.envelope.is_some(){return Err(envelope)}
        *self.envelope=Some(envelope);self.phase=Phase::ValidateEnvelope;Ok(())
    }

    fn running(&self) -> bool {
        !matches!(self.phase, Phase::Complete | Phase::RetireCancelled | Phase::RetireFault | Phase::Cancelled | Phase::Fault)
    }

    pub fn fail(&mut self, reason: impl Into<String>) {
        self.fault.get_or_insert_with(|| reason.into());
        self.phase = Phase::RetireFault;
    }

    fn spend(&mut self, cx: &mut StepContext<'_>, progress: RetainedCloneProgress) -> bool {
        match cx.consume_retained(progress) {
            Ok(()) => {
                cx.consume_fuel(1);
                true
            }
            Err(error) => {
                self.fail(error.into_message());
                false
            }
        }
    }

    pub fn refuse(&mut self, cx: &mut StepContext<'_>, error: ValueError) -> Turn {
        let _ = cx.consume_retained(error.retained_progress());
        self.fail(error.into_message());
        Turn::Yield
    }

    pub fn tick(&mut self, cx: &mut StepContext<'_>) -> Turn {
        self.spend(cx, ITEM);
        Turn::Yield
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn settle(&mut self, cx: &mut StepContext<'_>) -> bool {
        let Some(runtime) = self.runtime.as_mut() else { return false };
        match runtime.settle_current_retirement_step(cx.retained_grant()) {
            Ok(RetainedCloneStep::Complete(progress)) if progress == RetainedCloneProgress::default() => false,
            Ok(step) => {
                self.spend(cx, step.progress());
                true
            }
            Err(error) => {
                self.refuse(cx, error);
                true
            }
        }
    }

    pub fn turn(&mut self, cx: &mut StepContext<'_>) -> Turn {
        let grant = cx.retained_grant();
        let stale = cx.operation() != self.operation || cx.generation() != self.generation || cx.is_cancelled();
        let unclaimed = self.phase == Phase::Complete && !self.terminal_handoff && self.candidate.is_some();
        let closing = unclaimed && (self.cancel_requested || stale);
        if (!matches!(self.phase, Phase::Complete | Phase::Cancelled | Phase::Fault) || closing) && (cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0) {
            return Turn::Yield;
        }
        if stale {
            self.cancel_requested = true;
        }
        if self.cancel_requested && (self.running() || unclaimed) {
            self.phase = Phase::RetireCancelled;
        }
        if self.running() && !self.native.active() && self.settle(cx) {
            return Turn::Yield;
        }
        match self.phase {
            Phase::AwaitEnvelope=>Turn::Yield,
            Phase::ValidateEnvelope => {
                let valid = self.envelope.as_ref().is_some_and(|envelope| envelope.schema == self.schema && !envelope.id.is_empty() && envelope.id.len() <= ARTIFACT_STORE_REPLAY_FIELD_BYTES);
                if valid {
                    self.phase = Phase::ValidateEdit { index: 0 };
                } else {
                    self.fail("replay.envelope-invalid");
                }
                self.tick(cx)
            }
            Phase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated replay envelope remains retained");
                let demand=match self.edit_index.admission_demands(&envelope.vcs.edits,index,ARTIFACT_STORE_REPLAY_FIELD_BYTES){Ok(value)=>value,Err(error)=>return self.refuse(cx,error)};let mut result=None;
                let admitted=semio_framework_job::admit_original_construction(cx,&mut result,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|self.edit_index.admit(&envelope.vcs.edits,index,ARTIFACT_STORE_REPLAY_FIELD_BYTES,grant));
                if let Err(error)=admitted{self.index_open=true;self.original_failure=Some(error);self.phase=Phase::RetireFault;return Turn::Yield}let Some(admission)=result else{return Turn::Yield};self.index_open=true;
                match admission {
                    ArtifactStoreInitializationEditAdmission::Pending=>return Turn::Yield,
                    ArtifactStoreInitializationEditAdmission::Complete => self.phase = Phase::AdmitOwners,
                    ArtifactStoreInitializationEditAdmission::Admitted => {
                        self.index_open = true;
                        self.phase = Phase::ValidateEdit { index: index + 1 };
                    }
                    ArtifactStoreInitializationEditAdmission::Oversized | ArtifactStoreInitializationEditAdmission::Duplicate => self.fail("replay.duplicate-or-hostile-edit"),
                }
                Turn::Yield
            }
            Phase::AdmitOwners => self.admit_owners(cx, grant),
            Phase::AdmitCatalog => self.admit_catalog(cx, grant),
            Phase::BindGenesis => self.bind_genesis(cx),
            Phase::SeedHistory { edit, lane, index } => self.seed_history(cx, edit, lane, index),
            Phase::FoldSupersessions { transition } => {
                let envelope=self.envelope.as_ref().expect("original supersession envelope");let runtime=self.runtime.as_mut().expect("original supersession runtime");let source=envelope.transitions.get(transition).map(|event|native_supersession::NativeSupersessionSource{event,previous:transition.checked_sub(1).and_then(|at|envelope.transitions.get(at)),document:&envelope.id,line:envelope.active_alternative_id.as_deref()});
                match self.supersession_native.advance(source,&mut runtime.supersessions,cx){Ok(Some(true))=>self.phase=Phase::FoldSupersessions{transition:transition+1},Ok(Some(false))=>self.phase=Phase::CountApplied{position:0},Ok(None)=>{},Err(error)=>{self.original_failure=Some(error);self.phase=Phase::RetireFault;}}Turn::Yield
            }
            Phase::CountApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    self.phase = Phase::FindApplied { position: 0 };
                    return self.tick(cx);
                };
                let located = self.edit_index.position(id).and_then(|at| self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(at)).filter(|edit| edit.id == id)).map(|edit| edit.forwards.len());
                match located {
                    Some(forwards) => {
                        self.total += forwards as u64;
                        self.phase = Phase::CountApplied { position: position + 1 };
                    }
                    None => self.fail("replay.applied-edit-missing"),
                }
                self.tick(cx)
            }
            Phase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint=self.envelope.as_ref().and_then(|envelope|envelope.cursor.as_ref().and_then(|cursor|cursor.checkpoint_id.as_deref()).or_else(||envelope.vcs.checkpoints.last().map(|checkpoint|checkpoint.id.as_str())));let length=checkpoint.map_or(0,str::len);let copy=length+size_of::<Option<String>>()+size_of::<Phase>();let mut output=None;
                    let result=semio_framework_job::admit_original_construction(cx,&mut output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:length,release_bytes:0,depth:1},|_|Ok((checkpoint.map(str::to_owned),RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:length,..Default::default()})));
                    match result{Ok(_)=>{if let Some(checkpoint)=output{self.runtime.as_mut().expect("original runtime").set_current_checkpoint_id(checkpoint);self.phase=Phase::FindRedo{position:0};}},Err(error)=>{self.original_failure=Some(error);self.phase=Phase::RetireFault;}}return Turn::Yield;
                };
                let edit = self.edit_index.position(id).filter(|at| self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(*at)).is_some_and(|edit| edit.id == id));
                match edit {
                    Some(edit) => self.phase = Phase::Fold { position, edit, mutation: 0 },
                    None => self.fail("replay.applied-edit-missing"),
                }
                self.tick(cx)
            }
            Phase::Fold { position, edit, mutation } => self.fold(cx, grant, position, edit, mutation),
            Phase::CommitApplied { position, edit } => self.commit(cx,position,edit,false),
            Phase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = Phase::RetireIndex;
                    return self.tick(cx);
                };
                let edit = self.edit_index.position(id).filter(|at| self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(*at)).is_some_and(|edit| edit.id == id));
                match edit {
                    Some(edit) => self.phase = Phase::CommitRedo { position, edit },
                    None => self.fail("replay.redo-edit-missing"),
                }
                self.tick(cx)
            }
            Phase::CommitRedo { position, edit } => self.commit(cx,position,edit,true),
            Phase::RetireIndex => {
                match self.edit_index.retire_step(grant){Ok(step)=>{self.spend(cx,step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.index_open=false;self.phase=Phase::BuildCandidate;}Turn::Yield},Err(error)=>self.refuse(cx,error)}
            }
            Phase::BuildCandidate => self.build_candidate(cx),
            Phase::Complete => Turn::Complete,
            Phase::RetireCancelled | Phase::RetireFault => self.retire(cx),
            Phase::Cancelled => Turn::Cancelled,
            Phase::Fault => Turn::Fault,
        }
    }

    fn admit_owners(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Turn {
        let admitted = match self.owners.as_mut() {
            None => {
                let demand = match C::source_demands() {
                    Ok(demand) => demand,
                    Err(error) => return self.refuse(cx, error),
                };
                if !permits(grant,demand) {
                    return Turn::Yield;
                }
                match C::admit_sources(grant) {
                    Ok((owners, progress)) => {
                        *self.owners = Some(owners);
                        Ok(progress)
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        Err(refused.error.with_retained_progress(refused.progress))
                    }
                }
            }
            Some(owners) if !owners.constructor_is_complete() => {
                match owners.constructor_demands(grant.maximum_copy_bytes) {
                    Ok(demand) if permits(grant, demand) => {}
                    Ok(_) => return Turn::Yield,
                    Err(error) => return self.refuse(cx, error),
                }
                owners.admit_constructor(grant).map_err(|(error, progress)| error.with_retained_progress(progress))
            }
            Some(_) => {
                self.phase = Phase::AdmitCatalog;
                return self.tick(cx);
            }
        };
        match admitted {
            Ok(progress) => {
                if self.spend(cx, progress) && self.owners.as_ref().is_some_and(|owners| owners.constructor_is_complete()) {
                    self.phase = Phase::AdmitCatalog;
                }
                Turn::Yield
            }
            Err(error) => self.refuse(cx, error),
        }
    }

    fn admit_catalog(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Turn {
        let catalog = self.catalog.get_or_insert_with(ArtifactStoreInitializationOwnerCatalog::empty);
        if catalog.admission_is_complete() {
            self.phase = Phase::BindGenesis;
            return self.tick(cx);
        }
        match catalog.admit_next(grant) {
            Ok(progress) if progress == RetainedCloneProgress::default() => Turn::Yield,
            Ok(progress) => {
                self.spend(cx, progress);
                Turn::Yield
            }
            Err(error) => self.refuse(cx, error),
        }
    }

    fn bind_genesis(&mut self,cx:&mut StepContext<'_>)->Turn{
        if !self.registry_birth.ready().unwrap_or(false){let grant=cx.retained_grant();let result=cx.with_native_receiving(|control|self.registry_birth.advance(grant,control));let progress=result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());if let Some(issued)=cx.receiving_issued_grant(){if let Err(error)=cx.consume_receiving_retained(issued,progress){self.original_failure=Some(error.into());self.phase=Phase::RetireFault;return Turn::Yield}}cx.consume_fuel(progress.copied_items as u64);if let Err(error)=result{self.original_failure=Some(error.into());self.phase=Phase::RetireFault;}return Turn::Yield}
        let copy=size_of::<ArtifactStoreInitializationRuntime<P>>()+size_of::<ArtifactStoreInitializationOwnerCatalog>()+size_of::<ActorId>()+2*size_of::<Arc<P>>();let grant=cx.retained_grant();let d=RetirementDemand{copy_bytes:copy,depth:1,..Default::default()};if !permits(grant,d){return Turn::Yield}
        let (Some(envelope),Some(owners),Some(_))=(self.envelope.as_ref(),self.owners.as_ref(),self.actor.as_ref())else{self.fail("replay.genesis-owners-missing");return self.tick(cx)};
        let Some(factory)=owners.initial_snapshot_factory()else{self.fail("replay.initial-snapshot-factory-missing");return self.tick(cx)};
        let Some((registry,registry_progress))=self.registry_birth.take(grant).unwrap_or_else(|_|unreachable!("original registry is ready"))else{return Turn::Yield};
        let actor=self.actor.take().unwrap();let catalog=self.catalog.take().expect("complete original catalog");let genesis=envelope.vcs.genesis.facts();*self.runtime=Some(ArtifactStoreInitializationRuntime::new_with_registry(&envelope.id,&envelope.schema,genesis.share_snapshot(),genesis.digest(),actor,catalog,semio_framework_value_snapshot_read::SnapshotReadLeaseRegistryOwner::from_original(registry),semio_framework_os_spr::MutationDag::empty()));self.factory=Some(factory);self.phase=Phase::SeedHistory{edit:0,lane:0,index:0};self.spend(cx,RetainedCloneProgress{copied_items:1,copied_bytes:copy.max(registry_progress.copied_bytes),..Default::default()});Turn::Yield
    }

    fn seed_history(&mut self, cx: &mut StepContext<'_>, edit: usize, lane: u8, index: usize) -> Turn {
        let runtime=self.runtime.as_mut().expect("original causal runtime");let dag=runtime.dag.as_mut().expect("original causal owner");let demand=dag.backing_admission_demands();
        if demand.capacity_bytes!=0{let grant=cx.retained_grant();let mut output=None;let result=semio_framework_job::admit_original_construction(cx,&mut output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|Ok(((),dag.admit_backing_step(grant))));if let Err(error)=result{self.original_failure=Some(error);self.phase=Phase::RetireFault;}return Turn::Yield}
        let envelope = self.envelope.as_ref().expect("replay envelope remains retained while causal history is seeded");
        let Some(entry) = envelope.vcs.edits.get(edit) else {
            self.phase = Phase::FoldSupersessions { transition: 0 };
            return self.tick(cx);
        };
        let runtime=self.runtime.as_mut().expect("original causal runtime");
        let mut digits=[0u8;20];let mut at=digits.len();let mut number=index;loop{at-=1;digits[at]=b'0'+(number%10)as u8;number/=10;if number==0{break}}
        let authored=if lane==0{Some(entry.id.as_str())}else{entry.mutation_meta.get(index).and_then(|meta|meta.mutation_id.as_ref()).or_else(||entry.forwards.get(index).and_then(M::mutation_id)).map(|id|id.0.as_str())};
        let suffix=std::str::from_utf8(&digits[at..]).unwrap();
        let next=match lane{0=>Phase::SeedHistory{edit,lane:1,index:0},1 if index<entry.forwards.len()=>Phase::SeedHistory{edit,lane,index:index+1},1=>Phase::SeedHistory{edit,lane:2,index:0},2 if index<entry.mutation_meta.len()=>Phase::SeedHistory{edit,lane,index:index+1},_=>Phase::SeedHistory{edit:edit+1,lane:0,index:0}};
        let seeded=lane==0||(lane==1&&index<entry.forwards.len());let skipped=lane==1&&authored==Some(entry.id.as_str());
        let length=if seeded&&!skipped{authored.map_or(entry.id.len()+1+suffix.len(),str::len)}else{0};let copy=size_of::<Phase>()+size_of::<String>()+length*if authored.is_some(){1}else{2};let d=semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:length,release_bytes:0,depth:1};let mut output=None;
        let result=semio_framework_job::admit_original_construction(cx,&mut output,d,|_|{
            if seeded&&!skipped{let mut bytes=[0u8;256];if length>bytes.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original causal identity exceeds its declared byte bound"))}let text=if let Some(authored)=authored{authored}else{bytes[..entry.id.len()].copy_from_slice(entry.id.as_bytes());bytes[entry.id.len()]=b'#';bytes[entry.id.len()+1..length].copy_from_slice(suffix.as_bytes());std::str::from_utf8(&bytes[..length]).unwrap()};let dag=runtime.dag.as_mut().unwrap();if dag.validate_seed(text).is_err(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original causal identity is duplicate or outside its declaration").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:length,..Default::default()}))}dag.seed_applied(MutationId(text.to_owned())).unwrap_or_else(|_|unreachable!("original causal seed validated before insertion"));}
            if lane==0{runtime.observe_sequence(entry.sequence_number);}if lane==2&&index<entry.mutation_meta.len(){runtime.observe_timestamp(entry.mutation_meta[index].timestamp);}Ok((next,RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:length,..Default::default()}))});
        match result{Ok(_)=>{if let Some(next)=output{self.phase=next;}},Err(error)=>{self.original_failure=Some(error);self.phase=Phase::RetireFault;}}Turn::Yield
    }

    pub fn fold(&mut self,cx:&mut StepContext<'_>,_grant:RetainedCloneGrant,position:usize,edit:usize,mutation:usize)->Turn{
        if !self.native.active()&&self.envelope.as_ref().and_then(|envelope|envelope.vcs.edits.get(edit)).is_some_and(|edit|mutation>=edit.forwards.len()){
            self.phase=Phase::CommitApplied{position,edit};return self.tick(cx);
        }
        let (Some(envelope),Some(runtime),Some(owners))=(self.envelope.as_mut(),self.runtime.as_mut(),self.owners.as_ref())else{self.fail("replay.native-owners-missing");return self.tick(cx);};
        let result=self.native.advance(envelope,runtime,owners,edit,mutation,cx);
        if let Some(error)=self.native.take_failure(){self.original_failure=Some(error);self.phase=Phase::RetireFault;return Turn::Yield}
        match result{
            Ok(true)=>{self.folded+=1;self.phase=Phase::Fold{position,edit,mutation:mutation+1};Turn::Yield}
            Ok(false)=>Turn::Yield,
            Err(error)=>{self.original_failure=Some(error.into());self.phase=Phase::RetireFault;Turn::Yield},
        }
    }

    fn commit(&mut self,cx:&mut StepContext<'_>,position:usize,edit:usize,redo:bool)->Turn{
        if self.commit_ready{let demand=RetirementDemand{copy_bytes:size_of::<native_digest::NativeEditDigest>()+size_of::<native_digest::NativeEffectiveDigest>()+size_of::<bool>(),depth:1,..Default::default()};if !permits(cx.retained_grant(),demand){return Turn::Yield}self.digest=Default::default();self.effective_digest=Default::default();self.commit_ready=false;self.phase=if redo{Phase::FindRedo{position:position+1}}else{Phase::FindApplied{position:position+1}};self.spend(cx,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});return Turn::Yield}
        let envelope=self.envelope.as_ref().expect("original commit envelope");let entry=envelope.vcs.edits.get(edit).expect("original commit edit");
        let Some(digest)=self.digest.digest()else{let result=self.digest.advance(entry,cx.retained_grant());return match result{Ok(progress)=>{self.spend(cx,progress);Turn::Yield},Err(error)=>self.refuse(cx,error)}};
        let Some(digest)=self.effective_digest.digest()else{let result=self.effective_digest.advance::<P,M>(entry,&self.runtime.as_ref().expect("original effective commit runtime").supersessions,digest,cx.retained_grant());return match result{Ok(progress)=>{self.spend(cx,progress);Turn::Yield},Err(error)=>self.refuse(cx,error)}};
        let key=envelope.vcs.edits.key_at(edit).expect("original commit ledger key");let runtime=self.runtime.as_mut().expect("original commit runtime");
        let demand=match runtime.commit_metadata_demands(entry,key,digest,redo){Ok(value)=>value,Err(error)=>return self.refuse(cx,error)};
        let grant=cx.retained_grant();let mut complete=None;let result=semio_framework_job::admit_original_construction(cx,&mut complete,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|runtime.advance_commit_metadata(entry,key,digest,redo,grant));
        match result{Ok(true)if complete==Some(true)=>{self.commit_ready=true;Turn::Yield},Ok(_)=>Turn::Yield,Err(error)=>{self.original_failure=Some(error);self.phase=Phase::RetireFault;Turn::Yield}}
    }

    fn build_candidate(&mut self,cx:&mut StepContext<'_>)->Turn{
        let Some(candidate_generation)=self.generation.0.checked_add(1)else{self.fail("replay.generation-exhausted");return Turn::Yield};
        let Some(runtime)=self.runtime.as_ref()else{self.fail("replay.candidate-owners-missing");return Turn::Yield};let mut demand=match ArtifactStore::<P,M>::initialized_runtime_adoption_demands(self.envelope.as_ref().expect("original candidate envelope remains resident"),runtime){Ok(value)=>value,Err(error)=>return self.refuse(cx,error)};demand.copy_bytes=match demand.copy_bytes.checked_add(size_of::<Box<ArtifactStore<P,M>>>()){Some(bytes)=>bytes,None=>return self.refuse(cx,ValueError::literal(ValueRefusalKind::OwnershipLimit,"native candidate pointer extent overflow"))};demand.capacity_bytes=match demand.capacity_bytes.checked_add(size_of::<ArtifactStore<P,M>>()){Some(bytes)=>bytes,None=>return self.refuse(cx,ValueError::literal(ValueRefusalKind::OwnershipLimit,"native candidate allocation extent overflow"))};
        let result=semio_framework_job::admit_original_construction(cx,&mut self.candidate,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|{let envelope=self.envelope.take().expect("original candidate envelope");let runtime=self.runtime.take().expect("original candidate runtime");let owners=self.owners.take().expect("original candidate owner catalog");drop(self.factory.take());let candidate=Box::new(ArtifactStore::from_initialized_runtime_with_owners(envelope,runtime,candidate_generation,owners));Ok((candidate,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))});
        match result{Ok(true)=>{self.phase=Phase::Complete;Turn::Complete},Ok(false)=>Turn::Yield,Err(error)=>{self.original_failure=Some(error);self.phase=Phase::RetireFault;Turn::Yield}}
    }

    fn retire_native(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
        if self.supersession_native.active(){match self.supersession_native.close(cx){Ok(true)=>{},Ok(false)=>return Ok(false),Err(error)=>{self.original_failure=Some(error);return Ok(false)}}}

        if !self.registry_birth.terminal_is_empty(){
            self.registry_birth.begin_close();let grant=cx.retained_grant();let quote=self.registry_birth.close_demands()?;
            if !permits(grant,quote){return Ok(false);}
            let result=cx.with_native_retirement(|control|self.registry_birth.close_step(grant,control));
            let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(progress)?;cx.consume_fuel(progress.copied_items as u64);result?;
            return Ok(false);
        }
        if !self.native.active(){return Ok(true);}
        self.native.begin_close();
        let (Some(envelope),Some(runtime),Some(owners))=(self.envelope.as_mut(),self.runtime.as_mut(),self.owners.as_ref())else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native initialization close lost original owners"));};
        let result=self.native.close(envelope,runtime,owners,cx);if let Some(error)=self.native.take_failure(){self.original_failure=Some(error);return Ok(false)}result
    }

    pub fn retire(&mut self, cx: &mut StepContext<'_>) -> Turn {
        match self.retire_native(cx){Ok(true)=>{},Ok(false)=>return Turn::Yield,Err(error)=>{self.original_failure=Some(error.into());return Turn::Yield;}}
        match self.close_original(cx.retained_grant()) {
            Ok(step) => {
                if cx.consume_retained(step.progress()).is_err() {
                    return Turn::Yield;
                }
                cx.consume_fuel(1);
                if !self.close_is_terminal() {
                    return Turn::Yield;
                }
                self.terminal_handoff = true;
                if self.phase == Phase::RetireCancelled {
                    self.phase = Phase::Cancelled;
                    Turn::Cancelled
                } else {
                    self.phase = Phase::Fault;
                    self.fault_detail = Some(self.fault.take().unwrap_or_else(|| "replay.fault".into()));
                    Turn::Fault
                }
            }
            Err(error) => {
                let _ = cx.consume_retained(error.retained_progress());
                self.fault.get_or_insert_with(|| error.into_message());
                Turn::Yield
            }
        }
    }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.supersession_native.active(){return self.supersession_native.close_demands(body);}
        if !self.registry_birth.terminal_is_empty(){return self.registry_birth.close_demands();}
        if self.native.active(){let (Some(envelope),Some(runtime))=(self.envelope.as_ref(),self.runtime.as_ref())else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native initialization close lost original runtime"));};return self.native.close_demands(envelope,runtime,body);}
        if let Some(active) = self.active.as_ref() {
            return nested(artifact_retirement_box_demands(active, body)?);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return nested(runtime.initialization_retirement_demands(body)?);
        }
        if self.factory.is_some() {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(catalog) = self.catalog.as_ref() {
            return nested(catalog.close_demands()?);
        }
        if let Some(candidate) = self.candidate.as_ref() {
            return if candidate.close_owned_store_terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<ArtifactStore<P,M>>>>(),release_bytes:size_of::<ArtifactStore<P,M>>(),depth:1,..Default::default()})}else{nested(candidate.close_owned_demands(body)?)};
        }
        if let Some(retirement) = self.envelope_retirement.as_ref() {
            return nested(artifact_retirement_box_demands(retirement, body)?);
        }
        if let Some(envelope) = self.envelope.as_ref() {
            let Some(owners) = self.owners.as_ref() else {
                let mut source=C::source_demands()?;source.copy_bytes=source.copy_bytes.checked_add(size_of::<Option<DocumentStoreOwners<P,M>>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"replay original catalog placement overflows"))?;return Ok(source);
            };
            if !owners.constructor_is_complete() {
                return nested(owners.constructor_demands(body)?);
            }
            let mut demand = nested(owners.uninstalled_envelope_retirement_demands(envelope))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "replay initializer envelope placement overflow"))?;
            return Ok(demand);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.uninstalled_owners_terminal_is_empty() { Ok(RetirementDemand { copy_bytes: size_of::<Option<DocumentStoreOwners<P, M>>>(), depth: 1, ..Default::default() }) } else { nested(owners.uninstalled_owners_demands(body)?) };
        }
        if self.index_open {
            return self.edit_index.retirement_demands(body);
        }
        if let Some(actor) = self.actor_close.as_ref() {
            return if actor.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<ControlledRetirement<ActorId>>>(), depth: 1, ..Default::default() })
            } else {
                nested(actor.next_demand(body)?)
            };
        }
        if self.actor.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<Option<ActorId>>() + size_of::<Option<ControlledRetirement<ActorId>>>(), depth: 1, ..Default::default() });
        }
        if !self.publication.terminal_is_empty() {
            return nested(self.publication.retirement_demands()?);
        }
        Ok(Default::default())
    }

    fn close_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.close_is_terminal() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth.max(1) {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "replay initializer retirement exceeds admitted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if self.active.is_some() {
            return artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory = self.factory.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "replay runtime lost its snapshot factory"))?;
            let step = runtime.close_step(factory, child)?;
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.factory.is_some() {
            drop(self.factory.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(catalog) = self.catalog.as_mut() {
            if catalog.terminal_is_empty() {
                drop(self.catalog.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let step = catalog.close_step(child)?;
            return admit_retained_clone_close(child, step, catalog.terminal_is_empty(), "replay initializer catalog").map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(candidate) = self.candidate.as_mut() {
            if candidate.close_owned_store_terminal_is_empty(){drop(self.candidate.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..empty}));}
            let step = candidate.close_owned_store_step(child)?;
            let step = admit_retained_clone_close(child, step, candidate.close_owned_store_terminal_is_empty(), "replay initializer candidate store")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.envelope_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let owner_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match C::admit_sources(owner_grant) {
                    Ok((owners, mut receipt)) => {
                        *self.owners = Some(owners);
                        receipt.copied_bytes += placement;
                        Ok(RetainedCloneStep::Progress(receipt))
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        Err(refused.error.with_retained_progress(refused.progress))
                    }
                };
            }
            let owners = self.owners.as_mut().expect("replay initializer owner catalog");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, progress)| error.with_retained_progress(progress));
            }
            let placement = size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>();
            let child = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("replay initializer owner catalog");
            let envelope = self.envelope.take().expect("replay initializer envelope");
            return match owners.retire_envelope_uninstalled(envelope, child) {
                Ok((owner, mut receipt)) => {
                    *self.envelope_retirement = Some(owner);
                    receipt.copied_bytes += placement;
                    Ok(RetainedCloneStep::Progress(receipt))
                }
                Err((error, owners, envelope)) => {
                    *self.owners = Some(owners);
                    *self.envelope = Some(envelope);
                    Err(error)
                }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            return admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "replay initializer original catalog").map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.index_open {
            let step=self.edit_index.retire_step(grant)?;self.index_open=!matches!(step,RetainedCloneStep::Complete(_));return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(actor) = self.actor_close.as_mut() {
            if actor.terminal_is_empty() {
                drop(self.actor_close.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = actor.step(child)?;
            return admit_retained_clone_close(child, step, actor.terminal_is_empty(), "replay initializer original actor").map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(actor) = self.actor.take() {
            return match ControlledRetirement::new(actor) {
                Ok(owner) => {
                    *self.actor_close = Some(owner);
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
                }
                Err((error, actor)) => {
                    *self.actor = Some(actor);
                    Err(error)
                }
            };
        }
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
}

impl<P, M, C> ArtifactStoreInitializationAuthority<P, M> for ArtifactStoreReplayInitializer<P, M, C>
where
    P: RetireOwned + semio_framework_os_spr::StoreState,
    M: RetireOwned + semio_framework_pack_json::ArtifactCanonicalJsonTree + semio_framework_os_spr::StoreMutationFor<P>,
    C:DocumentStoreOwnerProducer<P,M>,
{
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> {
        let turn=self.turn(cx);if let Some(failure)=self.original_failure.take(){return Err(failure)}
        (match turn {
            Turn::Yield => JobOutcomeBorrow::admit_yield(cx),
            Turn::Complete => JobOutcomeBorrow::admit_complete(cx, None, None),
            Turn::Cancelled => JobOutcomeBorrow::admit_cancelled(cx),
            Turn::Fault => {
                let detail = self.fault_detail.as_deref().unwrap_or("replay.fault").as_bytes();
                self.publication.advance_from_source(JobPublicationKind::Fault, detail, cx)
            }
        }).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "replay initializer lends only yield and terminal outcomes")),
        }
    }

    fn progress(&self) -> (u64, u64) {
        (self.folded, self.total)
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<ArtifactStore<P, M>> {
        self.take_boxed_candidate().map(|candidate|*candidate)
    }

    fn take_boxed_candidate(&mut self) -> Option<Box<ArtifactStore<P, M>>> {
        if self.phase != Phase::Complete || self.terminal_handoff || self.cancel_requested { return None; }
        let candidate=self.candidate.take()?;
        self.terminal_handoff=true;
        Some(candidate)
    }

    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(maximum_copy_bytes)
    }

    fn begin_close(&mut self) {
        self.registry_birth.begin_close();
        self.native.begin_close();
        self.cancel_requested = true;
        if !matches!(self.phase, Phase::Cancelled | Phase::Fault | Phase::RetireFault) {
            self.phase = Phase::RetireCancelled;
        }
    }

    fn close_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>) -> Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure> {
        self.begin_close();
        if !self.retire_native(cx)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.terminal_is_empty(){return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))}
        let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
        self.begin_close();
        let step = self.close_original(grant)?;
        if self.close_is_terminal() {
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))

        })?;
        Ok(if self.terminal_is_empty(){semio_framework_value::retained_clone::RetainedCloneStep::Complete(step.progress())}else{semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())})
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_handoff && self.close_is_terminal()
    }
}

impl<P, M, C> ArtifactStoreReplayInitializer<P, M, C>
where
    P: RetireOwned + semio_framework_os_spr::StoreState,
    M: RetireOwned + semio_framework_pack_json::ArtifactCanonicalJsonTree + semio_framework_os_spr::StoreMutationFor<P>,
    C:DocumentStoreOwnerProducer<P,M>,
{
    fn close_core_is_terminal(&self) -> bool {
        self.supersession_native.terminal_is_empty() && self.registry_birth.terminal_is_empty() && self.native.terminal_is_empty() && self.envelope.is_none()
            && self.owners.is_none()
            && self.catalog.is_none()
            && self.runtime.is_none()
            && self.factory.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.actor.is_none()
            && self.actor_close.is_none()
            && !self.index_open
    }

    fn close_is_terminal(&self) -> bool {
        self.close_core_is_terminal() && self.publication.terminal_is_empty()
    }
}

impl<P, M, C> Drop for ArtifactStoreReplayInitializer<P, M, C>
where
    P: RetireOwned + semio_framework_os_spr::StoreState,
    M: RetireOwned + semio_framework_pack_json::ArtifactCanonicalJsonTree + semio_framework_os_spr::StoreMutationFor<P>,
    C:DocumentStoreOwnerProducer<P,M>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_handoff && self.close_is_terminal()), "replay initializer reached Drop before exact candidate handoff or retained rejection close");
    }
}

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;


#[cfg(test)]
#[path="🧬️replacement/🧪️tests/🦀️.rs"]
pub(crate) mod replacement_tests;

use semio_framework_os_store_replay::native_supersession;
#[cfg(test)]
#[path="🧬️supersession/🧪️tests/🦀️.rs"]
mod native_supersession_tests;
