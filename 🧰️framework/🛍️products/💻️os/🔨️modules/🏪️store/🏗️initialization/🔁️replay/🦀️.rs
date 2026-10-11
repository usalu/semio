//! 🔁️ The framework-default retained store initializer: a ledger replay under the grant-based retained-clone contract.

use super::{
    ActorId, ArtifactEnvelope, ArtifactOwnedValueRetirementFactory, ArtifactPack, ArtifactStore, ArtifactStoreInitializationAuthority, ArtifactStoreInitializationEditAdmission, ArtifactStoreInitializationEditIndex, ArtifactStoreInitializationOwnerCatalog,
    ArtifactStoreInitializationRuntime, CursorRevisionAccumulator, DocumentStoreOwners, Edit, ErasedSnapshotRetirement, FromValue, Mutation, MutationId, OpBinary, OpText, ReturnedSnapshotReadRetirement, ToValue, artifact_retirement_admit_owned, artifact_retirement_box_close_step,
    artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, bounded_artifact_store_owners, bounded_artifact_store_owners_birth_bytes, bounded_artifact_store_owners_birth_demand, fold_operation, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,
};
use semio_framework_job::{Generation, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPublicationKind, OperationId, RetainedJobPublication, StepContext};
use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{controlled::ControlledRetirement, RetireOwned};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::{size_of, size_of_val, ManuallyDrop};
use std::sync::Arc;

/// 📏️ The widest id or encoded operation one replay turn admits, the page every retained store initializer shares.
pub const ARTIFACT_STORE_REPLAY_FIELD_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

const INDEX_RETIRE_PAGE: usize = 64;
const ITEM: RetainedCloneProgress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: 0 };

/// ▶️ What folding one forward of an applied edit onto the initialization runtime's current projection produced.
pub enum ArtifactStoreInitializationNext<P> {
    /// 🏁️ The edit has no forward at that index.
    Exhausted,
    /// 🪙️ The operation's encoded bytes exceed the copy bytes the caller granted; nothing changed.
    Unfunded,
    /// ⏭️ The forward folded with keep-and-record semantics: `next` is the projection it produced (`None` when it folded as a
    /// no-op — withdrawn, faulted or refused) and `copy_bytes` the operation's encoded size.
    Folded { next: Option<P>, copy_bytes: usize },
}

impl<P> ArtifactStoreInitializationRuntime<P> {
    /// ▶️ Folds forward `index` of the applied `edit` onto the shared current projection through its effective input
    /// (supersessions, [`admit_replacement`](super::admit_replacement) and keep-and-record [`fold_operation`](super::fold_operation)),
    /// handing the produced projection to the caller instead of mutating a uniquely owned workspace: the genesis alias is never
    /// cloned, the one `diff` and `apply` of the operation is the whole traversal of the turn. Refuses an operation whose encoding
    /// fails or exceeds `maximum_bytes`; answers [`ArtifactStoreInitializationNext::Unfunded`] when it exceeds `granted_copy_bytes`.
    pub fn fold_next_forward<M>(&mut self, edit: &Edit<M>, index: usize, schema: &str, maximum_bytes: usize, granted_copy_bytes: usize) -> Result<ArtifactStoreInitializationNext<P>, &'static str>
    where
        M: Mutation<P> + OpBinary,
    {
        let Some(effective) = self.effective_forward(edit, index, schema) else { return Ok(ArtifactStoreInitializationNext::Exhausted) };
        let operation = effective.operation();
        let copy_bytes = match operation.map(OpBinary::encode_op) {
            None => 0,
            Some(Ok(encoded)) if encoded.len() <= maximum_bytes => encoded.len(),
            Some(_) => return Err("artifact store initialization forward encoding exceeds its page"),
        };
        if copy_bytes > granted_copy_bytes {
            return Ok(ArtifactStoreInitializationNext::Unfunded);
        }
        if let Some(original) = edit.forwards.get(index).filter(|op| op.may_emit_foreign_steps()) {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            let source = self.current.as_deref().ok_or("artifact store initialization lost its current projection")?;
            let presence = original.foreign_step_source(source, 0).map_err(|_| "artifact store original foreign source refused")?.is_some();
            self.revision.unit_flags.insert((digest, index), presence);
        }
        let next = match operation {
            None => None,
            Some(operation) => fold_operation::<P, M>(self.current.as_deref().ok_or("artifact store initialization lost its current projection")?, operation, index as u32).0,
        };
        Ok(ArtifactStoreInitializationNext::Folded { next, copy_bytes })
    }

