//! 🧮️ Cursor, ledger and VCS retirements: the revision accumulator, cursor and mutation DAG retirements, the history ledger and VCS retirements with their factories and the document envelope birth. See [`super`].

use super::*;

pub struct ArtifactStoreRevisionAccumulatorRetirement {
    pub owner: semio_framework_value::retirement::controlled::ControlledRetirement<CursorRevisionAccumulator>,
}

impl ArtifactStoreRevisionAccumulatorRetirement {
    pub fn new(accumulator: CursorRevisionAccumulator) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(accumulator).unwrap_or_else(|_| unreachable!("Store revision schema declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreRevisionAccumulatorRetirement {
        fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.owner.next_demand(body) }
fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owner.step(grant) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

pub struct ArtifactStoreCursorRetirement {
    pub owner: semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactCursor>,
}

impl ArtifactStoreCursorRetirement {
    pub fn new(cursor: ArtifactCursor) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(cursor).unwrap_or_else(|_| unreachable!("Store cursor declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreCursorRetirement {
        fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.owner.next_demand(body) }
fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owner.step(grant) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

pub struct ArtifactStoreMutationDagRetirement {
    pub dag: std::mem::ManuallyDrop<Option<semio_framework_os_spr::MutationDag>>,
    pub envelope: std::mem::ManuallyDrop<Option<ArtifactStoreConflictRetirement>>,
    pub identity: std::mem::ManuallyDrop<Option<ArtifactStoreStringRetirement>>,
}

impl ArtifactStoreMutationDagRetirement {
    pub fn new(dag: semio_framework_os_spr::MutationDag) -> Self {
        Self { dag: std::mem::ManuallyDrop::new(Some(dag)), envelope: std::mem::ManuallyDrop::new(None), identity: std::mem::ManuallyDrop::new(None) }
    }
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.envelope.is_some() { return retirement_owner_demands(&self.envelope, copy); }
        if self.identity.is_some() { return retirement_owner_demands(&self.identity, copy); }
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.dag.as_ref().map_or(0, |dag| dag.next_backing_release_byte_demand()), depth: usize::from(self.dag.is_some()), ..Default::default() })
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreMutationDagRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.demands(grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "mutation DAG exceeds admitted original owner depth")); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.envelope.is_some() { return retirement_owner_close(&mut self.envelope, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.identity.is_some() { return retirement_owner_close(&mut self.identity, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let dag = self.dag.as_mut().unwrap();
        if let Some(owner) = dag.take_one_close_owner() {
            match owner {
                semio_framework_os_spr::MutationDagCloseOwner::Envelope(envelope) => *self.envelope = Some(ArtifactStoreConflictRetirement::causal_envelope(envelope)),
                semio_framework_os_spr::MutationDagCloseOwner::Identity(id) => *self.identity = Some(ArtifactStoreStringRetirement::new(id)),
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if !dag.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mutation DAG lost an exact owner before terminal retirement")); }
        if !dag.backing_is_empty() {
            let (copied_items, released_bytes, _) = dag.close_backing_step(1, grant.maximum_release_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items, released_bytes, ..empty }));
        }
        drop(self.dag.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.dag.is_none() && self.envelope.is_none() && self.identity.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl Drop for ArtifactStoreMutationDagRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "mutation DAG retirement reached Drop before every exact nested owner was terminal-empty");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.dag); std::mem::ManuallyDrop::drop(&mut self.envelope); std::mem::ManuallyDrop::drop(&mut self.identity); } }
    }
}

pub enum ArtifactStoreVcsRetirementPhase {
    Mutations,
    Edits,
    Changes,
    Checkpoints,
    Alternatives,
    InitialSnapshot,
    Complete,
}

/// 📏️ Observes every current child demand and the separate physical cursor-shell release.

/// 🪆️ Quotes only the depth axis of the boxed turn, so a depth admission never re-derives the other three currencies.

/// 📦️ Retires a current nested payload and its original box in separate fully granted turns.

/// 🎫️ Prices a current original typed owner before its controlled child frame is born.

/// 📋️ Admits the complete original derived snapshot and registry under unchanged caller authority.
pub fn admit_original_snapshot_alias<P,Mu>(original:&mut Option<ArtifactDerivedSnapshot<P>>,snapshots:&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
where P:Send+Sync+'static,Mu:Mutation<P>+Send+'static {
    history_read_admission::admit_history_read_retirement(original,snapshots,mutations,grant,|owner,original|*owner.derived=Some(original))
}

pub struct ArtifactStoreHistoryLedgerRetirement<T> {
    pub ledger: std::mem::ManuallyDrop<Option<ArtifactHistoryLedger<T>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub pending: std::mem::ManuallyDrop<Option<T>>,
    pub visibility: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<semio_framework_os_vcs::ArtifactGroupVisibility>>>,
    pub factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    pub closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
}

impl<T: Send + 'static> ArtifactStoreHistoryLedgerRetirement<T> {
    pub fn new(ledger: ArtifactHistoryLedger<T>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>) -> Self {
        Self { ledger: std::mem::ManuallyDrop::new(Some(ledger)), active: std::mem::ManuallyDrop::new(None), pending: std::mem::ManuallyDrop::new(None), visibility: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory)), closing_factory: std::mem::ManuallyDrop::new(None) }
    }
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(closing) = self.closing_factory.as_ref() { return closing.demands(copy); }
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, copy); }
        if self.visibility.is_some() { return retirement_owner_demands(&self.visibility, copy); }
        if let Some(ledger) = self.ledger.as_ref() {
                        if let Some(key)=ledger.next_retained_read_key(){let overhead=size_of::<semio_framework_value::FactoryAuthority>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>();let mut demand=ledger.retained_read_return_demands(key,copy.saturating_sub(overhead))?;demand.copy_bytes=demand.copy_bytes.checked_add(overhead).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"history read return metadata extent overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history read return owner depth overflow"))?;return Ok(demand);}