    /// 📏️ Quotes [`Self::adopt_next_funded`]: the returned-read frame that retires the displaced projection and the shared
    /// backing of its successor.
    pub fn adopt_next_demand() -> RetirementDemand
    where
        P: Send + Sync + 'static,
    {
        RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes() + semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<P>(), depth: 2, ..Default::default() }
    }

    /// 🎟️ Makes `next` the current projection under one funded turn: the displaced projection moves into a returned-read
    /// retirement frame that [`Self::settle_current_retirement_step`] drains, and `next` takes its shared backing. Refusal
    /// hands `next` back and leaves the runtime unchanged.
    pub fn adopt_next_funded(&mut self, next: P, factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, P)>
    where
        P: Send + Sync + 'static,
    {
        let demand = Self::adopt_next_demand();
        if self.close_phase != 0 || self.close_active.is_some() || self.current.is_none() {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "initialization adoption requires a settled open current projection"), next));
        }
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes {
            return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "initialization adoption exceeds its admitted grant"), next));
        }
        let previous = self.current.take().expect("open initialization retains its current projection");
        match ReturnedSnapshotReadRetirement::admit(previous, Arc::clone(factory), grant) {
            Ok((owner, mut receipt)) => {
                *self.close_active = Some(owner);
                *self.current = Some(Arc::new(next));
                receipt.retained_capacity_bytes += semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<P>();
                Ok(receipt)
            }
            Err((error, previous, alias)) => {
                *self.current = Some(previous);
                drop(alias);
                Err((error, next))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
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
    Adopt { position: usize, edit: usize, mutation: usize },
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

fn permits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items > 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "replay initializer retained child depth overflow"))?;
    Ok(demand)
}

/// 🔁️ The framework-default [`ArtifactStoreInitializationAuthority`]: it replays one completed envelope's history ledger — the
/// genesis snapshot plus the applied edits' forwards, each through [`Mutation::diff`](super::Mutation) and `apply_diff`, under
/// the supersessions the log folds to ([`admit_replacement`](super::admit_replacement)) — into a store, one bounded turn at a time,
/// and loads every document whose initialization is nothing more than that replay.
///
/// ⛓️ Traversal bound: one turn performs at most ONE operation's `diff` + `apply_diff` (the single traversal of the projection it
/// patches) and otherwise O(1) work; validation, history seeding, supersession folding, applied/redo cursor building and every
/// retirement are one id, one edit or one funded retirement frame per turn. The projection a fold produces is the mutation's own
/// allocation and is not a grant currency; everything the initializer itself allocates (owner catalog, history pages, returned-read
/// frame, shared backing of each successor projection) is funded from the step wallet and released through exact retirement frames.
///
/// 🪙️ Wallet: a work turn spends `copied_items: 1` (and the encoded size of the operation it folds in `copied_bytes`); a wallet
/// that cannot afford the next turn makes no progress and yields, a wallet of at least [`ARTIFACT_STORE_REPLAY_FIELD_BYTES`] copy
/// bytes, one admission and one retirement frame always makes progress. Cancellation is honored at every turn boundary and closes
/// by exact retirement of whatever the replay had born so far.
///
/// 🔗️ [`ArtifactStoreInitializationRuntime`] · [`ArtifactStore::from_initialized_runtime_with_owners`]
pub struct ArtifactStoreReplayInitializer<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + RetireOwned + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + RetireOwned + Send + 'static,
{
    operation: OperationId,
    generation: Generation,
    schema: &'static str,
    envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    catalog: ManuallyDrop<Option<ArtifactStoreInitializationOwnerCatalog>>,
    runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    factory: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>,
    candidate: ManuallyDrop<Option<ArtifactStore<P, M>>>,
    staged: ManuallyDrop<Option<P>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    envelope_retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    actor: ManuallyDrop<Option<ActorId>>,
    actor_close: ManuallyDrop<Option<ControlledRetirement<ActorId>>>,
    edit_index: ArtifactStoreInitializationEditIndex,
    index_open: bool,
    publication: RetainedJobPublication,
    phase: Phase,
    cancel_requested: bool,
    fault: Option<String>,
    fault_detail: Option<String>,
    folded: u64,
    total: u64,
    terminal_handoff: bool,
}

impl<P, M> ArtifactStoreReplayInitializer<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + RetireOwned + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + RetireOwned + Send + 'static,
{
    /// 🌱️ Captures the completed `envelope` of a document of `schema` for the replay `operation` of `generation`, owned by `actor`.
    pub fn new(envelope: ArtifactEnvelope<P, M>, schema: &'static str, operation: OperationId, generation: Generation, actor: ActorId) -> Self {
        Self {
            operation,
            generation,
            schema,
            envelope: ManuallyDrop::new(Some(envelope)),
            owners: ManuallyDrop::new(None),
            catalog: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
            factory: None,
            candidate: ManuallyDrop::new(None),
            staged: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            envelope_retirement: ManuallyDrop::new(None),
            actor: ManuallyDrop::new(Some(actor)),
            actor_close: ManuallyDrop::new(None),
            edit_index: ArtifactStoreInitializationEditIndex::default(),
            index_open: false,
            publication: RetainedJobPublication::new(),
            phase: Phase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            fault_detail: None,
            folded: 0,
            total: 0,
            terminal_handoff: false,
        }
    }

    fn running(&self) -> bool {
        !matches!(self.phase, Phase::Complete | Phase::RetireCancelled | Phase::RetireFault | Phase::Cancelled | Phase::Fault)
    }