if let Some(value) = self.pending.as_ref().or_else(|| ledger.last()) { return Ok(semio_framework_value::RetirementDemand { capacity_bytes: self.factory.as_ref().unwrap().retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
            if !ledger.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger retains an unresolved original group or reservation")); }
            if ledger.retained_terminal_slot_count() != 0 { ledger.next_terminal_slot_visibility()?; return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            return Ok(semio_framework_value::RetirementDemand { release_bytes: ledger.next_empty_page_release_byte_demand()?, depth: 1, ..Default::default() });
        }
        if self.factory.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        
        Ok(Default::default())
    }
}

impl<T: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreHistoryLedgerRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_progress};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history ledger exceeds admitted original owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if let Some(closing) = self.closing_factory.as_mut() {
            let step = closing.step(grant)?;
            if closing.terminal_is_empty() { self.closing_factory.take(); }
            let progress = step.progress();
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) });
        }
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.visibility.is_some() { return retirement_owner_close(&mut self.visibility, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(ledger) = self.ledger.as_mut() {
                        if let Some(key)=ledger.next_retained_read_key(){let overhead=size_of::<semio_framework_value::FactoryAuthority>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,maximum_depth:grant.maximum_depth-1,..grant};let mut returned=None;let step=match ledger.return_retained_read(key,&mut returned,child){Ok(step)=>step,Err(error)if error.kind==semio_framework_value::ValueRefusalKind::WorkLimit=>return Ok(RetainedCloneStep::Progress(empty)),Err(error)=>return Err(error)};let mut progress=step.progress();if progress==empty{return Ok(RetainedCloneStep::Progress(empty));}if let Some(factory)=returned{let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;*self.closing_factory=Some(semio_framework_value::FactoryAuthority::new(factory));}progress.copied_items=1;progress.copied_bytes=progress.copied_bytes.checked_add(overhead).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history original read return receipt overflow"))?;admit_retained_clone_progress(grant,progress,"history original read source return")?;return Ok(RetainedCloneStep::Progress(progress));}

if let Some(value) = self.pending.take().or_else(|| ledger.pop()) {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                return match self.factory.as_ref().unwrap().retire_owned(value, child) {
                    Ok((owner, progress)) => { *self.active = Some(owner); admit_retained_clone_progress(child, progress, "original history entry constructor")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history entry factory changed its exact declared birth")); } Ok(RetainedCloneStep::Progress(progress)) }
                    Err((error, value)) => { *self.pending = Some(value); Err(error) }
                };
            }
            if ledger.retained_terminal_slot_count() != 0 {
                let (visibility, progress) = ledger.retire_terminal_slot(grant)?;
                *self.visibility = visibility.map(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease);
                return Ok(RetainedCloneStep::Progress(progress));
            }
            if !ledger.backing_is_empty() { return ledger.release_empty_page(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
            drop(self.ledger.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.factory.is_some() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.factory.take().unwrap();
            *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn terminal_is_empty(&self) -> bool { self.ledger.is_none() && self.active.is_none() && self.pending.is_none() && self.visibility.is_none() && self.factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<T> Drop for ArtifactStoreHistoryLedgerRetirement<T> {
    fn drop(&mut self) {
        let empty = self.ledger.is_none() && self.active.is_none() && self.pending.is_none() && self.visibility.is_none() && self.factory.is_none() && self.closing_factory.is_none();
        assert!(std::thread::panicking() || empty, "original history ledger retains its payloads, visibility, native backing and factory until exact terminal");
        if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.ledger); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.pending); std::mem::ManuallyDrop::drop(&mut self.visibility); std::mem::ManuallyDrop::drop(&mut self.factory); std::mem::ManuallyDrop::drop(&mut self.closing_factory); } }
    }
}

pub struct ArtifactStoreVcsRetirement<P, Mutation> {
    pub vcs: std::mem::ManuallyDrop<Option<ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub phase: ArtifactStoreVcsRetirementPhase,
    pub initial_snapshot_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub closing_factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactStoreCheckpointRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<Checkpoint> for ArtifactStoreCheckpointRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &Checkpoint) -> usize { size_of::<ArtifactStoreHistoryMetadataRetirement>() }
    fn retire_owned(&self, value: Checkpoint, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Checkpoint)> {
        admit_artifact_retirement(value, grant, ArtifactStoreHistoryMetadataRetirement::checkpoint)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactStoreAlternativeRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<Alternative> for ArtifactStoreAlternativeRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &Alternative) -> usize { size_of::<ArtifactStoreHistoryMetadataRetirement>() }
    fn retire_owned(&self, value: Alternative, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Alternative)> {
        admit_artifact_retirement(value, grant, ArtifactStoreHistoryMetadataRetirement::alternative)
    }
}

pub fn artifact_history_ledger_birth<T: Send + 'static, F: semio_framework_value::FactoryPayloadRetirement>() -> Result<usize, ValueError> {
    size_of::<ArtifactStoreHistoryLedgerRetirement<T>>().checked_add(semio_framework_value::factory_arc_birth_bytes::<F>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history owner constructor extent overflow"))
}

pub fn artifact_history_ledger_handoff<T: Send + 'static, F: ArtifactOwnedValueRetirementFactory<T> + semio_framework_value::FactoryPayloadRetirement>(ledger: &mut ArtifactHistoryLedger<T>, factory: F, slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
    let capacity = artifact_history_ledger_birth::<T, F>()?;
    if grant.maximum_items == 0 || grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if grant.maximum_depth < 2 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history owner constructor requires original child depth")); }
    let original = std::mem::replace(ledger, ArtifactHistoryLedger::empty());
    *slot = Some(Box::new(ArtifactStoreHistoryLedgerRetirement::new(original, Arc::new(factory))));
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }))
}

impl<P, Mutation> ArtifactStoreVcsRetirement<P, Mutation> {
    pub fn new(vcs: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { vcs: std::mem::ManuallyDrop::new(Some(vcs)), active: std::mem::ManuallyDrop::new(None), phase: ArtifactStoreVcsRetirementPhase::Mutations, initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(initial_snapshot_factory)), mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)), closing_factories: std::mem::ManuallyDrop::new([None, None]) }
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactStoreVcsRetirement<P, Mutation> {
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, copy); }
        let Some(vcs) = self.vcs.as_ref() else {
            if self.initial_snapshot_factory.is_some() || self.mutation_factory.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            if let Some(factory) = self.closing_factories.iter().flatten().next() { let mut demand = factory.demands(copy)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "VCS factory depth overflow"))?; return Ok(demand); }
            return Ok(Default::default());
        };
        let capacity_bytes = match self.phase {
            ArtifactStoreVcsRetirementPhase::Edits if !vcs.edits.backing_is_empty() || !vcs.edits.terminal_is_empty() => artifact_history_ledger_birth::<Edit<Mutation>, ArtifactStoreDecodedEditRetirementFactory<Mutation>>()?,
            ArtifactStoreVcsRetirementPhase::Changes if !vcs.changes.backing_is_empty() || !vcs.changes.terminal_is_empty() => artifact_history_ledger_birth::<Change, ArtifactStoreChangeRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::Checkpoints if !vcs.checkpoints.backing_is_empty() || !vcs.checkpoints.terminal_is_empty() => artifact_history_ledger_birth::<Checkpoint, ArtifactStoreCheckpointRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::Alternatives if !vcs.alternatives.backing_is_empty() || !vcs.alternatives.terminal_is_empty() => artifact_history_ledger_birth::<Alternative, ArtifactStoreAlternativeRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::InitialSnapshot => size_of::<ArtifactGenesisRetirement<P>>(),
            _ => 0,
        };
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: if capacity_bytes == 0 { 1 } else { 2 }, ..Default::default() })
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreVcsRetirement<P, Mutation> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "VCS retirement exceeds admitted original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.vcs.is_none() {
            if self.initial_snapshot_factory.is_some() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.initial_snapshot_factory.take().unwrap(); self.closing_factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
            if self.mutation_factory.is_some() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.mutation_factory.take().unwrap(); self.closing_factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
            let slot = self.closing_factories.iter_mut().find(|slot| slot.is_some()).unwrap();
            let factory = slot.as_mut().unwrap();
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = factory.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original VCS factory capability")?;
            if factory.terminal_is_empty() { drop(slot.take()); }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        let vcs = self.vcs.as_mut().unwrap();
        match self.phase {
            ArtifactStoreVcsRetirementPhase::Mutations => self.phase = ArtifactStoreVcsRetirementPhase::Edits,
            ArtifactStoreVcsRetirementPhase::Edits => {
                self.phase = ArtifactStoreVcsRetirementPhase::Changes;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.edits, ArtifactStoreDecodedEditRetirementFactory { mutation_factory: self.mutation_factory.as_ref().unwrap().clone() }, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Changes => {
                self.phase = ArtifactStoreVcsRetirementPhase::Checkpoints;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.changes, ArtifactStoreChangeRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Checkpoints => {
                self.phase = ArtifactStoreVcsRetirementPhase::Alternatives;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.checkpoints, ArtifactStoreCheckpointRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Alternatives => {
                self.phase = ArtifactStoreVcsRetirementPhase::InitialSnapshot;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.alternatives, ArtifactStoreAlternativeRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::InitialSnapshot => {
                if !(vcs.edits.terminal_is_empty() && vcs.edits.backing_is_empty() && vcs.changes.terminal_is_empty() && vcs.changes.backing_is_empty() && vcs.checkpoints.terminal_is_empty() && vcs.checkpoints.backing_is_empty() && vcs.alternatives.terminal_is_empty() && vcs.alternatives.backing_is_empty()) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS genesis handoff still retains original history backing")); }
                let ArtifactVcs { genesis, edits, changes, checkpoints, alternatives } = self.vcs.take().unwrap();
                drop((edits, changes, checkpoints, alternatives));
                *self.active = Some(Box::new(ArtifactGenesisRetirement::new(genesis, self.initial_snapshot_factory.as_ref().unwrap().clone())));
                self.phase = ArtifactStoreVcsRetirementPhase::Complete;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..empty }));
            }
            ArtifactStoreVcsRetirementPhase::Complete => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed VCS phase still retains its original VCS")),
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.vcs.is_none() && self.active.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.closing_factories.iter().all(Option::is_none) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactStoreVcsRetirement<P, Mutation> {
    fn drop(&mut self) {
        let empty = self.vcs.is_none() && self.active.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.closing_factories.iter().all(Option::is_none);
        assert!(std::thread::panicking() || empty, "VCS retirement reached Drop before every exact history backing and factory became terminal-empty");
        if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.vcs); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.initial_snapshot_factory); std::mem::ManuallyDrop::drop(&mut self.mutation_factory); std::mem::ManuallyDrop::drop(&mut self.closing_factories); } }
    }
}

/// 📏️ Quotes the exact cursor frame [`retire_document_envelope`] births before any original field or factory transfers.
pub fn retire_document_envelope_birth_bytes<P: Send + Sync + 'static, Mutation: Send + 'static>() -> usize {
    size_of::<ArtifactStoreEnvelopeRetirement<P, Mutation>>()
}

/// ♻️ Admits whole detached document custody before any original field or factory transfers.
pub fn retire_document_envelope<P: Send + Sync + 'static, Mutation: Send + 'static>(
    envelope: ArtifactEnvelope<P, Mutation>,
    initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    grant: RetainedCloneGrant,
) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, ArtifactEnvelope<P, Mutation>, Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>)> {
    admit_artifact_retirement((envelope, initial_snapshot_factory, mutation_factory), grant, |(envelope, snapshots, mutations)| ArtifactStoreEnvelopeRetirement::new(envelope, snapshots, mutations))
        .map_err(|(error, (envelope, snapshots, mutations))| (error, envelope, snapshots, mutations))
}