    fn fail(&mut self, reason: impl Into<String>) {
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

    fn refuse(&mut self, cx: &mut StepContext<'_>, error: ValueError) -> Turn {
        let _ = cx.consume_retained(error.retained_progress());
        self.fail(error.into_message());
        Turn::Yield
    }

    fn tick(&mut self, cx: &mut StepContext<'_>) -> Turn {
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

    fn turn(&mut self, cx: &mut StepContext<'_>) -> Turn {
        if cx.operation() != self.operation || cx.generation() != self.generation || cx.is_cancelled() {
            self.cancel_requested = true;
        }
        if self.cancel_requested && self.running() {
            self.phase = Phase::RetireCancelled;
        }
        if self.running() && self.settle(cx) {
            return Turn::Yield;
        }
        let grant = cx.retained_grant();
        if self.running() && (grant.maximum_items == 0 || grant.maximum_depth == 0) {
            return Turn::Yield;
        }
        match self.phase {
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
                match self.edit_index.admit(&envelope.vcs.edits, index, ARTIFACT_STORE_REPLAY_FIELD_BYTES) {
                    ArtifactStoreInitializationEditAdmission::Complete => self.phase = Phase::AdmitOwners,
                    ArtifactStoreInitializationEditAdmission::Admitted => {
                        self.index_open = true;
                        self.phase = Phase::ValidateEdit { index: index + 1 };
                    }
                    ArtifactStoreInitializationEditAdmission::Oversized | ArtifactStoreInitializationEditAdmission::Duplicate => self.fail("replay.duplicate-or-hostile-edit"),
                }
                self.tick(cx)
            }
            Phase::AdmitOwners => self.admit_owners(cx, grant),
            Phase::AdmitCatalog => self.admit_catalog(cx, grant),
            Phase::BindGenesis => self.bind_genesis(cx),
            Phase::SeedHistory { edit, lane, index } => self.seed_history(cx, edit, lane, index),
            Phase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("replay envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("replay runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = Phase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = Phase::CountApplied { position: 0 },
                    Err(reason) => self.fail(reason),
                }
                self.tick(cx)
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
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("replay runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = Phase::FindRedo { position: 0 };
                    return self.tick(cx);
                };
                let edit = self.edit_index.position(id).filter(|at| self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(*at)).is_some_and(|edit| edit.id == id));
                match edit {
                    Some(edit) => self.phase = Phase::Fold { position, edit, mutation: 0 },
                    None => self.fail("replay.applied-edit-missing"),
                }
                self.tick(cx)
            }
            Phase::Fold { position, edit, mutation } => self.fold(cx, grant, position, edit, mutation),
            Phase::Adopt { position, edit, mutation } => self.adopt(cx, grant, position, edit, mutation),
            Phase::CommitApplied { position, edit } => {
                let envelope = self.envelope.as_ref().expect("replay envelope remains retained");
                let entry = envelope.vcs.edits.get(edit).expect("replay applied edit remains retained");
                let key = envelope.vcs.edits.key_at(edit).expect("authoritative retained edit key");
                match self.runtime.as_mut().expect("replay runtime remains retained").push_applied_edit(entry, key) {
                    Ok(()) => self.phase = Phase::FindApplied { position: position + 1 },
                    Err(reason) => self.fail(reason),
                }
                self.tick(cx)
            }
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
            Phase::CommitRedo { position, edit } => {
                let envelope = self.envelope.as_ref().expect("replay envelope remains retained");
                let entry = envelope.vcs.edits.get(edit).expect("replay redo edit remains retained");
                let key = envelope.vcs.edits.key_at(edit).expect("authoritative retained edit key");
                match self.runtime.as_mut().expect("replay runtime remains retained").push_redo_edit(entry, key) {
                    Ok(()) => self.phase = Phase::FindRedo { position: position + 1 },
                    Err(reason) => self.fail(reason),
                }
                self.tick(cx)
            }
            Phase::RetireIndex => {
                if self.edit_index.retire_step(INDEX_RETIRE_PAGE) {
                    self.index_open = false;
                    self.phase = Phase::BuildCandidate;
                }
                self.tick(cx)
            }
            Phase::BuildCandidate => self.build_candidate(),
            Phase::Complete => Turn::Complete,
            Phase::RetireCancelled | Phase::RetireFault => self.retire(cx),
            Phase::Cancelled => Turn::Cancelled,
            Phase::Fault => Turn::Fault,
        }
    }

    fn admit_owners(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Turn {
        let admitted = match self.owners.as_mut() {
            None => {
                let demand = match bounded_artifact_store_owners_birth_demand::<P, M>() {
                    Ok(demand) => demand,
                    Err(error) => return self.refuse(cx, error),
                };
                if demand.admit(grant).is_err() {
                    return Turn::Yield;
                }
                match bounded_artifact_store_owners::<P, M>(grant) {
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

    fn bind_genesis(&mut self, cx: &mut StepContext<'_>) -> Turn {
        let (Some(envelope), Some(owners), Some(actor)) = (self.envelope.as_ref(), self.owners.as_ref(), self.actor.take()) else {
            self.fail("replay.genesis-owners-missing");
            return self.tick(cx);
        };
        let Some(factory) = owners.initial_snapshot_factory() else {
            *self.actor = Some(actor);
            self.fail("replay.initial-snapshot-factory-missing");
            return self.tick(cx);
        };
        let catalog = self.catalog.take().expect("a complete replay catalog precedes genesis binding");
        let genesis = envelope.vcs.genesis.facts();
        *self.runtime = Some(ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, genesis.share_snapshot(), genesis.digest(), actor, catalog));
        self.factory = Some(factory);
        self.phase = Phase::SeedHistory { edit: 0, lane: 0, index: 0 };
        self.tick(cx)
    }

    fn seed_history(&mut self, cx: &mut StepContext<'_>, edit: usize, lane: u8, index: usize) -> Turn {
        let envelope = self.envelope.as_ref().expect("replay envelope remains retained while causal history is seeded");
        let Some(entry) = envelope.vcs.edits.get(edit) else {
            self.phase = Phase::FoldSupersessions { transition: 0 };
            return self.tick(cx);
        };
        let runtime = self.runtime.as_mut().expect("replay runtime remains retained while causal history is seeded");
        let next = match lane {
            0 => runtime.seed_mutation(MutationId(entry.id.clone())).map(|()| {
                runtime.observe_sequence(entry.sequence_number);
                Phase::SeedHistory { edit, lane: 1, index: 0 }
            }),
            1 if index < entry.forwards.len() => {
                let id = crate::os_spr::mutation_id_for_edit_operation::<P, M>(entry, index).expect("indexed operation exists");
                runtime.seed_edit_operation(&entry.id, id).map(|()| Phase::SeedHistory { edit, lane, index: index + 1 })
            }
            1 => Ok(Phase::SeedHistory { edit, lane: 2, index: 0 }),
            2 if index < entry.mutation_meta.len() => {
                runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                Ok(Phase::SeedHistory { edit, lane, index: index + 1 })
            }
            _ => Ok(Phase::SeedHistory { edit: edit + 1, lane: 0, index: 0 }),
        };
        match next {
            Ok(phase) => self.phase = phase,
            Err(reason) => self.fail(reason),
        }
        self.tick(cx)
    }

    fn fold(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant, position: usize, edit: usize, mutation: usize) -> Turn {
        let envelope = self.envelope.as_ref().expect("replay envelope remains retained while its forwards fold");
        let entry = envelope.vcs.edits.get(edit).expect("replay applied edit remains retained");
        let runtime = self.runtime.as_mut().expect("replay runtime remains retained while its forwards fold");
        match runtime.fold_next_forward(entry, mutation, &envelope.schema, ARTIFACT_STORE_REPLAY_FIELD_BYTES, grant.maximum_copy_bytes) {
            Ok(ArtifactStoreInitializationNext::Exhausted) => {
                self.phase = Phase::CommitApplied { position, edit };
                self.tick(cx)
            }
            Ok(ArtifactStoreInitializationNext::Unfunded) => Turn::Yield,
            Ok(ArtifactStoreInitializationNext::Folded { next, copy_bytes }) => {
                self.folded += 1;
                self.phase = match next {
                    Some(next) => {
                        *self.staged = Some(next);
                        Phase::Adopt { position, edit, mutation }
                    }
                    None => Phase::Fold { position, edit, mutation: mutation + 1 },
                };
                self.spend(cx, RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() });
                Turn::Yield
            }
            Err(reason) => {
                self.fail(reason);
                self.tick(cx)
            }
        }
    }

    fn adopt(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant, position: usize, edit: usize, mutation: usize) -> Turn {
        if !permits(grant, ArtifactStoreInitializationRuntime::<P>::adopt_next_demand()) {
            return Turn::Yield;
        }
        let next = self.staged.take().expect("replay adoption retains its staged projection");
        let factory = self.factory.as_ref().expect("replay runtime retains its snapshot factory");
        match self.runtime.as_mut().expect("replay runtime remains retained while its projection is adopted").adopt_next_funded(next, factory, grant) {
            Ok(progress) => {
                self.phase = Phase::Fold { position, edit, mutation: mutation + 1 };
                self.spend(cx, progress);
                Turn::Yield
            }
            Err((error, next)) => {
                *self.staged = Some(next);
                self.refuse(cx, error)
            }
        }
    }

    fn build_candidate(&mut self) -> Turn {
        let Some(candidate_generation) = self.generation.0.checked_add(1) else {
            self.fail("replay.generation-exhausted");
            return Turn::Yield;
        };
        drop(self.factory.take());
        let (Some(envelope), Some(runtime), Some(owners)) = (self.envelope.take(), self.runtime.take(), self.owners.take()) else {
            self.fail("replay.candidate-owners-missing");
            return Turn::Yield;
        };
        *self.candidate = Some(ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners));
        self.phase = Phase::Complete;
        Turn::Complete
    }

    fn retire(&mut self, cx: &mut StepContext<'_>) -> Turn {
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

    fn close_core_is_terminal(&self) -> bool {
        self.envelope.is_none()
            && self.owners.is_none()
            && self.catalog.is_none()
            && self.runtime.is_none()
            && self.factory.is_none()
            && self.candidate.is_none()
            && self.staged.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.actor.is_none()
            && self.actor_close.is_none()
            && !self.index_open
    }

    fn close_is_terminal(&self) -> bool {
        self.close_core_is_terminal() && self.publication.terminal_is_empty()
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return nested(artifact_retirement_box_demands(active, body)?);
        }
        if self.staged.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.staged)?);
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
            return nested(candidate.close_owned_demands(body)?);
        }
        if let Some(retirement) = self.envelope_retirement.as_ref() {
            return nested(artifact_retirement_box_demands(retirement, body)?);
        }
        if let Some(envelope) = self.envelope.as_ref() {
            let Some(owners) = self.owners.as_ref() else {
                return Ok(RetirementDemand { copy_bytes: size_of::<Option<DocumentStoreOwners<P, M>>>(), capacity_bytes: bounded_artifact_store_owners_birth_bytes::<P, M>(), depth: 1, ..Default::default() });
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
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(actor) = self.actor_close.as_ref() {
            return if actor.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<ControlledRetirement<ActorId>>>(), depth: 1, ..Default::default() })
            } else {
                nested(RetirementDemand { copy_bytes: actor.next_copy_byte_demand()?, capacity_bytes: actor.next_capacity_byte_demand(body)?, release_bytes: actor.next_release_byte_demand()?, depth: actor.next_depth_demand()? })
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
        if self.staged.is_some() {
            return artifact_retirement_admit_owned(&mut self.staged, &mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
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
            let step = candidate.close_owned_store_step(child)?;
            let step = admit_retained_clone_close(child, step, candidate.close_owned_store_terminal_is_empty(), "replay initializer candidate store")?;
            if candidate.close_owned_store_terminal_is_empty() {
                drop(self.candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.envelope_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let owner_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match bounded_artifact_store_owners::<P, M>(owner_grant) {
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
            self.index_open = !self.edit_index.retire_step(INDEX_RETIRE_PAGE);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
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

impl<P, M> ArtifactStoreInitializationAuthority<P, M> for ArtifactStoreReplayInitializer<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + RetireOwned + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + RetireOwned + Send + 'static,
{
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        match self.turn(cx) {
            Turn::Yield => JobOutcomeBorrow::admit_yield(cx),
            Turn::Complete => JobOutcomeBorrow::admit_complete(cx, None, None),
            Turn::Cancelled => JobOutcomeBorrow::admit_cancelled(cx),
            Turn::Fault => {
                let detail = self.fault_detail.as_deref().unwrap_or("replay.fault").as_bytes();
                self.publication.advance_from_source(JobPublicationKind::Fault, detail, cx)
            }
        }
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
        if self.phase != Phase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(maximum_copy_bytes)
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, Phase::Cancelled | Phase::Fault | Phase::RetireFault) {
            self.phase = Phase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.begin_close();
        let step = self.close_original(grant)?;
        if self.close_is_terminal() {
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_handoff && self.close_is_terminal()
    }
}

impl<P, M> Drop for ArtifactStoreReplayInitializer<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + RetireOwned + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + RetireOwned + Send + 'static,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_handoff && self.close_is_terminal()), "replay initializer reached Drop before exact candidate handoff or retained rejection close");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
